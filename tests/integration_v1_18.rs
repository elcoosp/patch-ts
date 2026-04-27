use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

// ── Content‑based replacement (no line number) ──
#[test]
fn integration_content_based_exact() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("lib.rs");
    fs::write(&file, "fn old() {\n    do_thing();\n}\n").unwrap();

    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch")
        .arg("--file").arg(file.to_str().unwrap())
        .arg("--old").arg("fn old() {\n    do_thing();\n}")
        .arg("--new").arg("fn old() {\n    do_other();\n}")
        .assert().success();

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("do_other();"));
}

#[test]
fn integration_content_based_relaxed() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("main.rs");
    fs::write(&file, "fn main() {\n    println!(\"hi\");\n}\n").unwrap();

    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch")
        .arg("--file").arg(file.to_str().unwrap())
        .arg("--old").arg("fn main() { println!(\"hi\"); }")   // single line
        .arg("--new").arg("fn main() { println!(\"bye\"); }")
        .assert().success();

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("println!(\"bye\");"));
}

// ── SEARCH/REPLACE block via heredoc ──
#[test]
fn integration_search_replace_block() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("lib.rs");
    fs::write(&file, "fn greet() {\n    println!(\"hello\");\n}\n").unwrap();

    let input = "<<< SEARCH\nfn greet() {\n    println!(\"hello\");\n}\n---\nfn greet() {\n    println!(\"hi\");\n}\n";

    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch")
        .arg("--file").arg(file.to_str().unwrap())
        .arg("--line").arg("1")
        .write_stdin(input)
        .assert().success();

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("println!(\"hi\");"));
}

// ── Entity body replacement ──
#[test]
fn integration_entity_body_replace() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("main.rs");
    fs::write(&file, "fn main() {\n    let x = 1;\n}\n").unwrap();

    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch")
        .arg("--file").arg(file.to_str().unwrap())
        .arg("--symbol").arg("main")
        .arg("--new").arg("    let x = 42;\n")
        .arg("--entity-body")
        .assert().success();

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("let x = 42;"));
    assert!(content.contains("fn main()"));
}

// ── Hunk fuzzy matching (unified diff with offset line numbers) ──
#[test]
fn integration_hunk_fuzzy_diff() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("lib.rs");
    fs::write(&file, "// header\nline a\nline b\nline c\n").unwrap();

    let diff = "@@ -100,3 +100,3 @@\n line a\n-line b\n+new b\n line c\n";

    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch")
        .arg("--file").arg(file.to_str().unwrap())
        .arg("--diff")
        .write_stdin(diff)
        .assert().success();

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("new b"));
}
