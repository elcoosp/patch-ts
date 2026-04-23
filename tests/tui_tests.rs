use patch_ts::tui::highlight;

#[test]
fn test_syntax_highlight_rust() {
    let code = "fn main() { println!(\"hi\"); }\n";
    let spans = highlight(code, "rust").unwrap();
    assert!(!spans.is_empty());
    assert!(spans.iter().any(|(_, style)| style.fg.is_some()));
}
