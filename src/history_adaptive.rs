use anyhow::Result;
use std::path::Path;

/// Suggest an optimal confidence threshold based on history.
pub fn suggest_threshold(history_dir: &Path) -> Result<f64> {
    let history_file = history_dir.join("history.jsonl");
    let content = std::fs::read_to_string(&history_file).unwrap_or_default();
    let mut total = 0usize;
    let mut failures = 0usize;
    for line in content.lines() {
        if let Ok(record) = serde_json::from_str::<serde_json::Value>(line) {
            total += 1;
            if record.get("success").and_then(|v| v.as_bool()) == Some(false) {
                failures += 1;
            }
        }
    }
    if total == 0 { return Ok(0.9); }
    let success_rate = (total - failures) as f64 / total as f64;
    // Simple heuristic: set threshold just above the current success rate
    let suggested = (success_rate + 0.05).min(0.99);
    Ok(suggested)
}
