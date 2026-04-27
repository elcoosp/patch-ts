use patch_ts::matching::fuzzy_match_line;

#[test]
fn test_exact_match_within_radius() {
    let content = "line1\nline2\nline3\nline4";
    let lines: Vec<&str> = content.lines().collect();
    let result = fuzzy_match_line(&lines, 2, "line3", 2, 0.9, 0.2, content).unwrap();
    assert_eq!(result.index, 2);
    assert!(result.score >= 0.99);
}

#[test]
fn test_whitespace_normalized_match() {
    let content = "  line2  ";
    let lines: Vec<&str> = content.lines().collect();
    let result = fuzzy_match_line(&lines, 0, "line2", 0, 0.9, 0.2, content).unwrap();
    assert_eq!(result.index, 0);
}

#[test]
fn test_no_match_below_threshold() {
    let content = "apple\nbanana";
    let lines: Vec<&str> = content.lines().collect();
    let result = fuzzy_match_line(&lines, 0, "orange", 1, 0.9, 0.2, content);
    assert!(result.is_err());
}

#[test]
fn test_ambiguous_match_tie() {
    let content = "target\nother\ntarget";
    let lines: Vec<&str> = content.lines().collect();
    let result = fuzzy_match_line(&lines, 1, "target", 2, 0.9, 0.2, content);
    assert!(result.is_ok());
    let match_result = result.unwrap();
    assert_eq!(match_result.index, 0);
}
