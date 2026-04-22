use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_multi_file_patch() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.rs"), "fn a() {}\n").unwrap();
    fs::write(dir.path().join("b.rs"), "fn b() {}\n").unwrap();

    let pattern_path = dir.path().join("*.rs");
    let pattern = pattern_path.to_str().unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--files").arg(pattern)
        .arg("--line").arg("1")
        .arg("--old").arg("fn a() {}")
        .arg("--new").arg("fn a_new() {}")
        .assert()
        .success();

    assert!(fs::read_to_string(dir.path().join("a.rs")).unwrap().contains("fn a_new()"));
    assert!(!fs::read_to_string(dir.path().join("b.rs")).unwrap().contains("fn a_new()"));
}

#[test]
fn test_multi_file_balance() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.rs"), "fn a() {}\n}\n").unwrap();
    fs::write(dir.path().join("b.rs"), "fn b() {}\n}\n").unwrap();

    let pattern_path = dir.path().join("*.rs");
    let pattern = pattern_path.to_str().unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--files").arg(pattern)
        .arg("--apply")
        .assert()
        .success();

    let content_a = fs::read_to_string(dir.path().join("a.rs")).unwrap();
    let content_b = fs::read_to_string(dir.path().join("b.rs")).unwrap();
    assert!(!content_a.contains("}\n}"));
    assert!(!content_b.contains("}\n}"));
}

#[test]
fn test_glob_no_match_error() {
    let dir = tempdir().unwrap();
    let pattern_path = dir.path().join("nonexistent-*.rs");
    let pattern = pattern_path.to_str().unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--files").arg(pattern)
        .arg("--line").arg("1")
        .arg("--old").arg("x")
        .arg("--new").arg("y")
        .assert()
        .failure()
        .stderr(predicates::str::contains("No files matched pattern"));
}
