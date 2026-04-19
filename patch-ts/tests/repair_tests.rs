use patch_ts::ast::{Language, RustLanguage};
use patch_ts::repair::{balance_file, explain_error};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_balance_removes_extra_brace() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {\n    println!(\"hi\");\n}\n}\n").unwrap();

    let lang = RustLanguage::new();
    let result = balance_file(&file_path, None, false, &lang);
    assert!(result.is_ok());
    let balanced = fs::read_to_string(&file_path).unwrap();
    assert_eq!(balanced, "fn main() {\n    println!(\"hi\");\n}\n");
}

#[test]
fn test_balance_on_valid_file_does_nothing() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    let content = "fn main() {}\n";
    fs::write(&file_path, content).unwrap();

    let lang = RustLanguage::new();
    let result = balance_file(&file_path, None, false, &lang);
    assert!(result.is_ok());
    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, content);
}

#[test]
fn test_explain_on_error_line() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {\n    println!(\"hi\");\n}\n}\n").unwrap();

    let lang = RustLanguage::new();
    let diag = explain_error(&file_path, 4, false, &lang).unwrap();
    assert!(diag.is_some());
    let diag = diag.unwrap();
    assert!(diag.details.contains("extra"));
}
