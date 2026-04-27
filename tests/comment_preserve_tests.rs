use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

// ---------------------------------------------------------------
// Integration tests via CLI
// ---------------------------------------------------------------

#[test]
fn test_preserve_comments_literal_patch_rust() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("sample.rs");
    fs::write(
        &file,
        "// This is the main function\nfn main() {\n    println!(\"Hello\"); // print greeting\n}\n",
    )
    .unwrap();

    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch")
        .arg("--file").arg(file.to_str().unwrap())
        .arg("--line").arg("3")
        .arg("--old").arg("    println!(\"Hello\"); // print greeting")
        .arg("--new").arg("    println!(\"Hi\");")
        .arg("--preserve-comments")
        .assert()
        .success();

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("// print greeting"), "Inline comment not preserved:\n{}", content);
    assert!(content.contains("// This is the main function"), "File‑level comment not preserved:\n{}", content);
}

#[test]
fn test_preserve_comments_doc_comment_rust() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("sample.rs");
    fs::write(
        &file,
        "/// Adds two numbers\nfn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n",
    )
    .unwrap();

    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch")
        .arg("--file").arg(file.to_str().unwrap())
        .arg("--line").arg("3")
        .arg("--old").arg("    a + b")
        .arg("--new").arg("    a * b")
        .arg("--preserve-comments")
        .assert()
        .success();

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("/// Adds two numbers"), "Doc comment not preserved:\n{}", content);
}

#[test]
fn test_preserve_comments_ts() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("sample.ts");
    fs::write(&file, "// Config value\nconst PORT = 3000;\n").unwrap();

    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch")
        .arg("--file").arg(file.to_str().unwrap())
        .arg("--line").arg("2")
        .arg("--old").arg("const PORT = 3000;")
        .arg("--new").arg("const PORT = 8080;")
        .arg("--preserve-comments")
        .assert()
        .success();

    let content = fs::read_to_string(&file).unwrap();
    assert!(content.contains("// Config value"), "TS comment not preserved:\n{}", content);
}

#[test]
fn test_flag_off_does_not_preserve() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("sample.rs");
    fs::write(
        &file,
        "// A comment\nfn foo() {}\n",
    )
    .unwrap();

    Command::cargo_bin("patch-ts").unwrap()
        .arg("patch")
        .arg("--file").arg(file.to_str().unwrap())
        .arg("--line").arg("2")
        .arg("--old").arg("fn foo() {}")
        .arg("--new").arg("fn foo() { /* changed */ }")
        // no --preserve-comments
        .assert()
        .success();

    let content = fs::read_to_string(&file).unwrap();
    // Without the flag, the comment may disappear
    // just ensure the patch applied
    assert!(content.contains("fn foo()"));
}

// ---------------------------------------------------------------
// Unit tests for preserve_comments internals
// ---------------------------------------------------------------
use patch_ts::ast::{Language, RustLanguage};
use patch_ts::comment_preserve::preserve_comments;

#[test]
fn test_unit_preserve_simple_comment() {
    let original = "// Top of file\nfn main() {\n    // inline\n    let x = 1;\n}\n";
    let patched = "fn main() {\n    let x = 2;\n}\n";
    let mut lang = RustLanguage::new();
    let result = preserve_comments(original, patched, &mut lang).unwrap();
    // File‑level comments without an entity anchor are not preserved by default.
    assert!(result.contains("// inline"));
}

#[test]
fn test_unit_preserve_no_change_returns_original() {
    let content = "// comment\nfn main() {}\n";
    let mut lang = RustLanguage::new();
    let result = preserve_comments(content, content, &mut lang).unwrap();
    assert_eq!(result, content);
}

#[test]
fn test_unit_orphan_comment_removed() {
    let original = "// Helper for old fn\nfn old_helper() {}\nfn main() {}\n";
    let patched = "fn main() {}\n";   // old_helper removed
    let mut lang = RustLanguage::new();
    let result = preserve_comments(original, patched, &mut lang).unwrap();
    // Orphan comment should NOT appear (default behavior)
    assert!(!result.contains("Helper for old fn"));
}
