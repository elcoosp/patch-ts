use anyhow::Result;
use std::path::Path;

///
/// # Errors
///
/// Returns an error if the operation fails.
pub fn suggest_threshold(history_dir: &Path) -> Result<f64> {
    let history_file = history_dir.join("history.jsonl");
    let content = std::fs::read_to_string(&history_file).unwrap_or_default();
    let mut total = 0usize;
    let mut failures = 0usize;
    for line in content.lines() {
        if let Ok(record) = serde_json::from_str::<serde_json::Value>(line) {
            total += 1;
            if record.get("success").and_then(serde_json::Value::as_bool) == Some(false) {
                failures += 1;
            }
        }
    }
    if total == 0 {
        return Ok(0.9);
    }
    let success_rate = crate::num::to_f64(total - failures) / crate::num::to_f64(total);
    // Simple heuristic: set threshold just above the current success rate to challenge the agent
    let suggested = (success_rate + 0.05).min(0.99);
    Ok(suggested)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn empty_history_suggests_default_threshold() {
        let dir = tempdir().unwrap();
        let threshold = suggest_threshold(dir.path()).unwrap();
        assert!((0.0..=1.0).contains(&threshold));
        assert_eq!(threshold, 0.9);
    }

    #[test]
    fn successful_history_raises_threshold() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("history.jsonl"),
            "{\"success\":true}\n{\"success\":true}\n{\"success\":true}\n",
        )
        .unwrap();
        let threshold = suggest_threshold(dir.path()).unwrap();
        assert!(
            (threshold - 0.99).abs() < 1e-9,
            "all-success history should cap at 0.99, got {threshold}"
        );
    }

    #[test]
    fn failing_history_lowers_threshold() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("history.jsonl"),
            "{\"success\":true}\n{\"success\":false}\n",
        )
        .unwrap();
        let threshold = suggest_threshold(dir.path()).unwrap();
        assert!(threshold > 0.5 && threshold < 0.99, "got {threshold}");
    }

    #[test]
    fn malformed_lines_are_ignored() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("history.jsonl"), "not json\n").unwrap();
        assert_eq!(suggest_threshold(dir.path()).unwrap(), 0.9);
    }
}
