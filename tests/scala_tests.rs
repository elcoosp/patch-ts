use assert_cmd::Command; use std::fs; use tempfile::tempdir;
#[test]
fn test_scala_patch_exact() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("Main.scala");
    fs::write(&fp, "object Main {\n  def main(args: Array[String]): Unit = {\n    println(\"hello\")\n  }\n}\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--no-compile-check")
        .write_stdin("<<< SEARCH\n    println(\"hello\")\n---\n    println(\"world\")\n")
        .assert().success();
    assert!(fs::read_to_string(&fp).unwrap().contains("println(\"world\")"));
}
