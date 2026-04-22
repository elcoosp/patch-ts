use patch_ts::ast::RustLanguage;
use patch_ts::repair::{balance_file, explain_error, quick_balance};
use patch_ts::ast::{DelimiterError, Span};
use patch_ts::repair::apply_repair;
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
    assert!(result.unwrap_err().to_string().contains("Could not identify any delimiter errors"));
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
    fs::write(&file_path, "fn main() { let x: = 1; }\n").unwrap();

    let mut lang = RustLanguage::new();
    let result = balance_file(&file_path, None, false, &mut lang);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Could not identify any delimiter errors") || err.contains("did not fix"));
}

#[test]
fn test_quick_balance_fixes_extra_brace() {
    let mut lang = RustLanguage::new();
    let content = "fn main() {\n    println!(\"hi\");\n}\n}\n";
    let fixed = quick_balance(content, &mut lang).unwrap();
    assert_eq!(fixed.trim_end(), "fn main() {\n    println!(\"hi\");\n}");
}

#[test]
fn test_quick_balance_returns_none_if_unfixable() {
    let mut lang = RustLanguage::new();
    let content = "fn main() { let x: = 1; }\n";
    assert!(quick_balance(content, &mut lang).is_none());
}

#[test]
fn test_apply_repair_extra() {
    let content = "fn main() { let x = (1 + 2)); }";
    let span = Span {
        start_byte: 26,
        end_byte: 27,
        start_line: 1,
        start_column: 27,
        end_line: 1,
        end_column: 28,
    };
    let error = DelimiterError::Extra { span, delimiter: ')' };
    let repaired = apply_repair(content, &error);
    assert_eq!(repaired, "fn main() { let x = (1 + 2); }");
}

#[test]
fn test_apply_repair_missing() {
    let content = "fn main() { println!(\"hi\"); ";
    let end_byte = content.len();
    let span = Span {
        start_byte: 0,
        end_byte,
        start_line: 1,
        start_column: 1,
        end_line: 1,
        end_column: content.len() + 1,
    };
    let error = DelimiterError::Missing { expected: '}', insert_at: span };
    let repaired = apply_repair(content, &error);
    assert_eq!(repaired, "fn main() { println!(\"hi\"); }");
}

#[test]
fn test_balance_function_scoped() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    let content = r#"
fn foo() {
    if true {
        let x = 1;
    // missing closing brace for if
}
fn bar() {
    let y = (3 + 4));
}
"#;
    fs::write(&file_path, content).unwrap();

    let mut lang = RustLanguage::new();
    balance_file(&file_path, Some("foo"), false, &mut lang).unwrap();
    let balanced = fs::read_to_string(&file_path).unwrap();

    assert!(balanced.contains("if true {"));
    assert!(balanced.contains("let x = 1;"));
}

#[test]
fn test_balance_function_ambiguous() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    let content = r#"
fn foo() { let x = (1 + 2; }
fn foo() { let y = (3 + 4; }
"#;
    fs::write(&file_path, content).unwrap();

    let mut lang = RustLanguage::new();
    let result = balance_file(&file_path, Some("foo"), false, &mut lang);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("ambiguous"));
}

#[test]
fn test_balance_function_not_found() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    let content = "fn main() {}";
    fs::write(&file_path, content).unwrap();

    let mut lang = RustLanguage::new();
    let result = balance_file(&file_path, Some("nonexistent"), false, &mut lang);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}
