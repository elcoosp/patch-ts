use assert_cmd::Command; use std::fs; use tempfile::tempdir;
#[test]
fn test_ts_balance_removes_extra_brace() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.ts");
    fs::write(&fp, "function main() {\n  console.log(\"hi\");\n}\n}\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap().arg("balance").arg("--file").arg(fp.to_str().unwrap()).arg("--apply").assert().success();
    assert_eq!(fs::read_to_string(&fp).unwrap().trim_end(), "function main() {\n  console.log(\"hi\");\n}");
}
#[test]
fn test_ts_balance_inserts_missing_paren() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.ts");
    fs::write(&fp, "const x = (1 + 2;\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap().arg("balance").arg("--file").arg(fp.to_str().unwrap()).arg("--apply").assert().success();
}
#[test]
fn test_ts_patch_exact() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.ts");
    fs::write(&fp, "const a = 1;\nconst b = 2;\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--no-compile-check")
        .write_stdin("<<< SEARCH\nconst a = 1;\n---\nconst a = 42;\n")
        .assert().success();
    assert_eq!(fs::read_to_string(&fp).unwrap(), "const a = 42;\nconst b = 2;\n");
}
#[test]
fn test_ts_patch_fuzzy() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.ts");
    fs::write(&fp, "// comment\nconst a = 1;\nconst b = 2;\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--no-compile-check")
        .write_stdin("<<< SEARCH\nconst a = 1;\n---\nconst a = 42;\n")
        .assert().success();
    assert_eq!(fs::read_to_string(&fp).unwrap(), "// comment\nconst a = 42;\nconst b = 2;\n");
}
#[test]
fn test_ts_explain_json() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.ts");
    fs::write(&fp, "const x = ;\n").unwrap();
    let out = Command::cargo_bin("patch-ts").unwrap()
        .arg("explain").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--json")
        .assert().success().get_output().stdout.clone();
    let json: serde_json::Value = serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
    assert_eq!(json["success"], false);
    assert_eq!(json["error"]["code"], "patch_ts::syntax_error");
}
#[test]
fn test_ts_unsupported_extension_errors() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.txt");
    fs::write(&fp, "hello").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1")
        .arg("--old").arg("hello").arg("--new").arg("world")
        .assert().failure().stderr(predicates::str::contains("Unsupported file extension"));
}
#[test]
fn test_ts_balance_removes_extra_paren() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.ts");
    fs::write(&fp, "const x = (1 + 2));\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap().arg("balance").arg("--file").arg(fp.to_str().unwrap()).arg("--apply").assert().success();
    assert_eq!(fs::read_to_string(&fp).unwrap().trim_end(), "const x = (1 + 2);");
}
