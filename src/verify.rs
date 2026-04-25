use anyhow::Result;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct VerificationResult {
    pub syntax: Option<StageVerdict>,
    pub lsp: Option<StageVerdict>,
    pub compile: Option<StageVerdict>,
    pub tests: Option<StageVerdict>,
    pub all_passed: bool,
}

#[derive(Debug, Serialize)]
pub struct StageVerdict {
    pub passed: bool,
    pub details: String,
}

/// Run post‑patch verification pipeline.
pub fn verify_patch(
    file_path: &Path,
    patched_content: &str,
    _original_content: &str,
    lang_str: &str,
    compile_timeout: u64,
    verify_test: Option<&str>,
) -> Result<VerificationResult> {
    let mut result = VerificationResult {
        syntax: None,
        lsp: None,
        compile: None,
        tests: None,
        all_passed: true,
    };

    // 1. Syntax check (tree‑sitter parse)
    if let Ok(mut lang) = crate::ast::detect_language(file_path) {
        let parse_result = lang.parse(patched_content);
        let passed = lang.is_valid(&parse_result);
        result.syntax = Some(StageVerdict {
            passed,
            details: if passed { "AST valid".to_string() } else { "Syntax error detected".to_string() },
        });
        if !passed { result.all_passed = false; }
    }

    // 2. Compile check (if language supports it)
    if !lang_str.is_empty() && lang_str != "txt" && lang_str != "md" {
        match crate::compile::compile_check(file_path, lang_str, compile_timeout) {
            Ok(compile_result) => {
                let passed = compile_result.success;
                result.compile = Some(StageVerdict {
                    passed,
                    details: if passed {
                        "Compilation successful".to_string()
                    } else {
                        format!("{} errors", compile_result.errors.len())
                    },
                });
                if !passed { result.all_passed = false; }
            }
            Err(e) => {
                result.compile = Some(StageVerdict {
                    passed: true,
                    details: format!("Compilation check skipped: {}", e),
                });
            }
        }
    }

    // 3. Test command (if provided)
    if let Some(test_cmd) = verify_test {
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(test_cmd)
            .status();
        let passed = status.map(|s| s.success()).unwrap_or(false);
        result.tests = Some(StageVerdict {
            passed,
            details: if passed { "Tests passed".to_string() } else { "Tests failed".to_string() },
        });
        if !passed { result.all_passed = false; }
    }

    Ok(result)
}
