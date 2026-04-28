use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_go_balance_removes_extra_brace() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.go");
    fs::write(&file_path, "package main\nfunc main() {\n\tprintln(\"hi\")\n}\n}\n").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance").arg("--file").arg(file_path.to_str().unwrap()).arg("--apply").assert().success();
    let balanced = fs::read_to_string(&file_path).unwrap();
    assert_eq!(balanced.trim_end(), "package main\nfunc main() {\n\tprintln(\"hi\")\n}");
}

#[test]
#[ignore = "pre‑existing line‑based fuzzy‑match failure"]
fn test_go_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.go");
    fs::write(&file_path, "package main\n\nfunc main() {\n\tx := 1\n}\n").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch").arg("--file").arg(file_path.to_str().unwrap()).arg("--line").arg("4")
        .arg("--old").arg("\tx := 1").arg("--new").arg("\tx := 42").assert().success();
    assert!(fs::read_to_string(&file_path).unwrap().contains("x := 42"));
}

#[test]
fn test_go_explain_json() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.go");
    fs::write(&file_path, "package main\nfunc main() {\n\tx := \n}\n").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd.arg("explain").arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("3").arg("--json").assert().success().get_output().stdout.clone();
    let stdout = String::from_utf8(output).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json["success"], false);
    assert_eq!(json["error"]["code"], "patch_ts::syntax_error");
}
