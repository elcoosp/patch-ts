use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_unified_diff_with_offset_line_numbers() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    // The file has the correct context lines but they appear at a different position
    // than the diff expects.
    fs::write(&file_path, "// header comment\ncontext line1\ncontext line2\ncontext line3\n").unwrap();

    // Diff expects context at line 100, but the actual content is at line 2.
    let diff = "@@ -100,3 +100,3 @@\n context line1\n-context line2\n+new context line2\n context line3\n";

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--diff")
        .write_stdin(diff)
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("new context line2"));
    assert!(!content.contains("-context line2"));
}
