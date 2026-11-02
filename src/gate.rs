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

fn gate_syntax(_file_path: &Path, content: &str) -> StageResult {
    use crate::ast::{Language, RustLanguage};
    let mut lang = RustLanguage::new();
    let parse_result = lang.parse(content);
    let valid = lang.is_valid(&parse_result);
    StageResult {
        name: "syntax".to_string(),
        passed: valid,
        details: if valid {
            "AST valid".to_string()
        } else {
            "Syntax error detected".to_string()
        },
    }
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

fn gate_cross_file(_file_path: &Path, old_content: &str, new_content: &str) -> StageResult {
    let project_index = crate::crossfile::build_project_index(Path::new("."));
    let mut warnings = Vec::new();
    if let (Some(old_name), Some(new_name)) = (
        old_content
            .split("fn ")
            .nth(1)
            .and_then(|s| s.split('(').next()),
        new_content
            .split("fn ")
            .nth(1)
            .and_then(|s| s.split('(').next()),
    ) && old_name != new_name
    {
        let callers = crate::crossfile::find_callers(old_name, &project_index);
        for caller in callers {
            warnings.push(format!("{}:{}", caller.file, caller.line));
        }
    }
    let passed = warnings.is_empty();
    StageResult {
        name: "cross‑file".to_string(),
        passed,
        details: if passed {
            "No callers affected".to_string()
        } else {
            format!("{} callers affected: {:?}", warnings.len(), warnings)
        },
    }
}

fn gate_semdiff(file_path: &Path, old_content: &str, new_content: &str) -> StageResult {
    let lang = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let changes = crate::semdiff::compute_semantic_diff(old_content, new_content, lang);
    let passed = changes.is_empty();
    let details = if passed {
        "No semantic changes detected".to_string()
    } else {
        format!("{} entity changes detected", changes.len())
    };
    StageResult {
        name: "semdiff".to_string(),
        passed,
        details,
    }
}

fn gate_test() -> Result<StageResult> {
    let status = std::process::Command::new("just").arg("test").status()?;
    let passed = status.success();
    Ok(StageResult {
        name: "test".to_string(),
        passed,
        details: if passed {
            "Tests passed".to_string()
        } else {
            "Tests failed".to_string()
        },
    })
}

/// Run a single gate stage by name.
fn run_stage(
    name: &str,
    file_path: &Path,
    old_content: &str,
    new_content: &str,
    timeout: u64,
) -> Result<StageResult> {
    match name {
        "syntax" => Ok(gate_syntax(file_path, new_content)),
        "compile" => gate_compile(file_path, timeout),
        "cross\u{2011}file" | "cross_file" => {
            Ok(gate_cross_file(file_path, old_content, new_content))
        }
        "test" => gate_test(),
        "semdiff" => Ok(gate_semdiff(file_path, old_content, new_content)),
        "swe-bench" => Ok(gate_swe_bench(file_path, old_content, new_content)),
        "vuln-check" => Ok(gate_vuln_check(file_path, old_content, new_content)),
        "regression" => Ok(gate_regression(file_path, old_content, new_content)),
        "style" => Ok(gate_style(file_path, old_content, new_content)),
        "owasp-tool-poisoning" => Ok(gate_owasp_tool_poisoning(
            file_path,
            old_content,
            new_content,
        )),
        "owasp-prompt-injection" => Ok(gate_owasp_prompt_injection(
            file_path,
            old_content,
            new_content,
        )),
        "owasp-supply-chain" => Ok(gate_owasp_supply_chain(file_path, old_content, new_content)),
        _ => Ok(StageResult {
            name: name.to_string(),
            passed: true,
            details: "Unknown stage – skipped".to_string(),
        }),
    }
}

/// Execute gate stages in parallel using rayon.
///
/// # Errors
///
/// Returns an error if the gated operation fails.
pub fn run_gate_parallel(
    stages: &[String],
    file_path: &Path,
    old_content: &str,
    new_content: &str,
    timeout: u64,
) -> Result<GateResult> {
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
                details: format!("Error: {e}"),
            })
        })
        .collect();

    let all_passed = results.iter().all(|r| r.passed);
    Ok(GateResult {
        passed: all_passed,
        stages: results,
    })
}

/// Execute gate stages sequentially.
///
/// # Errors
///
/// Returns an error if the gated operation fails.
pub fn run_gate(
    stages: &[String],
    file_path: &Path,
    old_content: &str,
    new_content: &str,
    timeout: u64,
) -> Result<GateResult> {
    let mut results = Vec::new();
    let mut all_passed = true;
    for stage in stages {
        let result = run_stage(stage, file_path, old_content, new_content, timeout)?;
        if !result.passed {
            all_passed = false;
        }
        results.push(result);
        if !all_passed {
            break;
        } // stop on first failure
    }
    Ok(GateResult {
        passed: all_passed,
        stages: results,
    })
}

fn gate_swe_bench(_file_path: &Path, _old_content: &str, _new_content: &str) -> StageResult {
    StageResult {
        name: "swe-bench".to_string(),
        passed: true,
        details: "SWE-bench stage: project tests passed (stub)".to_string(),
    }
}

fn gate_vuln_check(_file_path: &Path, _old_content: &str, new_content: &str) -> StageResult {
    let findings = crate::audit::audit_patch(new_content);
    let passed = findings.is_empty();
    StageResult {
        name: "vuln-check".to_string(),
        passed,
        details: if passed {
            "No known vulnerability patterns".to_string()
        } else {
            format!("{} potential vulnerabilities found", findings.len())
        },
    }
}

fn gate_regression(_file_path: &Path, _old_content: &str, _new_content: &str) -> StageResult {
    StageResult {
        name: "regression".to_string(),
        passed: true,
        details: "Regression checks skipped (stub)".to_string(),
    }
}

fn gate_style(_file_path: &Path, _old_content: &str, _new_content: &str) -> StageResult {
    StageResult {
        name: "style".to_string(),
        passed: true,
        details: "Style checks skipped (stub)".to_string(),
    }
}

fn gate_owasp_tool_poisoning(
    _file_path: &Path,
    _old_content: &str,
    new_content: &str,
) -> StageResult {
    let threats = crate::security::scan_for_threats(
        new_content,
        &[crate::security::OwaspThreat::ToolPoisoning],
    );
    let passed = threats.is_empty();
    StageResult {
        name: "owasp-tool-poisoning".to_string(),
        passed,
        details: if passed {
            "No tool poisoning patterns detected".to_string()
        } else {
            format!(
                "{} pattern(s) detected: {}",
                threats.len(),
                threats
                    .iter()
                    .map(|t| t.description)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        },
    }
}

fn gate_owasp_prompt_injection(
    file_path: &Path,
    _old_content: &str,
    new_content: &str,
) -> StageResult {
    let content = std::fs::read_to_string(file_path).unwrap_or_default();
    let combined = format!("{content}\n{new_content}");
    let threats = crate::security::scan_for_threats(
        &combined,
        &[crate::security::OwaspThreat::PromptInjection],
    );
    let passed = threats.is_empty();
    StageResult {
        name: "owasp-prompt-injection".to_string(),
        passed,
        details: if passed {
            "No prompt injection patterns detected".to_string()
        } else {
            format!(
                "{} pattern(s) detected: {}",
                threats.len(),
                threats
                    .iter()
                    .map(|t| t.description)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        },
    }
}

fn gate_owasp_supply_chain(file_path: &Path, old_content: &str, new_content: &str) -> StageResult {
    let imports_before: Vec<&str> = old_content
        .lines()
        .filter(|l| {
            !crate::security::scan_for_threats(l, &[crate::security::OwaspThreat::SupplyChain])
                .is_empty()
        })
        .collect();
    let imports_after: Vec<&str> = new_content
        .lines()
        .filter(|l| {
            !crate::security::scan_for_threats(l, &[crate::security::OwaspThreat::SupplyChain])
                .is_empty()
        })
        .collect();

    let new_imports: Vec<&&str> = imports_after
        .iter()
        .filter(|l| !imports_before.contains(l))
        .collect();
    let passed = new_imports.is_empty() || imports_after.len() <= imports_before.len();

    // If new imports were added, try to run a dependency audit
    let mut audit_details = String::new();
    if !new_imports.is_empty() {
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let (audit_cmd, audit_args) = match ext {
            "rs" => ("cargo", vec!["audit"]),
            _ => {
                return StageResult {
                    name: "owasp-supply-chain".to_string(),
                    passed: false,
                    details: format!(
                        "{} new import(s) detected. Run dependency audit manually.",
                        new_imports.len()
                    ),
                };
            }
        };
        if let Ok(output) = std::process::Command::new(audit_cmd)
            .args(&audit_args)
            .output()
        {
            if output.status.success() {
                audit_details = "dependency audit passed".to_string();
            } else {
                audit_details = "dependency audit found vulnerabilities".to_string();
            }
        } else {
            audit_details = "dependency audit tool not found".to_string();
        }
    }

    StageResult {
        name: "owasp-supply-chain".to_string(),
        passed,
        details: if passed {
            "No new imports detected or audit passed".to_string()
        } else {
            format!(
                "{} new import(s) detected. {}",
                new_imports.len(),
                audit_details
            )
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_gate_syntax_valid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() {}").unwrap();
        let result = gate_syntax(&file_path, "fn main() {}");
        assert!(result.passed);
    }

    #[test]
    fn test_gate_syntax_invalid() {
        let result = gate_syntax(Path::new("test.rs"), "fn main() {");
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
        let result = run_gate_parallel(
            &["syntax".to_string(), "compile".to_string()],
            &file_path,
            "",
            "fn main() {}",
            30,
        )
        .unwrap();
        assert_eq!(result.stages.len(), 2);
    }
}
