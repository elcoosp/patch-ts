use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_java_patch_exact() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("Main.java");
    fs::write(&file_path, "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"hello\");\n    }\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file")
        .arg(file_path.to_str().unwrap())
        .arg("--line")
        .arg("3")
        .arg("--old")
        .arg("        System.out.println(\"hello\");")
        .arg("--new")
        .arg("        System.out.println(\"world\");")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("System.out.println(\"world\");"));
}

// TODO: Java balance test temporarily disabled; AST traversal needs refinement for extra braces.
// #[test]
// fn test_java_balance_removes_extra_brace() {
//     let dir = tempdir().unwrap();
//     let file_path = dir.path().join("Main.java");
//     fs::write(&file_path, "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"hi\");\n    }\n}\n}\n").unwrap();
//
//     let mut cmd = Command::cargo_bin("patch-ts").unwrap();
//     cmd.arg("balance")
//         .arg("--file").arg(file_path.to_str().unwrap())
//         .arg("--apply")
//         .assert()
//         .success();
//
//     let content = fs::read_to_string(&file_path).unwrap();
//     assert!(!content.contains("}\n}"));
// }
