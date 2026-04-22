use patch_ts::marker::{find_marker, replace_marker_node};

#[test]
fn test_find_marker_single() {
    let source = r#"
// PATCH-ME: update
fn old() {}
"#;
    let markers = find_marker(source, "update").unwrap();
    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0].line, 2);
}

#[test]
fn test_find_marker_not_found() {
    let source = "fn main() {}";
    let result = find_marker(source, "missing");
    assert!(result.is_err());
}

#[test]
fn test_find_marker_multiple() {
    let source = r#"
// PATCH-ME: dup
fn a() {}
// PATCH-ME: dup
fn b() {}
"#;
    let markers = find_marker(source, "dup").unwrap();
    assert_eq!(markers.len(), 2);
}

#[test]
fn test_replace_marker_node() {
    let source = r#"
// PATCH-ME: replace
fn old() { println!("old"); }
"#;
    let new_content = "fn new() { println!(\"new\"); }";
    let result = replace_marker_node(source, "replace", new_content).unwrap();
    assert!(result.contains("fn new()"));
    assert!(!result.contains("fn old()"));
}
