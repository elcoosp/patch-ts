use patch_ts::tui::highlight;

#[test]
fn test_syntax_highlight_rust() {
    let code = "fn main() { println!(\"hi\"); }\n";
    let spans = highlight(code, "rust").unwrap();
    assert_ne!(spans.len(), 0, "expected highlighted spans");
    assert!(spans.iter().any(|(_, style)| style.fg.is_some()));
}
