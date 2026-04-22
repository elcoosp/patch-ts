use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_c_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.c");
    fs::write(&file_path, "#include <stdio.h>\nint main() {\n    printf(\"hello\");\n    return 0;\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("3")
        .arg("--old").arg("    printf(\"hello\");")
        .arg("--new").arg("    printf(\"world\");")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("printf(\"world\");"));
}

#[test]
fn test_c_balance_removes_extra_brace() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.c");
    fs::write(&file_path, "int main() {\n    return 0;\n}\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--apply")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content.trim_end(), "int main() {\n    return 0;\n}");
}
