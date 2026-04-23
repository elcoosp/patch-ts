use patch_ts::matching::fuzzy_match_line;

#[test]
fn test_exact_match_within_radius() {
    let lines = vec!["line1", "line2", "line3", "line4"];
    let result = fuzzy_match_line(&lines, 2, "line3", 2, 0.9, 0.2).unwrap();
    assert_eq!(result.index, 2);
    assert!(result.score >= 0.99);
}

#[test]
fn test_whitespace_normalized_match() {
    let lines = vec!["  line2  "];
    let result = fuzzy_match_line(&lines, 0, "line2", 0, 0.9, 0.2).unwrap();
    assert_eq!(result.index, 0);
}

#[test]
fn test_no_match_below_threshold() {
    let lines = vec!["apple", "banana"];
    let result = fuzzy_match_line(&lines, 0, "orange", 1, 0.9, 0.2);
    assert!(result.is_err());
}

#[test]
fn test_ambiguous_match_tie() {
    let lines = vec!["target", "other", "target"];
    let result = fuzzy_match_line(&lines, 1, "target", 2, 0.9, 0.2);
    assert!(result.is_ok());
    let match_result = result.unwrap();
    assert_eq!(match_result.index, 0);
}

use patch_ts::matching::find_best_block_match;

#[test]
fn test_block_match_with_offset() {
    let lines: Vec<&str> = vec![
        "// comment",
        "fn foo() {",
        "    println!(\"hi\");",
        "}",
        "other stuff",
    ];
    let expected = "fn foo() {\n    println!(\"hi\");\n}";
    let result = find_best_block_match(&lines, expected, 2, 0.9).unwrap();
    assert_eq!(result.start_index, 1);
    assert_eq!(result.end_index, 3);
    assert!(result.score >= 0.9);
}
