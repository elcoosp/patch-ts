use anyhow::Result;
use regex::Regex;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;
use wait_timeout::ChildExt;

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

/// Find the closest `Cargo.toml` walking up from the given file's directory.
fn find_cargo_manifest(file_path: &Path) -> Option<PathBuf> {
    let mut dir = file_path.parent()?.to_path_buf();
    loop {
        let candidate = dir.join("Cargo.toml");
        if candidate.is_file() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Build the compiler invocation for a given language.
/// For Rust we prefer a lock-free single-file `rustc` check and only fall back
/// to `cargo check` (with an explicit manifest path) when the file belongs to a
/// cargo project — running bare `cargo check` in the process' cwd can deadlock
/// on the target-directory lock.
/// `sink_dir` receives compiler artifacts (rustc metadata) for languages that
/// need a real output path.
fn compiler_command(
    file_path: &Path,
    lang: &str,
    sink_dir: &Path,
) -> Option<(String, Vec<String>)> {
    let path_str = file_path.to_str()?.to_string();
    match lang {
        "rs" => {
            if let Some(manifest) = find_cargo_manifest(file_path) {
                Some((
                    "cargo".into(),
                    vec![
                        "check".into(),
                        "--message-format=short".into(),
                        "--quiet".into(),
                        format!("--manifest-path={}", manifest.display()),
                    ],
                ))
            } else {
                let sink = sink_dir.join("check.rmeta");
                Some((
                    "rustc".into(),
                    vec![
                        "--edition=2021".into(),
                        "--crate-type=lib".into(),
                        "--emit=metadata".into(),
                        "--error-format=short".into(),
                        "--cap-lints=allow".into(),
                        "-o".into(),
                        sink.to_string_lossy().to_string(),
                        path_str,
                    ],
                ))
            }
        }
        "ts" | "tsx" => Some(("tsc".into(), vec!["--noEmit".into(), path_str])),
        "js" | "jsx" | "mjs" | "cjs" => Some(("node".into(), vec!["--check".into(), path_str])),
        "py" | "pyi" => Some((
            "python3".into(),
            vec!["-m".into(), "py_compile".into(), path_str],
        )),
        "go" => Some((
            "go".into(),
            vec!["build".into(), "-o".into(), "/dev/null".into(), path_str],
        )),
        _ => None,
    }
}

/// Resolve the directory a compiler should run in: the file's parent when it
/// has one, else the current directory (bare relative filenames yield an
/// empty parent, and `current_dir("")` fails with ENOENT).
fn working_dir_for(file_path: &Path) -> &Path {
    file_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

/// Drain a child's output stream on a helper thread so the pipe buffer can
/// never fill up and deadlock the compiler while we wait for it to exit.
fn drain_stream<R: Read + Send + 'static>(mut stream: R) -> mpsc::Receiver<Vec<u8>> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stream.read_to_end(&mut buf);
        let _ = tx.send(buf);
    });
    rx
}

/// Run a language-specific compiler check on the given file.
/// Returns a `CompileResult` indicating success and any errors.
///
/// # Errors
///
/// Returns an error if the compiler invocation fails.
pub fn compile_check(file_path: &Path, lang: &str, timeout_secs: u64) -> Result<CompileResult> {
    // Scratch dir for compiler artifacts; removed automatically on drop.
    let sink_dir = tempfile::tempdir()?;
    let Some((cmd, args)) = compiler_command(file_path, lang, sink_dir.path()) else {
        return Ok(CompileResult {
            success: true,
            errors: vec![],
        });
    };

    // Run in the file's own directory so tools resolve relative configs.
    let working_dir = working_dir_for(file_path);

    let mut child = match Command::new(&cmd)
        .args(&args)
        .current_dir(working_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        // Compiler not installed: skip validation successfully.
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Warning: {cmd} not found; skipping compilation check.");
            return Ok(CompileResult {
                success: true,
                errors: vec![],
            });
        }
        Err(e) => return Err(anyhow::anyhow!("Failed to run compiler: {e}")),
    };

    let stdout_rx = child
        .stdout
        .take()
        .map_or_else(|| mpsc::channel().1, drain_stream);
    let stderr_rx = child
        .stderr
        .take()
        .map_or_else(|| mpsc::channel().1, drain_stream);

    let timeout = Duration::from_secs(timeout_secs.max(1));
    let status_code = if let Some(status) = child.wait_timeout(timeout)? {
        status.code().unwrap_or(1)
    } else {
        let _ = child.kill();
        let _ = child.wait();
        return Ok(CompileResult {
            success: false,
            errors: vec![CompileError {
                file: file_path.to_string_lossy().to_string(),
                line: 0,
                column: 0,
                message: format!("Compilation timed out after {timeout_secs} seconds"),
            }],
        });
    };

    let stderr = String::from_utf8_lossy(&stderr_rx.recv().unwrap_or_default()).to_string();
    let stdout = String::from_utf8_lossy(&stdout_rx.recv().unwrap_or_default()).to_string();

    if status_code == 0 {
        return Ok(CompileResult {
            success: true,
            errors: vec![],
        });
    }

    let errors = parse_compiler_output(&format!("{stderr}\n{stdout}"), lang);
    Ok(CompileResult {
        success: false,
        errors,
    })
}

/// Compile-check raw content by materializing it in a temp file with the
/// given extension. Used for delta compilation (baseline vs patched).
///
/// # Errors
///
/// Returns an error if the compiler invocation fails.
pub fn compile_check_content(content: &str, ext: &str, timeout_secs: u64) -> Result<CompileResult> {
    let dir = tempfile::tempdir()?;
    let file = dir.path().join(format!("snippet.{ext}"));
    std::fs::write(&file, content)?;
    compile_check(&file, ext, timeout_secs)
}

/// Extract error locations and messages from compiler output.
fn parse_compiler_output(output: &str, _lang: &str) -> Vec<CompileError> {
    let mut errors = Vec::new();
    let first_line = || {
        output
            .lines()
            .next()
            .unwrap_or("Compilation error")
            .to_string()
    };

    // Rust (rustc/cargo short format): file.rs:10:5: error[E0308]: message
    let rust_re = Regex::new(r"^(\S+?):(\d+):(\d+):\s+error(?:\[[^\]]*\])?:\s*(.+)$").unwrap();
    for cap in rust_re.captures_iter(output) {
        errors.push(CompileError {
            file: cap[1].to_string(),
            line: cap[2].parse().unwrap_or(0),
            column: cap[3].parse().unwrap_or(0),
            message: cap[4].to_string(),
        });
    }

    // TypeScript: src/app.ts(10,5): error TS2322: message
    let ts_re = Regex::new(r"^(.+?)\((\d+),(\d+)\):\s+error\s+(\w+):\s*(.+)$").unwrap();
    for cap in ts_re.captures_iter(output) {
        errors.push(CompileError {
            file: cap[1].to_string(),
            line: cap[2].parse().unwrap_or(0),
            column: cap[3].parse().unwrap_or(0),
            message: format!("{}: {}", &cap[4], &cap[5]),
        });
    }

    // Python: SyntaxError: message (file, line N)
    if errors.is_empty() {
        let py_re = Regex::new(r"line (\d+)").unwrap();
        if let Some(cap) = py_re.captures(output) {
            errors.push(CompileError {
                file: String::new(),
                line: cap[1].parse().unwrap_or(0),
                column: 0,
                message: first_line(),
            });
        }
    }

    // Fallback: if no structured errors found, return the whole output.
    if errors.is_empty() && !output.trim().is_empty() {
        errors.push(CompileError {
            file: String::new(),
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
        let output = "src/main.rs:10:5: error[E0308]: mismatched types";
        let errors = parse_compiler_output(output, "rs");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].file, "src/main.rs");
        assert_eq!(errors[0].line, 10);
        assert_eq!(errors[0].column, 5);
    }

    #[test]
    fn test_parse_typescript_error() {
        let output =
            "src/app.ts(10,5): error TS2322: Type 'string' is not assignable to type 'number'.";
        let errors = parse_compiler_output(output, "ts");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 10);
        assert_eq!(errors[0].column, 5);
    }

    #[test]
    fn test_parse_python_error() {
        let output = "SyntaxError: invalid syntax (test.py, line 10)";
        let errors = parse_compiler_output(output, "py");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].line, 10);
    }

    #[test]
    fn test_fallback_captures_raw_output() {
        let errors = parse_compiler_output("mystery failure", "go");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "mystery failure");
    }

    #[test]
    fn test_empty_output_has_no_errors() {
        let errors = parse_compiler_output("", "rs");
        assert!(errors.is_empty());
    }

    #[test]
    fn test_success_result() {
        let result = compile_check(Path::new("nonexistent.rs"), "txt", 30).unwrap();
        assert!(result.success);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_missing_tool_skips_check() {
        // An exotic-but-real path with an unknown tool exercises the skip path
        // through `compiler_command` returning None.
        let result = compile_check(Path::new("whatever.unknownext"), "cobol", 5).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_working_dir_for_bare_filename() {
        assert_eq!(working_dir_for(Path::new("main.rs")), Path::new("."));
        assert_eq!(working_dir_for(Path::new("src/main.rs")), Path::new("src"));
        assert_eq!(
            working_dir_for(Path::new("/abs/dir/main.rs")),
            Path::new("/abs/dir")
        );
    }

    #[test]
    fn test_find_cargo_manifest_none_for_tmp() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("a.rs");
        std::fs::write(&file, "fn main() {}").unwrap();
        assert!(find_cargo_manifest(&file).is_none());
    }

    #[test]
    fn test_find_cargo_manifest_walks_up() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Cargo.toml"), "[package]\nname='x'\n").unwrap();
        let nested = dir.path().join("src").join("deep");
        std::fs::create_dir_all(&nested).unwrap();
        let file = nested.join("a.rs");
        std::fs::write(&file, "fn main() {}").unwrap();
        let manifest = find_cargo_manifest(&file).unwrap();
        assert_eq!(manifest, dir.path().join("Cargo.toml"));
    }

    #[test]
    fn test_rustc_single_file_ok() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("ok.rs");
        std::fs::write(&file, "pub fn add(a: i32, b: i32) -> i32 { a + b }").unwrap();
        let result = compile_check(&file, "rs", 30).unwrap();
        assert!(result.success, "errors: {:?}", result.errors);
    }

    #[test]
    fn test_rustc_single_file_error() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("bad.rs");
        std::fs::write(&file, "fn main() { let x: i32 = \"oops\"; }").unwrap();
        let result = compile_check(&file, "rs", 30).unwrap();
        assert!(!result.success);
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.message.contains("mismatched types") || e.message.contains("E0308"))
        );
    }

    #[test]
    fn test_python_check_ok_and_bad() {
        let dir = tempfile::tempdir().unwrap();
        let ok = dir.path().join("ok.py");
        std::fs::write(&ok, "x = 1\n").unwrap();
        assert!(compile_check(&ok, "py", 30).unwrap().success);
        let bad = dir.path().join("bad.py");
        std::fs::write(&bad, "def f(:\n").unwrap();
        assert!(!compile_check(&bad, "py", 30).unwrap().success);
    }
}
