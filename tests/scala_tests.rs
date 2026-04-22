use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_scala_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("Main.scala");
    fs::write(&file_path, "object Main {\n  def main(args: Array[String]): Unit = {\n    println(\"hello\")\n  }\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("3")
        .arg("--old").arg("    println(\"hello\")")
        .arg("--new").arg("    println(\"world\")")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("println(\"world\")"));
}
