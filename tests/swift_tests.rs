use assert_cmd::Command; use std::fs; use tempfile::tempdir;
#[test]
fn test_swift_patch_exact() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("main.swift");
    fs::write(&fp, "print(\"hello\")\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--no-compile-check")
        .write_stdin("<<< SEARCH\nprint(\"hello\")\n---\nprint(\"world\")\n")
        .assert().success();
    assert!(fs::read_to_string(&fp).unwrap().contains("print(\"world\")"));
}
