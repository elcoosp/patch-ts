use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_php_balance_inserts_missing_brace() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.php");
    fs::write(&file_path, "<?php function hello() { echo 'hi';").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance").arg("--file").arg(file_path.to_str().unwrap()).arg("--apply").assert().success();
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("}"));
}

#[test]
fn test_php_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.php");
    fs::write(&file_path, "<?php\n$x = 1;\n$y = 2;\n").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch").arg("--file").arg(file_path.to_str().unwrap()).arg("--line").arg("2").arg("--old").arg("$x = 1;").arg("--new").arg("$x = 42;").assert().success();
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("$x = 42;"));
}
