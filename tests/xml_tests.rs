use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_xml_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.xml");
    fs::write(&file_path, "<root><item>value</item></root>\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .arg("--old").arg("<root><item>value</item></root>")
        .arg("--new").arg("<root><item>new</item></root>")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("<item>new</item>"));
}
