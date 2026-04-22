use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cli_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("2")
        .arg("--old").arg("line2")
        .arg("--new").arg("new line2")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line1\nnew line2\nline3\n");
}

#[test]
fn test_cli_patch_mismatch_json() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("2")
        .arg("--old").arg("wrong")
        .arg("--new").arg("new")
        .arg("--json")
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], false);
    assert!(json["error"]["code"].as_str().unwrap().contains("mismatch") || json["error"]["code"].as_str().unwrap().contains("error"));
}

#[test]
fn test_cli_heredoc_malformed() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "old line\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .write_stdin("missing separator\nnew line\n")
        .assert()
        .failure()
        .get_output()
        .stderr
        .clone();

    let stderr = String::from_utf8(output).unwrap();
    assert!(stderr.contains("Heredoc must contain"));
}

#[test]
fn test_cli_patch_json_success() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("2")
        .arg("--old").arg("line2")
        .arg("--new").arg("new line2")
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], true);
}

#[test]
fn test_cli_patch_force_invalid() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .arg("--old").arg("fn main() {}")
        .arg("--new").arg("fn main() {")
        .arg("--force")
        .assert()
        .success();
}

#[test]
fn test_cli_balance_dry_run() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {\n    println!(\"hi\");\n}\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains("Would remove extra delimiter"));
}

#[test]
fn test_cli_balance_json() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {\n    println!(\"hi\");\n}\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--apply")
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], true);
}

#[test]
fn test_cli_explain_json() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {\n    println!(\"hi\");\n}\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("explain")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("4")
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], false);
    assert_eq!(json["error"]["code"], "patch_ts::syntax_error");
}

#[test]
fn test_cli_patch_delete_missing_expect() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--delete").arg("2")
        .assert()
        .failure();
}

#[test]
fn test_cli_patch_heredoc_success() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "old line\n").unwrap();

    let heredoc = "<<<\nold line\n---\nnew line\n";
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .write_stdin(heredoc)
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content.trim_end(), "new line");
}

#[test]
fn test_cli_patch_diff_stdin() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let diff = r#"--- a/test.rs
+++ b/test.rs
@@ -1,3 +1,3 @@
 line1
-line2
+new line2
 line3
"#;
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--diff")
        .write_stdin(diff)
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line1\nnew line2\nline3\n");
}

#[test]
fn test_cli_patch_delete() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--delete").arg("2")
        .arg("--expect").arg("line2")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line1\nline3\n");
}

#[test]
fn test_cli_patch_after() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nline2\nline4\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--after").arg("2")
        .arg("--content").arg("line3")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line1\nline2\nline3\nline4\n");
}

#[test]
fn test_cli_balance_json_success_on_valid_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], true);
}

#[test]
fn test_cli_explain_json_on_valid_line() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("explain")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .arg("--json")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], true);
}

#[test]
fn test_cli_file_not_found_json() {
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("patch")
        .arg("--file").arg("/nonexistent/file.rs")
        .arg("--line").arg("1")
        .arg("--old").arg("x")
        .arg("--new").arg("y")
        .arg("--json")
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], false);
    assert!(json["error"]["code"].as_str().unwrap().contains("error"));
}

#[test]
fn test_cli_patch_syntax_error_json() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .arg("--old").arg("fn main() {}")
        .arg("--new").arg("fn main() {") // missing closing brace
        .arg("--json")
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], false);
    assert_eq!(json["error"]["code"], "patch_ts::syntax_error");
}

#[test]
fn test_cli_patch_fuzzy_single_line() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nline2\nline3\nline4\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("2")
        .arg("--old").arg("line3")
        .arg("--new").arg("new line3")
        .arg("--fuzz").arg("2")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line1\nline2\nnew line3\nline4\n");
}

#[test]
fn test_cli_marker_patch() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, r#"
// PATCH-ME: replace
fn old() {}
"#).unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--marker").arg("replace")
        .arg("--new").arg("fn new() {}")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("fn new()"));
    assert!(!content.contains("fn old()"));
}

#[test]
fn test_cli_json_output_with_suggestion() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nlineX\nline3\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("2")
        .arg("--old").arg("line2")
        .arg("--new").arg("new")
        .arg("--fuzz").arg("2")
        .arg("--json")
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["success"], false);
    assert!(json["error"]["suggestion"].as_str().unwrap().contains("--fuzz"));
}

#[test]
fn test_cli_diff_with_fuzz_flag_allowed() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    let diff = r#"--- a/test.rs
+++ b/test.rs
@@ -1,3 +1,3 @@
 line1
-line2
+new line2
 line3
"#;
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--diff")
        .arg("--fuzz").arg("2")  // Ensure fuzz is allowed with diff
        .write_stdin(diff)
        .assert()
        .success();
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line1\nnew line2\nline3\n");
}
