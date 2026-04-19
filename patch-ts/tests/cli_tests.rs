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
