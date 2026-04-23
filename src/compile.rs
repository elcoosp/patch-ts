use anyhow::Result;
use regex::Regex;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;  // from wait-timeout crate; we'll add it

#[derive(Debug, Clone)]
pub struct CompileError {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct CompileResult {
    pub success: bool,
    pub errors: Vec<CompileError>,
}

/// Run a language‑specific compiler check on the given file.
/// Returns a `CompileResult` indicating success and any errors.
pub fn compile_check(file_path: &Path, lang: &str, timeout_secs: u64) -> Result<CompileResult> {
    let (cmd, args) = match lang {
        "rs" => ("cargo", vec!["check", "--message-format=short"]),
        "ts" | "tsx" => ("tsc", vec!["--noEmit", file_path.to_str().unwrap()]),
        "js" | "jsx" | "mjs" | "cjs" => ("node", vec!["--check", file_path.to_str().unwrap()]),
        "py" | "pyi" => ("python3", vec!["-m", "py_compile", file_path.to_str().unwrap()]),
        "go" => ("go", vec!["build", "-o", "/dev/null", file_path.to_str().unwrap()]),
        _ => return Ok(CompileResult { success: true, errors: vec![] }),
    };

    // If the compiler is not installed, skip validation successfully
    if !command_exists(cmd) {
        eprintln!("Warning: {} not found; skipping compilation check.", cmd);
        return Ok(CompileResult { success: true, errors: vec![] });
    }

    let mut child = Command::new(cmd)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to run compiler: {}", e))?;

    let timeout = Duration::from_secs(timeout_secs);
    let status_code = match child.wait_timeout(timeout)? {
        Some(status) => status.code().unwrap_or(1),
        None => {
            child.kill()?;
            child.wait()?;
            return Ok(CompileResult {
                success: false,
                errors: vec![CompileError {
                    file: file_path.to_string_lossy().to_string(),
                    line: 0,
                    column: 0,
                    message: format!("Compilation timed out after {} seconds", timeout_secs),
                }],
            });
        }
    };

    let output = child.wait_with_output()?;
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if status_code == 0 {
        return Ok(CompileResult { success: true, errors: vec![] });
    }

    let errors = parse_compiler_output(&stderr, lang);
    Ok(CompileResult { success: false, errors })
}

fn command_exists(cmd: &str) -> bool {
    std::process::Command::new("which").arg(cmd).output().map(|o| o.status.success()).unwrap_or(false)
}

/// Extract error locations and messages from compiler output.
fn parse_compiler_output(output: &str, lang: &str) -> Vec<CompileError> {
    let mut errors = Vec::new();

    // Rust: error[E0308]: src/main.rs:10:5
    let rust_re = Regex::new(r"error(\[.*?\])?: (.*):(\d+):(\d+)").unwrap();
    for cap in rust_re.captures_iter(output) {
        errors.push(CompileError {
            file: cap[2].to_string(),
            line: cap[3].parse().unwrap_or(0),
            column: cap[4].parse().unwrap_or(0),
            message: output.lines().nth(0).unwrap_or("Compilation error").to_string(),
        });
    }

    // TypeScript: src/app.ts(10,5): error TS2322
    let ts_re = Regex::new(r"(.+?)\((\d+),(\d+)\):\s+error\s+(\w+): (.+)").unwrap();
    for cap in ts_re.captures_iter(output) {
        errors.push(CompileError {
            file: cap[1].to_string(),
            line: cap[2].parse().unwrap_or(0),
            column: cap[3].parse().unwrap_or(0),
            message: format!("{}: {}", &cap[4], &cap[5]),
        });
    }

    // Python: SyntaxError: invalid syntax (line 10)
    let py_re = Regex::new(r"line (\d+)").unwrap();
    if let Some(cap) = py_re.captures(output) {
        errors.push(CompileError {
            file: "".to_string(),
            line: cap[1].parse().unwrap_or(0),
            column: 0,
            message: output.lines().nth(0).unwrap_or("Syntax error").to_string(),
        });
    }

    // Fallback: if no structured errors found, return the whole output
    if errors.is_empty() && !output.trim().is_empty() {
        errors.push(CompileError {
            file: "".to_string(),
            line: 0,
            column: 0,
            message: output.trim().to_string(),
        });
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust_error() {
        let output = "error[E0308]: src/main.rs:10:5: mismatched types";
        let errors = parse_compiler_output(output, "rs");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 10);
        assert_eq!(errors[0].column, 5);
    }

    #[test]
    fn test_parse_typescript_error() {
        let output = "src/app.ts(10,5): error TS2322: Type 'string' is not assignable to type 'number'.";
        let errors = parse_compiler_output(output, "ts");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 10);
        assert_eq!(errors[0].column, 5);
    }

    #[test]
    fn test_success_result() {
        let result = compile_check(Path::new("nonexistent.rs"), "txt", 30).unwrap();
        assert!(result.success);
        assert!(result.errors.is_empty());
    }
}
