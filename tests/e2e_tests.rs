use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn e2e_type_annotation_fix() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("state.rs");
    let content = r#"
impl RouterState {
    pub fn navigate(&mut self, loc: Location, options: NavigateOptions, cx: &mut App) {
        cx.spawn(|cx| async move {
            // ...
        }).detach();
    }
}
"#;
    fs::write(&file_path, content).unwrap();

    let heredoc = r#"<<<
        cx.spawn(|cx| async move {
---
        cx.spawn(|_cx: &mut App| async move {
"#;
    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file")
        .arg(file_path.to_str().unwrap())
        .arg("--line")
        .arg("3")
        .write_stdin(heredoc)
        .assert()
        .success();

    let patched = fs::read_to_string(&file_path).unwrap();
    assert!(patched.contains("_cx: &mut App"));
}

#[test]
fn e2e_balance_removes_extra_brace() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "fn main() {\n    println!(\"hi\");\n}\n}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("balance")
        .arg("--file")
        .arg(file_path.to_str().unwrap())
        .arg("--apply")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content.trim_end(), "fn main() {\n    println!(\"hi\");\n}");
}
