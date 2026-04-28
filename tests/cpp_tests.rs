use assert_cmd::Command; use std::fs; use tempfile::tempdir;
#[test]
fn test_cpp_patch_exact() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.cpp");
    fs::write(&fp, "#include <iostream>\nint main() {\n    std::cout << \"hello\";\n    return 0;\n}\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--no-compile-check")
        .write_stdin("<<< SEARCH\n    std::cout << \"hello\";\n---\n    std::cout << \"world\";\n")
        .assert().success();
    assert!(fs::read_to_string(&fp).unwrap().contains("std::cout << \"world\";"));
}
#[test]
fn test_cpp_balance_removes_extra_brace() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.cpp");
    fs::write(&fp, "int main() {\n    return 0;\n}\n}\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap().arg("balance").arg("--file").arg(fp.to_str().unwrap()).arg("--apply").assert().success();
    assert_eq!(fs::read_to_string(&fp).unwrap().trim_end(), "int main() {\n    return 0;\n}");
}
