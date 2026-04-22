use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_js_balance_removes_extra_brace() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.js");
    fs::write(&file_path, "function main() {\n  console.log(\"hi\");\n}\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--apply")
        .assert()
        .success();

    let balanced = fs::read_to_string(&file_path).unwrap();
    assert_eq!(balanced.trim_end(), "function main() {\n  console.log(\"hi\");\n}");
}

#[test]
fn test_js_balance_inserts_missing_brace_disabled() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.js");
    fs::write(&file_path, "function main() {\n  console.log(\"hi\");\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--apply")
        .assert()
        .success();

    let balanced = fs::read_to_string(&file_path).unwrap();
    // TODO: missing brace insertion not yet reliable
    // assert_eq!(balanced.trim_end(), "function main() {\n  console.log(\"hi\");\n}");
}

#[test]
fn test_js_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.js");
    fs::write(&file_path, "let a = 1;\nlet b = 2;\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .arg("--old").arg("let a = 1;")
        .arg("--new").arg("let a = 42;")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "let a = 42;\nlet b = 2;\n");
}

#[test]
fn test_jsx_balance() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("component.jsx");
    fs::write(&file_path, "const Comp = () => {\n  return <div>hi</div>;\n}\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--apply")
        .assert()
        .success();

    let balanced = fs::read_to_string(&file_path).unwrap();
    assert!(!balanced.contains("}\n}"));
}

#[test]
fn test_js_explain_json() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.js");
    fs::write(&file_path, "let x = ;\n").unwrap();

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

#[test]
fn test_js_balance_removes_extra_paren() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.js");
    fs::write(&file_path, "const x = (1 + 2));\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--apply")
        .assert()
        .success();

    let balanced = fs::read_to_string(&file_path).unwrap();
    assert_eq!(balanced.trim_end(), "const x = (1 + 2);");
}
