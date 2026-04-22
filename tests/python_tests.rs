use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_py_balance_removes_extra_paren() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.py");
    fs::write(&file_path, "print((\"hello\"))\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--apply")
        .assert()
        .success();

    let balanced = fs::read_to_string(&file_path).unwrap();
    assert_eq!(balanced.trim_end(), "print(\"hello\")");
}

#[test]
fn test_py_balance_inserts_missing_paren() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.py");
    fs::write(&file_path, "x = (1 + 2\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--apply")
        .assert()
        .success();

    let balanced = fs::read_to_string(&file_path).unwrap();
    assert_eq!(balanced.trim_end(), "x = (1 + 2)");
}

#[test]
fn test_py_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.py");
    fs::write(&file_path, "x = 1\ny = 2\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .arg("--old").arg("x = 1")
        .arg("--new").arg("x = 42")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "x = 42\ny = 2\n");
}

#[test]
fn test_py_explain_json() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.py");
    fs::write(&file_path, "x = \n").unwrap();

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
    assert_eq!(json["success"], false);
    assert_eq!(json["error"]["code"], "patch_ts::syntax_error");
}
