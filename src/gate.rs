use anyhow::Result;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct StageResult {
    pub name: String,
    pub passed: bool,
    pub details: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GateResult {
    pub passed: bool,
    pub stages: Vec<StageResult>,
}

// -- Individual stage implementations --

fn gate_syntax(_file_path: &Path, content: &str) -> Result<StageResult> {
    use crate::ast::{Language, RustLanguage};
    let mut lang = RustLanguage::new();
    let parse_result = lang.parse(content);
    let valid = lang.is_valid(&parse_result);
    Ok(StageResult {
        name: "syntax".to_string(),
        passed: valid,
        details: if valid { "AST valid".to_string() } else { "Syntax error detected".to_string() },
    })
}

fn gate_compile(file_path: &Path, timeout_secs: u64) -> Result<StageResult> {
    let lang_str = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let compile_result = crate::compile::compile_check(file_path, lang_str, timeout_secs)?;
    Ok(StageResult {
        name: "compile".to_string(),
        passed: compile_result.success,
        details: if compile_result.success {
            "Compilation successful".to_string()
        } else {
            format!("{} errors", compile_result.errors.len())
        },
    })
}

fn gate_cross_file(_file_path: &Path, old_content: &str, new_content: &str) -> Result<StageResult> {
    let project_index = crate::crossfile::build_project_index(Path::new("."));
    let mut warnings = Vec::new();
    if let (Some(old_name), Some(new_name)) = (
        old_content.split("fn ").nth(1).and_then(|s| s.split('(').next()),
        new_content.split("fn ").nth(1).and_then(|s| s.split('(').next()),
    ) {
        if old_name != new_name {
            let callers = crate::crossfile::find_callers(old_name, &project_index);
            for caller in callers {
                warnings.push(format!("{}:{}", caller.file, caller.line));
            }
        }
    }
    let passed = warnings.is_empty();
    Ok(StageResult {
        name: "cross‑file".to_string(),
        passed,
        details: if passed {
            "No callers affected".to_string()
        } else {
            format!("{} callers affected: {:?}", warnings.len(), warnings)
        },
    })
}

fn gate_test() -> Result<StageResult> {
    let status = std::process::Command::new("just").arg("test").status()?;
    let passed = status.success();
    Ok(StageResult {
        name: "test".to_string(),
        passed,
        details: if passed { "Tests passed".to_string() } else { "Tests failed".to_string() },
    })
}

/// Run a single gate stage by name.
fn run_stage(name: &str, file_path: &Path, old_content: &str, new_content: &str, timeout: u64) -> Result<StageResult> {
    match name {
        "syntax" => gate_syntax(file_path, new_content),
        "compile" => gate_compile(file_path, timeout),
        "cross‑file" | "cross_file" => gate_cross_file(file_path, old_content, new_content),
        "test" => gate_test(),
        _ => Ok(StageResult { name: name.to_string(), passed: true, details: "Unknown stage – skipped".to_string() }),
    }
}

/// Execute gate stages in parallel using rayon.
pub fn run_gate_parallel(stages: &[String], file_path: &Path, old_content: &str, new_content: &str, timeout: u64) -> Result<GateResult> {
    let results: Vec<StageResult> = stages
        .iter()
        .map(|stage| {
            let name = stage.clone();
            let fp = file_path.to_path_buf();
            let old = old_content.to_string();
            let new = new_content.to_string();
            // Each stage runs in its own scope
            run_stage(&name, &fp, &old, &new, timeout).unwrap_or_else(|e| StageResult {
                name: name.clone(),
                passed: false,
                details: format!("Error: {}", e),
            })
        })
        .collect();

    let all_passed = results.iter().all(|r| r.passed);
    Ok(GateResult { passed: all_passed, stages: results })
}

/// Execute gate stages sequentially.
pub fn run_gate(stages: &[String], file_path: &Path, old_content: &str, new_content: &str, timeout: u64) -> Result<GateResult> {
    let mut results = Vec::new();
    let mut all_passed = true;
    for stage in stages {
        let result = run_stage(stage, file_path, old_content, new_content, timeout)?;
        if !result.passed { all_passed = false; }
        results.push(result);
        if !all_passed { break; } // stop on first failure
    }
    Ok(GateResult { passed: all_passed, stages: results })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_gate_syntax_valid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() {}").unwrap();
        let result = gate_syntax(&file_path, "fn main() {}").unwrap();
        assert!(result.passed);
    }

    #[test]
    fn test_gate_syntax_invalid() {
        let result = gate_syntax(Path::new("test.rs"), "fn main() {").unwrap();
        assert!(!result.passed);
    }

    #[test]
    fn test_run_gate_syntax_only() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() {}").unwrap();
        let result = run_gate(&["syntax".to_string()], &file_path, "", "fn main() {}", 30).unwrap();
        assert!(result.passed);
        assert_eq!(result.stages.len(), 1);
    }

    #[test]
    fn test_run_gate_parallel() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() {}").unwrap();
        let result = run_gate_parallel(&["syntax".to_string(), "compile".to_string()], &file_path, "", "fn main() {}", 30).unwrap();
        assert_eq!(result.stages.len(), 2);
    }
}
