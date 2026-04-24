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

/// Run a single gate stage: syntax validation.
fn gate_syntax(_file_path: &Path, content: &str) -> Result<StageResult> {
    use crate::ast::{Language, RustLanguage};
    let mut lang = RustLanguage::new(); // Simplification: use Rust for now; in practice, detect language
    let parse_result = lang.parse(content);
    let valid = lang.is_valid(&parse_result);
    Ok(StageResult {
        name: "syntax".to_string(),
        passed: valid,
        details: if valid { "AST valid".to_string() } else { "Syntax error detected".to_string() },
    })
}

/// Run a single gate stage: compilation check.
fn gate_compile(file_path: &Path, timeout_secs: u64) -> Result<StageResult> {
    let lang_str = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let compile_result = crate::compile::compile_check(file_path, lang_str, timeout_secs)?;
    Ok(StageResult {
        name: "compile".to_string(),
        passed: compile_result.success,
        details: if compile_result.success { "Compilation successful".to_string() } else { format!("{} errors", compile_result.errors.len()) },
    })
}

/// Run a single gate stage: cross‑file impact analysis.
fn gate_cross_file(_file_path: &Path, old_content: &str, new_content: &str) -> Result<StageResult> {
    let project_index = crate::crossfile::build_project_index(Path::new("."));
    let mut warnings = Vec::new();
    // Simple heuristic: detect function name changes
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
        details: if passed { "No callers affected".to_string() } else { format!("{} callers affected: {:?}", warnings.len(), warnings) },
    })
}

/// Run a single gate stage: test suite.
fn gate_test() -> Result<StageResult> {
    let status = std::process::Command::new("just").arg("test").status()?;
    let passed = status.success();
    Ok(StageResult {
        name: "test".to_string(),
        passed,
        details: if passed { "Tests passed".to_string() } else { "Tests failed".to_string() },
    })
}

/// Execute a sequence of gate stages.
pub fn run_gate(stages: &[String], file_path: &Path, old_content: &str, new_content: &str, compile_timeout: u64) -> Result<GateResult> {
    let mut results = Vec::new();
    let mut all_passed = true;
    for stage in stages {
        let result = match stage.as_str() {
            "syntax" => gate_syntax(file_path, new_content)?,
            "compile" => gate_compile(file_path, compile_timeout)?,
            "cross‑file" | "cross_file" => gate_cross_file(file_path, old_content, new_content)?,
            "test" => gate_test()?,
            _ => StageResult { name: stage.clone(), passed: true, details: "Unknown stage – skipped".to_string() },
        };
        if !result.passed { all_passed = false; }
        results.push(result);
        if !all_passed { break; } // Stop on first failure
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
}
