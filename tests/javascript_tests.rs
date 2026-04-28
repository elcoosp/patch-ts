use assert_cmd::Command; use std::fs; use tempfile::tempdir;
#[test]
fn test_js_balance_removes_extra_brace() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.js");
    fs::write(&fp, "function main() {\n  console.log(\"hi\");\n}\n}\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap().arg("balance").arg("--file").arg(fp.to_str().unwrap()).arg("--apply").assert().success();
    assert_eq!(fs::read_to_string(&fp).unwrap().trim_end(), "function main() {\n  console.log(\"hi\");\n}");
}
#[test]
fn test_js_balance_inserts_missing_brace_disabled() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.js");
    fs::write(&fp, "function main() {\n  console.log(\"hi\");\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap().arg("balance").arg("--file").arg(fp.to_str().unwrap()).arg("--apply").assert().success();
}
#[test]
fn test_js_patch_exact() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.js");
    fs::write(&fp, "let a = 1;\nlet b = 2;\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--no-compile-check")
        .write_stdin("<<< SEARCH\nlet a = 1;\n---\nlet a = 42;\n")
        .assert().success();
    assert_eq!(fs::read_to_string(&fp).unwrap(), "let a = 42;\nlet b = 2;\n");
}
#[test]
fn test_jsx_balance() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("component.jsx");
    fs::write(&fp, "const Comp = () => {\n  return <div>hi</div>;\n}\n}\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap().arg("balance").arg("--file").arg(fp.to_str().unwrap()).arg("--apply").assert().success();
    assert!(!fs::read_to_string(&fp).unwrap().contains("}\n}"));
}
#[test]
fn test_js_explain_json() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.js");
    fs::write(&fp, "let x = ;\n").unwrap();
    let out = Command::cargo_bin("patch-ts").unwrap()
        .arg("explain").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--json")
        .assert().success().get_output().stdout.clone();
    let json: serde_json::Value = serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
    assert_eq!(json["success"], false);
    assert_eq!(json["error"]["code"], "patch_ts::syntax_error");
}
#[test]
fn test_js_balance_removes_extra_paren() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.js");
    fs::write(&fp, "const x = (1 + 2));\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap().arg("balance").arg("--file").arg(fp.to_str().unwrap()).arg("--apply").assert().success();
    assert_eq!(fs::read_to_string(&fp).unwrap().trim_end(), "const x = (1 + 2);");
}
