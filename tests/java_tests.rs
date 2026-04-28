use assert_cmd::Command; use std::fs; use tempfile::tempdir;
#[test]
fn test_java_patch_exact() {
    let dir = tempdir().unwrap();
    let fp = dir.path().join("Main.java");
    fs::write(&fp, "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"hello\");\n    }\n}\n").unwrap();
    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch").arg("--file").arg(fp.to_str().unwrap()).arg("--line").arg("1").arg("--no-compile-check")
        .write_stdin("<<< SEARCH\n        System.out.println(\"hello\");\n---\n        System.out.println(\"world\");\n")
        .assert().success();
    assert!(fs::read_to_string(&fp).unwrap().contains("System.out.println(\"world\");"));
}
