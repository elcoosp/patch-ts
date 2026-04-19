use patch_ts::ast::{Language, RustLanguage};
use patch_ts::patch::{apply_literal_patch, PatchOptions};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_patch_rejected_on_syntax_error() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut lang = RustLanguage::new();
    let options = PatchOptions::default();
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
    assert_eq!(new_content, "fn main() {");
}
