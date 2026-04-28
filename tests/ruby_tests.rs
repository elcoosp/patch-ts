use assert_cmd::Command; use std::fs; use tempfile::tempdir;
#[test]
fn test_rb_patch_exact() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.rb");
    fs::write(&fp, "x = 1\ny = 2\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--no-compile-check")
        .write_stdin("<<< SEARCH\nx = 1\n---\nx = 42\n")
        .assert().success();
    assert!(fs::read_to_string(&fp).unwrap().contains("x = 42"));
}
