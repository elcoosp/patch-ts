use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_entity_body_replace_rust() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {\n    let x = 1;\n    println!(\"{}\", x);\n}\n").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--symbol").arg("main")
        .arg("--new").arg("    let x = 42;\n    println!(\"{}\", x);\n")
        .arg("--entity-body")
        .assert().success();
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("let x = 42;"));
    assert!(content.contains("fn main()"));
}

#[test]
fn test_entity_body_nonexistent_symbol() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--symbol").arg("nonexistent")
        .arg("--new").arg("x")
        .arg("--entity-body")
        .assert().failure();
}
