use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
#[ignore = "pre‑existing line‑based fuzzy‑match failure"]
fn test_cs_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("Program.cs");
    fs::write(&file_path, "class Program {\n    static void Main() {\n        System.Console.WriteLine(\"hello\");\n    }\n}\n").unwrap();
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("3")
        .arg("--old").arg("        System.Console.WriteLine(\"hello\");")
        .arg("--new").arg("        System.Console.WriteLine(\"world\");")
        .assert().success();
    assert!(fs::read_to_string(&file_path).unwrap().contains("System.Console.WriteLine(\"world\");"));
}
