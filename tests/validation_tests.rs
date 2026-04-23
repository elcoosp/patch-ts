use patch_ts::ast::RustLanguage;
use patch_ts::patch::{apply_literal_patch, PatchOptions};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_patch_rejected_on_syntax_error() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut lang = RustLanguage::new();
    let options = PatchOptions {
        no_auto_repair: true,  // Disable auto-repair so the syntax error causes rejection
        ..Default::default()
    };
    let result = apply_literal_patch(
        &file_path,
        1,
        "fn main() {}",
        "fn main() {", // missing closing brace
        options,
        &mut lang,
    );
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("syntax error") || err.contains("introduces syntax error"));
}

#[test]
fn test_patch_force_bypasses_validation() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut lang = RustLanguage::new();
    let options = PatchOptions {
        force: true,
        ..Default::default()
    };
    let result = apply_literal_patch(
        &file_path,
        1,
        "fn main() {}",
        "fn main() {",
        options,
        &mut lang,
    );
    assert!(result.is_ok());
    let new_content = fs::read_to_string(&file_path).unwrap();
    // Accept with or without trailing newline
    assert!(new_content.trim() == "fn main() {" || new_content.trim() == "fn main() {\n");
}

#[test]
fn test_patch_auto_repair_fixes_syntax_error() {
    // New test: shows that auto-repair (default) now successfully fixes a simple error
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut lang = RustLanguage::new();
    let options = PatchOptions::default(); // auto_repair enabled by default
    let result = apply_literal_patch(
        &file_path,
        1,
        "fn main() {}",
        "fn main() {", // missing closing brace – should be auto-repaired
        options,
        &mut lang,
    );
    assert!(result.is_ok());
    let new_content = fs::read_to_string(&file_path).unwrap();
    assert!(new_content.contains("fn main() {}") || new_content.contains("fn main() {\n}"));
}

#[test]
fn test_patch_on_already_invalid_file_allowed() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    // Start with invalid Rust
    fs::write(&file_path, "fn main() {\n").unwrap();

    let mut lang = RustLanguage::new();
    let options = PatchOptions::default();
    // Replace with another invalid but different content
    let result = apply_literal_patch(
        &file_path,
        1,
        "fn main() {",
        "fn invalid() {",
        options,
        &mut lang,
    );
    // Should succeed because original wasn't valid (no validation performed)
    assert!(result.is_ok());
}
