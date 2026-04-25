use anyhow::Result;
use regex::Regex;

/// Correct hallucinated line numbers in unified diff headers.
/// Returns the corrected unified diff string.
pub fn fix_hunk_headers(diff_text: &str, file_content: &str, fuzz: usize) -> Result<String> {
    let hunk_re = Regex::new(
        r"(?m)^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@(.*)\n?"
    ).unwrap();

    let file_lines: Vec<&str> = file_content.lines().collect();
    let mut corrected = String::new();
    let mut last_match_end = 0;

    for cap in hunk_re.captures_iter(diff_text) {
        // Copy any text between previous hunk and this one
        corrected.push_str(&diff_text[last_match_end..cap.get(0).unwrap().start()]);

        let old_start: usize = cap[1].parse().unwrap_or(1);
        let new_start: usize = cap[3].parse().unwrap_or(1);
        let _context = cap.get(5).map(|m| m.as_str()).unwrap_or("");

        // Find the end of this hunk: the next `@@` line or end of string.
        let hunk_header_end = cap.get(0).unwrap().end();
        let next_hunk = hunk_re.find_at(diff_text, hunk_header_end);
        let hunk_body_end = next_hunk.map(|m| m.start()).unwrap_or(diff_text.len());

        // Extract the body lines (everything after the header line)
        let body_text = &diff_text[hunk_header_end..hunk_body_end];

        // Extract context lines (lines starting with space) from the body
        let context_lines: Vec<&str> = body_text
            .lines()
            .filter(|l| l.starts_with(' '))
            .map(|l| &l[1..]) // remove leading space
            .collect();

        if context_lines.is_empty() {
            corrected.push_str(&cap[0]);
            corrected.push_str(body_text);
            last_match_end = hunk_body_end;
            continue;
        }

        // Search file for the context lines
        let match_pos = find_context(&file_lines, &context_lines, fuzz);
        if let Some(actual_line) = match_pos {
            let corrected_old_start = actual_line + 1; // 1-based
            let corrected_new_start = actual_line + 1;
            let old_count = context_lines.len();
            let new_count = context_lines.len();
            let new_header = format!(
                "@@ -{},{} +{},{} @@{}",
                corrected_old_start, old_count,
                corrected_new_start, new_count,
                cap.get(5).map(|m| m.as_str()).unwrap_or("")
            );
            corrected.push_str(&new_header);
            corrected.push_str("\n");
        } else {
            // Keep original header
            corrected.push_str(&cap[0]);
            corrected.push_str("\n");
        }

        // Append the body (unchanged)
        corrected.push_str(body_text);

        last_match_end = hunk_body_end;
    }

    // Append any remaining text
    corrected.push_str(&diff_text[last_match_end..]);

    Ok(corrected)
}

/// Simple search for context lines within the file.
fn find_context(file_lines: &[&str], context: &[&str], fuzz: usize) -> Option<usize> {
    if context.is_empty() {
        return Some(0);
    }
    let ctx_len = context.len();
    if file_lines.len() < ctx_len {
        return None;
    }

    for start in 0..=fuzz.min(file_lines.len() - ctx_len) {
        let mut matches = true;
        for (j, ctx_line) in context.iter().enumerate() {
            let file_idx = start + j;
            if file_idx >= file_lines.len() {
                matches = false;
                break;
            }
            let file_line = file_lines[file_idx];
            let similarity = strsim::normalized_levenshtein(file_line, ctx_line);
            if similarity < 0.7 {
                matches = false;
                break;
            }
        }
        if matches {
            return Some(start);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_offset() {
        let diff = "@@ -10,3 +10,3 @@\n context line 1\n context line 2\n context line 3\n";
        let file = "line0\n context line 1\n context line 2\n context line 3\nline5\n";
        let corrected = fix_hunk_headers(diff, file, 2).unwrap();
        assert!(corrected.contains("@@ -2,3 +2,3 @@"), "Expected corrected header, got: {}", corrected);
    }
}
