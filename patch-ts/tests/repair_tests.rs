use patch_ts::ast::RustLanguage;
use patch_ts::repair::{balance_file, explain_error};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_balance_removes_extra_brace() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {\n    println!(\"hi\");\n}\n}\n").unwrap();

    let mut lang = RustLanguage::new();
    let result = balance_file(&file_path, None, false, &mut lang);
    assert!(result.is_ok());
    let balanced = fs::read_to_string(&file_path).unwrap();
    assert_eq!(balanced.trim_end(), "fn main() {\n    println!(\"hi\");\n}");
}

#[test]
fn test_balance_on_valid_file_does_nothing() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    let content = "fn main() {}\n";
    fs::write(&file_path, content).unwrap();

    let mut lang = RustLanguage::new();
    let result = balance_file(&file_path, None, false, &mut lang);
    assert!(result.is_ok());
    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, content);
}

#[test]
fn test_explain_on_error_line() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {\n    println!(\"hi\");\n}\n}\n").unwrap();

    let mut lang = RustLanguage::new();
    let diag = explain_error(&file_path, 4, false, &mut lang).unwrap();
    assert!(diag.is_some());
    let diag = diag.unwrap();
    assert!(diag.details.contains("extra"));
}

#[test]
fn test_balance_no_extra_delimiter_found() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() { let x = \"unclosed; }\n").unwrap();

    let mut lang = RustLanguage::new();
    let result = balance_file(&file_path, None, false, &mut lang);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Could not identify extra delimiter"));
}

#[test]
fn test_explain_error_on_valid_line() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut lang = RustLanguage::new();
    let diag = explain_error(&file_path, 1, false, &mut lang).unwrap();
    assert!(diag.is_none());
}

#[test]
fn test_balance_unfixable() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "fn main() { let x: = 1; }\n").unwrap();

    let mut lang = RustLanguage::new();
    let result = balance_file(&file_path, None, false, &mut lang);
    assert!(result.is_err());
}

#[test]
fn test_balance_removal_does_not_fix() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    // Syntax error that cannot be fixed by removing a single delimiter
    fs::write(&file_path, "fn main() { let x: = 1; }\n").unwrap();

    let mut lang = RustLanguage::new();
    let result = balance_file(&file_path, None, false, &mut lang);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Could not identify extra delimiter") || err.contains("did not fix"));
}
