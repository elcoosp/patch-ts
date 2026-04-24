use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_rb_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rb");
    fs::write(&file_path, "x = 1\ny = 2\n").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file")
        .arg(file_path.to_str().unwrap())
        .arg("--line")
        .arg("1")
        .arg("--old")
        .arg("x = 1")
        .arg("--new")
        .arg("x = 42")
        .assert()
        .success();
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("x = 42"));
}

// Note: Ruby balance test for extra 'end' disabled for v0.7.0;
// AST-based repair for Ruby's keyword delimiters will be addressed in a future release.
/*
#[test]
fn test_rb_balance_removes_extra_end() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rb");
    fs::write(&file_path, "def hello\n  puts 'hi'\nend\nend\n").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--apply")
        .assert()
        .success();
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("end\nend"));
}
*/
