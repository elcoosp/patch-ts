use regex::Regex;

/// Extract unified diff blocks from free‑form text (prose, fences, mixed).
/// Returns a vector of extracted diff strings.
pub fn extract_diffs(input: &str) -> Vec<String> {
    let re = Regex::new(r"(?m)^(@@\s+-\d+(?:,\d+)?\s+\+\d+(?:,\d+)?\s+@@.*(?:\n.*)+)").unwrap();
    let mut diffs = Vec::new();
    for cap in re.captures_iter(input) {
        if let Some(m) = cap.get(1) {
            diffs.push(m.as_str().to_string());
        }
    }
    if diffs.is_empty() {
        // Fallback: try to find anything that looks like a unified diff header
        let re2 = Regex::new(r"(?s)(--- [^\n]+\n\+\+\+ [^\n]+\n.*)").unwrap();
        for cap in re2.captures_iter(input) {
            if let Some(m) = cap.get(1) {
                diffs.push(m.as_str().to_string());
            }
        }
    }
    diffs
}

/// Wrapper: sanitize LLM output and extract the first diff.
pub fn sanitize_and_extract_diff(input: &str) -> Option<String> {
    // First try to extract a fenced block
    if let Some((_, content)) = crate::sanitize::extract_fenced_block(input) {
        return Some(content.to_string());
    }
    // Then try to find diff hunks directly
    let diffs = extract_diffs(input);
    if diffs.is_empty() {
        None
    } else {
        Some(diffs[0].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_single_hunk() {
        let input = "Here's a fix:\n@@ -1,3 +1,3 @@\n line1\n-old\n+new\n line3\n";
        let diffs = extract_diffs(input);
        assert!(!diffs.is_empty());
        assert!(diffs[0].contains("@@"));
    }

    #[test]
    fn test_sanitize_and_extract_from_prose() {
        let input =
            "Sure! I'll apply this patch:\n```diff\n@@ -1 +1 @@\n-old\n+new\n```\nHope that helps!";
        let diff = sanitize_and_extract_diff(input).unwrap();
        assert!(diff.contains("old"));
    }
}
