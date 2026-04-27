use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_content_patch_relaxed_whitespace() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    // File has proper indentation
    fs::write(&file_path, "fn main() {\n    let x = 1;\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    // Agent omits indentation in search block
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--old").arg("fn main() { let x = 1; }")
        .arg("--new").arg("fn main() { let x = 2; }")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("let x = 2;"));
}

#[test]
fn test_content_patch_relaxed_comments() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {\n    let x = 1; // important\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    // Agent includes comment in search, but file also has comment (should still match)
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--old").arg("let x = 1; // important")
        .arg("--new").arg("let x = 2;")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("let x = 2;"));
}

#[test]
fn test_content_patch_fails_with_unmatchable_search() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--old").arg("completely different content")
        .arg("--new").arg("something")
        .assert()
        .failure();
}
