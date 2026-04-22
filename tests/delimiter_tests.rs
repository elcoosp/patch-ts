use patch_ts::ast::{DelimiterError, Language, RustLanguage};

#[test]
fn test_detect_extra_paren() {
    let mut lang = RustLanguage::new();
    let source = "fn main() { let x = (1 + 2)); }";
    let result = lang.parse(source);
    let errors = lang.find_delimiter_errors(&result);
    assert_eq!(errors.len(), 1);
    match &errors[0] {
        DelimiterError::Extra { delimiter, .. } => assert_eq!(*delimiter, ')'),
        _ => panic!("Expected Extra error"),
    }
}

#[test]
fn test_detect_missing_brace() {
    let mut lang = RustLanguage::new();
    let source = "fn main() { println!(\"hi\"); ";
    let result = lang.parse(source);
    let errors = lang.find_delimiter_errors(&result);
    assert_eq!(errors.len(), 1);
    match &errors[0] {
        DelimiterError::Missing { expected, .. } => assert_eq!(*expected, '}'),
        _ => panic!("Expected Missing error"),
    }
}

// #[test]
// fn test_detect_missing_bracket() {
//     let mut lang = RustLanguage::new();
//     let source = "fn main() { let arr = [1, 2, 3; }";
//     let result = lang.parse(source);
//     let errors = lang.find_delimiter_errors(&result);
//     assert!(!errors.is_empty(), "Expected at least one error");

//     // Look for a missing bracket error among all errors
//     let missing_bracket = errors
//         .iter()
//         .any(|e| matches!(e, DelimiterError::Missing { expected: ']', .. }));
//     assert!(
//         missing_bracket,
//         "Expected a missing ']' error, but got: {:?}",
//         errors
//     );
// }

// #[test]
// fn test_detect_multiple_errors() {
//     let mut lang = RustLanguage::new();
//     let source = "fn main() { let x = (1 + 2; } }";
//     let result = lang.parse(source);
//     let errors = lang.find_delimiter_errors(&result);
//     assert!(errors.len() >= 2);
// }

#[test]
fn test_no_errors_on_valid_code() {
    let mut lang = RustLanguage::new();
    let source = "fn main() { let x = (1 + 2); }";
    let result = lang.parse(source);
    let errors = lang.find_delimiter_errors(&result);
    assert_eq!(errors.len(), 0);
}
