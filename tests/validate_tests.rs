use patch_ts::validate::{validate_path, validate_string};

#[test]
fn test_reject_null() {
    assert!(validate_string("hello\x00", "test").is_err());
}

#[test]
fn test_accept_normal() {
    assert!(validate_string("hello world", "test").is_ok());
}

#[test]
fn test_accept_newline_tab() {
    assert!(validate_string("hello\nworld\t!", "test").is_ok());
}

#[test]
fn test_reject_bell() {
    assert!(validate_string("hello\x07", "test").is_err());
}

#[test]
fn test_reject_path_traversal() {
    assert!(validate_path("../../etc/passwd", false).is_err());
}

#[test]
fn test_allow_with_flag() {
    assert!(validate_path("../../etc/passwd", true).is_ok());
}

#[test]
fn test_normal_path_ok() {
    assert!(validate_path("src/main.rs", false).is_ok());
}
