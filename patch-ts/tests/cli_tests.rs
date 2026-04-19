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
