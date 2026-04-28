use assert_cmd::Command; use std::fs; use tempfile::tempdir;
#[test]
fn test_php_balance_inserts_missing_brace() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.php");
    fs::write(&fp, "<?php function hello() { echo 'hi';").unwrap();
    Command::cargo_bin("patch-ts").unwrap().arg("balance").arg("--file").arg(fp.to_str().unwrap()).arg("--apply").assert().success();
    assert!(fs::read_to_string(&fp).unwrap().contains("}"));
}
#[test]
fn test_php_patch_exact() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("sample.php");
    fs::write(&fp, "<?php\n$x = 1;\n$y = 2;\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--no-compile-check")
        .write_stdin("<<< SEARCH\n$x = 1;\n---\n$x = 42;\n")
        .assert().success();
    assert!(fs::read_to_string(&fp).unwrap().contains("$x = 42;"));
}
