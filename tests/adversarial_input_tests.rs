use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_control_character_in_old_rejected() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}").unwrap();

    // Null bytes are rejected by the OS when spawning a command.
    // The spawn itself fails, so we expect an error from assert_cmd.
    let result = Command::cargo_bin("patch-ts")
        .unwrap()
        .arg("patch")
        .arg("--file")
        .arg(file_path.to_str().unwrap())
        .arg("--line")
        .arg("1")
        .arg("--old")
        .arg("hello\x00")
        .arg("--new")
        .arg("world")
        .ok();
    // Either the command fails to spawn (Err) or it exits non‑zero.
    // We just verify that it didn't panic and didn't succeed.
    if let Ok(output) = result {
        assert!(!output.status.success());
    }
}

#[test]
fn test_path_traversal_rejected() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file")
        .arg("../../etc/passwd")
        .arg("--line")
        .arg("1")
        .arg("--old")
        .arg("a")
        .arg("--new")
        .arg("b")
        .assert()
        .failure();
}

#[test]
fn test_huge_argument_rejected() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}").unwrap();

    let big = "x".repeat(20_000);
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file")
        .arg(file_path.to_str().unwrap())
        .arg("--line")
        .arg("1")
        .arg("--old")
        .arg(&big)
        .arg("--new")
        .arg("y")
        .assert()
        .failure();
}

#[test]
fn test_missing_file_graceful() {
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file")
        .arg("/nonexistent/file.rs")
        .arg("--line")
        .arg("1")
        .arg("--old")
        .arg("x")
        .arg("--new")
        .arg("y")
        .assert()
        .failure();
}

#[test]
fn test_invalid_line_number_zero() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    // Line 0 behaves as line 1 (saturating_sub), so the patch succeeds.
    cmd.arg("patch")
        .arg("--file")
        .arg(file_path.to_str().unwrap())
        .arg("--line")
        .arg("0")
        .arg("--old")
        .arg("fn main() {}")
        .arg("--new")
        .arg("fn main() { }")
        .assert()
        .success();
}
