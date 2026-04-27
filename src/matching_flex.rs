use std::collections::HashSet;
use strsim::normalized_levenshtein;

// ------------------------------------------------------------------
// Content‑based search
// ------------------------------------------------------------------
pub fn find_content_block(content: &str, search: &str) -> Option<std::ops::Range<usize>> {
    content.find(search).map(|start| start..start + search.len())
}

pub fn normalize_whitespace(s: &str) -> String {
    let mut r = String::with_capacity(s.len());
    let mut last_was_space = false;
    for ch in s.chars() {
        if ch.is_ascii_whitespace() {
            if !last_was_space {
                r.push(' ');
                last_was_space = true;
            }
        } else {
            r.push(ch);
            last_was_space = false;
        }
    }
    r.trim().to_string()
}

pub fn strip_line_comments(line: &str) -> String {
    if let Some(pos) = line.find("//") {
        line[..pos].to_string()
    } else if let Some(pos) = line.find('#') {
        line[..pos].to_string()
    } else {
        line.to_string()
    }
}

pub fn find_content_block_relaxed(content: &str, search: &str) -> Option<std::ops::Range<usize>> {
    if let Some(range) = find_content_block(content, search) {
        return Some(range);
    }

    let normalized_search = normalize_whitespace(search);
    let normalized_content = normalize_whitespace(content);
    if normalized_content.find(&normalized_search).is_some() {
        let first_line = search.lines().next().unwrap_or("");
        if !first_line.is_empty() {
            if let Some(start) = content.find(first_line) {
                let end = (start + search.len()).min(content.len());
                return Some(start..end);
            }
        }
        return Some(0..content.len());
    }

    let stripped_search = strip_line_comments(search);
    let stripped_content: String = content
        .lines()
        .map(|l| strip_line_comments(l))
        .collect::<Vec<_>>()
        .join("\n");
    if let Some(_pos) = stripped_content.find(&stripped_search) {
        let first_line = stripped_search.lines().next().unwrap_or("");
        if !first_line.is_empty() {
            if let Some(start) = content.find(first_line) {
                return Some(start..(start + stripped_search.len()).min(content.len()));
            }
        }
        return Some(0..content.len());
    }

    jaccard_match(content, search, 0.7)
}

fn jaccard_match(
    content: &str,
    search: &str,
    threshold: f64,
) -> Option<std::ops::Range<usize>> {
    let search_tokens: HashSet<&str> = search.split_whitespace().collect();
    if search_tokens.is_empty() {
        return None;
    }
    let search_lines: Vec<&str> = search.lines().collect();
    let window_lines = search_lines.len();
    let content_lines: Vec<&str> = content.lines().collect();

    let mut best_score = 0.0;
    let mut best_start = 0usize;
    let mut best_end = 0usize;

    for i in 0..=content_lines.len().saturating_sub(window_lines) {
        let candidate = content_lines[i..i + window_lines].join("\n");
        let candidate_tokens: HashSet<&str> = candidate.split_whitespace().collect();
        let intersection = search_tokens.intersection(&candidate_tokens).count();
        let union = search_tokens.union(&candidate_tokens).count();
        let score = if union == 0 { 0.0 } else { intersection as f64 / union as f64 };
        if score > best_score {
            best_score = score;
            best_start = content_lines[..i].iter().map(|l| l.len() + 1).sum::<usize>();
            best_end = best_start + candidate.len();
        }
    }
    if best_score >= threshold {
        Some(best_start..best_end)
    } else {
        None
    }
}

// ------------------------------------------------------------------
// Hunk‑level fuzzy matching for unified diffs
// ------------------------------------------------------------------
pub fn match_hunk_by_context(
    file_content: &str,
    context_lines: &[&str],
    fuzz: usize,
) -> Option<usize> {
    let file_lines: Vec<&str> = file_content.lines().collect();
    let ctx_len = context_lines.len();
    if ctx_len == 0 || file_lines.len() < ctx_len {
        return Some(0usize);
    }

    for start in 0..=file_lines.len() - ctx_len {
        if file_lines[start..start + ctx_len] == context_lines[..] {
            return Some(start);
        }
    }

    let ctx_joined = context_lines.join("\n");
    let norm_ctx = normalize_whitespace(&ctx_joined);

    let mut best_score = 0.0;
    let mut best_idx = 0usize;

    let search_start = 0usize;
    let search_end = (file_lines.len() - ctx_len).min(fuzz);

    for i in search_start..=search_end {
        let candidate = file_lines[i..i + ctx_len].join("\n");
        let norm_candidate = normalize_whitespace(&candidate);
        let score = normalized_levenshtein(&norm_ctx, &norm_candidate);
        if score > best_score {
            best_score = score;
            best_idx = i;
        }
    }

    if best_score > 0.7 {
        Some(best_idx)
    } else {
        None
    }
}

pub fn correct_hunk_line_numbers(
    file_content: &str,
    hunk_header: &str,
    context_lines: &[&str],
    fuzz: usize,
) -> Option<String> {
    let match_line = match_hunk_by_context(file_content, context_lines, fuzz)?;
    let new_old_start = match_line + 1;
    let new_new_start = match_line + 1;

    let header_body = hunk_header.strip_prefix("@@")?.strip_suffix("@@")?;
    let parts: Vec<&str> = header_body.split_whitespace().collect();
    let old_len = if let Some(comma) = parts[0].find(',') {
        parts[0][comma + 1..].trim_end_matches(|c: char| !c.is_ascii_digit()).to_string()
    } else {
        "1".to_string()
    };
    let new_len = if let Some(comma) = parts[1].find(',') {
        parts[1][comma + 1..].trim_end_matches(|c: char| !c.is_ascii_digit()).to_string()
    } else {
        "1".to_string()
    };

    Some(format!(
        "@@ -{},{} +{},{} @@",
        new_old_start, old_len, new_new_start, new_len,
    ))
}

pub fn fix_all_hunk_headers(diff_text: &str, file_content: &str, fuzz: usize) -> String {
    let mut result = String::new();
    let lines: Vec<&str> = diff_text.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("@@") && line.ends_with("@@") {
            let mut context: Vec<&str> = Vec::new();
            let mut body_lines = Vec::new();
            let mut j = i + 1;
            while j < lines.len() {
                if lines[j].starts_with(' ') {
                    context.push(&lines[j][1..]);
                }
                body_lines.push(lines[j]);
                j += 1;
                if j < lines.len() && lines[j].starts_with("@@") {
                    break;
                }
            }
            if let Some(corrected) = correct_hunk_line_numbers(file_content, line, &context, fuzz) {
                result.push_str(&corrected);
                result.push('\n');
            } else {
                result.push_str(line);
                result.push('\n');
            }
            for bl in body_lines {
                result.push_str(bl);
                result.push('\n');
            }
            i = j;
        } else {
            result.push_str(line);
            result.push('\n');
            i += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_substring_match() {
        let content = "hello\nworld\nhello";
        let range = find_content_block(content, "world").unwrap();
        assert_eq!(range, 6..11);
    }

    #[test]
    fn test_no_match() {
        let content = "abc";
        assert!(find_content_block(content, "xyz").is_none());
    }

    #[test]
    fn test_whitespace_normalization() {
        let result = normalize_whitespace("  hello   world\t\nfoo");
        assert_eq!(result, "hello world foo");
    }

    #[test]
    fn test_strip_comments() {
        let line = "let x = 1; // this is a comment";
        let stripped = strip_line_comments(line);
        assert_eq!(stripped.trim(), "let x = 1;");
    }

    #[test]
    fn test_relaxed_whitespace() {
        let content = "fn main() {\n    println!(\"hi\");\n}";
        let search = "fn main() { println!(\"hi\"); }";
        let range = find_content_block_relaxed(content, search).unwrap();
        assert!(range.start < content.len() && range.end <= content.len());
    }

    #[test]
    fn test_jaccard_fallback() {
        let content = "fn old() { let x = 1; }";
        let search = "fn old() { let x = 2; }";
        let range = find_content_block_relaxed(content, search).unwrap();
        assert_eq!(range.start, 0);
    }

    #[test]
    fn test_hunk_context_exact_match() {
        let file = "line0\n context line1\n context line2\nline5";
        let ctx = vec!["context line1", "context line2"];
        let idx = match_hunk_by_context(file, &ctx, 5).unwrap();
        assert_eq!(idx, 1);
    }

    #[test]
    fn test_hunk_header_correction() {
        let file = "a\nb\nc\nd\ne";
        let header = "@@ -10,3 +10,3 @@";
        let ctx = vec!["b", "c"];
        let corrected = correct_hunk_line_numbers(file, header, &ctx, 5).unwrap();
        assert_eq!(corrected, "@@ -2,3 +2,3 @@");
    }

    #[test]
    fn test_fix_all_hunks() {
        let diff = "@@ -99,2 +99,2 @@\n context1\n context2\n";
        let file = "line0\n context1\n context2\nline3";
        let fixed = fix_all_hunk_headers(diff, file, 5);
        assert!(fixed.contains("@@ -2,2 +2,2 @@"));
    }
}
