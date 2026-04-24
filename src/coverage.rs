use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct CoverageResult {
    pub changed_lines: Vec<usize>,
    pub uncovered_lines: Vec<usize>,
    pub suggestions: Vec<String>,
}

pub fn detect_changed_lines(old: &str, new: &str) -> Vec<usize> {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();
    let mut changed = Vec::new();
    for (i, (o, n)) in old_lines.iter().zip(new_lines.iter()).enumerate() {
        if o != n { changed.push(i + 1); }
    }
    if new_lines.len() > old_lines.len() {
        for i in old_lines.len()..new_lines.len() {
            changed.push(i + 1);
        }
    }
    changed
}

pub fn run_coverage(_project_root: &Path) -> std::collections::HashMap<String, Vec<usize>> {
    std::collections::HashMap::new()
}

pub fn coverage_stage(_file_path: &Path, old: &str, new: &str) -> CoverageResult {
    let changed = detect_changed_lines(old, new);
    let coverage = run_coverage(Path::new("."));
    let mut uncovered = Vec::new();
    for line in &changed {
        if !coverage.values().any(|lines| lines.contains(line)) {
            uncovered.push(*line);
        }
    }
    let mut suggestions = Vec::new();
    for line in &uncovered {
        let func_name = new.lines().nth(line.saturating_sub(1))
            .and_then(|l| l.split("fn ").nth(1))
            .and_then(|s| s.split('(').next())
            .unwrap_or("function_name");
        suggestions.push(format!(
            "#[test]\nfn test_{}() {{\n    // TODO: test the change at line {}\n    assert!(true);\n}}\n",
            func_name, line
        ));
    }
    CoverageResult { changed_lines: changed, uncovered_lines: uncovered, suggestions }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_changed_lines() {
        let old = "line1\nline2\nline3";
        let new = "line1\nline2_changed\nline3";
        let changed = detect_changed_lines(old, new);
        assert_eq!(changed, vec![2]);
    }

    #[test]
    fn test_detect_added_lines() {
        let old = "line1";
        let new = "line1\nline2";
        let changed = detect_changed_lines(old, new);
        assert_eq!(changed, vec![2]);
    }
}
