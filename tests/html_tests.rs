use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_html_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.html");
    fs::write(&file_path, "<div>hello</div>\n").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch").arg("--file").arg(file_path.to_str().unwrap()).arg("--line").arg("1").arg("--old").arg("<div>hello</div>").arg("--new").arg("<span>hello</span>").assert().success();
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("<span>hello</span>"));
}
