use patch_ts::patch::replace_byte_range;

#[test]
fn byte_span_replace_rust_fn() {
    let mut content = String::from("fn old_name() {\n    println!(\"hi\");\n}\n");
    // "old_name" is at bytes 3..11 (after "fn ")
    replace_byte_range(&mut content, 3..11, "new_name");
    assert!(content.contains("fn new_name()"));
}

#[test]
fn byte_span_replace_across_lines() {
    let mut content = String::from("line one\nline two\nline three\n");
    // "line two" starts at byte 9, ends at byte 17
    replace_byte_range(&mut content, 9..17, "REPLACED");
    assert_eq!(content, "line one\nREPLACED\nline three\n");
}

#[test]
#[should_panic]
fn byte_span_out_of_bounds_panics() {
    let mut content = String::from("short");
    replace_byte_range(&mut content, 10..15, "boom");
}
