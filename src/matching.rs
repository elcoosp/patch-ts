use anyhow::{anyhow, Result};
use std::collections::HashSet;
use strsim::normalized_levenshtein;

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub index: usize,
    pub score: f64,
    pub confidence: f64,
    pub strategy: String,
    pub uniqueness_score: f64,
    /// Byte range (start, end) of the matched text within the original content.
    pub match_byte_range: (usize, usize),
}

fn normalize_line(s: &str) -> String {
    s.trim().split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn compute_uniqueness(line: &str, all_lines: &[&str]) -> f64 {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return 0.5;
    }
    let count = all_lines.iter().filter(|l| l.trim() == trimmed).count();
    if count == 0 {
        1.0
    } else {
        1.0 / (count as f64)
    }
}

fn adjusted_threshold(base_threshold: f64, uniqueness: f64, weight: f64) -> f64 {
    let adjustment = weight * (1.0 - uniqueness);
    (base_threshold + adjustment).min(0.99)
}

#[allow(unused_variables)]
fn most_unique_lines<'a>(
    lines: &'a [&str],
    all_lines: &[&str],
    n: usize,
) -> Vec<(usize, &'a str, f64)> {
    let mut scored: Vec<(usize, &str, f64)> = lines
        .iter()
        .enumerate()
        .map(|(i, &l)| (i, l, compute_uniqueness(l, all_lines)))
        .collect();
    scored.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    scored.truncate(n);
    scored
}

#[allow(unused_variables)]
fn try_anchor_pair(
    lines: &[&str],
    expected_lines: &[&str],
    fuzz_radius: usize,
    max_gap: usize,
) -> Option<usize> {
    if expected_lines.len() < 2 {
        return None;
    }
    let best_pair = most_unique_lines(expected_lines, lines, 2);
    if best_pair.len() < 2 {
        return None;
    }
    let (i1, line1, _) = best_pair[0];
    let (i2, line2, _) = best_pair[1];
    let (anchor1, anchor2) = if i1 < i2 { (i1, i2) } else { (i2, i1) };
    let gap = anchor2 - anchor1;

    let start = 0usize.saturating_sub(fuzz_radius);
    let end = (lines.len().saturating_sub(1)).min(lines.len().saturating_sub(gap) + fuzz_radius);

    for i in start..=end {
        if i + anchor1 >= lines.len() || i + anchor2 >= lines.len() {
            continue;
        }
        if lines[i + anchor1] == line1 && lines[i + anchor2] == line2 {
            return Some(i);
        }
    }
    None
}

/// Compute the byte range of a matched line within the original content.
///
/// `lines` contains `&str` slices that point directly into `content`.
/// The byte offset is computed via pointer arithmetic.
fn line_byte_range(lines: &[&str], idx: usize, content: &str) -> (usize, usize) {
    let line = lines[idx];
    let start = line.as_ptr() as usize - content.as_ptr() as usize;
    (start, start + line.len())
}

fn try_ellipsis_match(
    lines: &[&str],
    expected: &str,
    fuzz_radius: usize,
    content: &str,
) -> Option<MatchResult> {
    let expected_lines: Vec<&str> = expected.lines().collect();
    let ellipsis_pos = expected_lines.iter().position(|l| l.trim() == "...")?;
    let before: Vec<&str> = expected_lines[..ellipsis_pos].to_vec();
    let after: Vec<&str> = expected_lines[ellipsis_pos + 1..].to_vec();

    let target_idx = 0usize;
    let start = target_idx.saturating_sub(fuzz_radius);
    let end = (target_idx + fuzz_radius).min(lines.len().saturating_sub(1));

    for i in start..=end {
        if i + before.len() > lines.len() {
            continue;
        }
        let mut matches = true;
        for (j, bline) in before.iter().enumerate() {
            if lines[i + j] != *bline {
                matches = false;
                break;
            }
        }
        if !matches {
            continue;
        }
        let after_start = i + before.len();
        for gap in 0..(lines.len() - after_start) {
            let aj = after_start + gap;
            if aj + after.len() > lines.len() {
                break;
            }
            let mut after_matches = true;
            for (k, aline) in after.iter().enumerate() {
                if lines[aj + k] != *aline {
                    after_matches = false;
                    break;
                }
            }
            if after_matches {
                let byte_range = line_byte_range(lines, i, content);
                return Some(MatchResult {
                    index: i,
                    score: 1.0,
                    confidence: 0.95,
                    strategy: "ellipsis".to_string(),
                    uniqueness_score: 1.0,
                    match_byte_range: byte_range,
                });
            }
        }
    }
    None
}

pub fn cascade_match(
    lines: &[&str],
    target_line: usize,
    expected: &str,
    fuzz_radius: usize,
    similarity_threshold: f64,
    uniqueness_weight: f64,
    content: &str,
) -> Result<MatchResult> {
    let target_idx = target_line.saturating_sub(1);
    let start = target_idx.saturating_sub(fuzz_radius);
    let end = (target_idx + fuzz_radius).min(lines.len().saturating_sub(1));

    if start > end {
        anyhow::bail!("empty search range");
    }

    let uniqueness = compute_uniqueness(expected, lines);
    let effective_threshold =
        adjusted_threshold(similarity_threshold, uniqueness, uniqueness_weight);

    // 1. Exact match at target line
    if target_idx < lines.len() && lines[target_idx] == expected {
        return Ok(MatchResult {
            index: target_idx,
            score: 1.0,
            confidence: 1.0,
            strategy: "exact".to_string(),
            uniqueness_score: uniqueness,
            match_byte_range: line_byte_range(lines, target_idx, content),
        });
    }

    // 2. Anchor‑based: any line in range matches exactly
    for i in start..=end {
        if lines[i] == expected {
            return Ok(MatchResult {
                index: i,
                score: 1.0,
                confidence: 1.0,
                strategy: "anchor".to_string(),
                uniqueness_score: uniqueness,
                match_byte_range: line_byte_range(lines, i, content),
            });
        }
    }

    // 2b. Multi‑line anchor fallback (try pair of unique lines)
    if expected.lines().count() >= 2 {
        if let Some(idx) = try_anchor_pair(
            lines,
            &expected.lines().collect::<Vec<_>>(),
            fuzz_radius,
            10,
        ) {
            return Ok(MatchResult {
                index: idx,
                score: 0.98,
                confidence: 0.98,
                strategy: "anchor_pair".to_string(),
                uniqueness_score: uniqueness,
                match_byte_range: line_byte_range(lines, idx, content),
            });
        }
    }

    // 3. Ellipsis pattern (if applicable)
    if expected.contains("\n...\n") || expected.lines().any(|l| l.trim() == "...") {
        if let Some(m) = try_ellipsis_match(lines, expected, fuzz_radius, content) {
            return Ok(m);
        }
    }

    // 4. Similarity – token‑based Jaccard
    let expected_tokens: HashSet<&str> = expected.split_whitespace().collect();
    let mut best_conf = 0.0f64;
    let mut best_idx = start;
    for i in start..=end {
        let actual_tokens: HashSet<&str> = lines[i].split_whitespace().collect();
        let intersection = expected_tokens.intersection(&actual_tokens).count();
        let union = expected_tokens.union(&actual_tokens).count();
        let conf = if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        };
        if conf > best_conf {
            best_conf = conf;
            best_idx = i;
        }
    }
    if best_conf >= effective_threshold {
        return Ok(MatchResult {
            index: best_idx,
            score: best_conf,
            confidence: best_conf,
            strategy: "similarity".to_string(),
            uniqueness_score: uniqueness,
            match_byte_range: line_byte_range(lines, best_idx, content),
        });
    }

    // 5. Fuzzy – normalized Levenshtein
    let normalized_expected = normalize_line(expected);
    for i in start..=end {
        let normalized_actual = normalize_line(lines[i]);
        let score = normalized_levenshtein(&normalized_expected, &normalized_actual);
        if score >= effective_threshold {
            return Ok(MatchResult {
                index: i,
                score,
                confidence: score,
                strategy: "fuzzy".to_string(),
                uniqueness_score: uniqueness,
                match_byte_range: line_byte_range(lines, i, content),
            });
        }
    }

    anyhow::bail!(
        "no match found with any strategy (best confidence {:.2}, threshold {:.2}, uniqueness {:.2})",
        best_conf, effective_threshold, uniqueness
    )
}

#[allow(unused_variables)]
pub fn fuzzy_match_line(
    lines: &[&str],
    target_line: usize,
    expected: &str,
    fuzz_radius: usize,
    similarity_threshold: f64,
    uniqueness_weight: f64,
    content: &str,
) -> Result<MatchResult> {
    cascade_match(
        lines,
        target_line,
        expected,
        fuzz_radius,
        similarity_threshold,
        uniqueness_weight,
        content,
    )
}

// Block matching
use crate::ast::{Language, RustLanguage};
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct BlockMatchResult {
    pub start_index: usize,
    pub end_index: usize,
    pub score: f64,
}

fn tokenize_rust(code: &str) -> Vec<String> {
    let mut lang = RustLanguage::new();
    let parse_result = lang.parse(code);
    let mut tokens = Vec::new();
    let root = parse_result.tree.root_node();
    collect_tokens(root, parse_result.text(), &mut tokens);
    tokens
}

fn collect_tokens(node: Node, source: &str, tokens: &mut Vec<String>) {
    if node.child_count() == 0 {
        if let Ok(text) = node.utf8_text(source.as_bytes()) {
            tokens.push(text.to_string());
        }
    } else {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_tokens(child, source, tokens);
        }
    }
}

fn jaccard_similarity(tokens1: &[String], tokens2: &[String]) -> f64 {
    let set1: HashSet<_> = tokens1.iter().collect();
    let set2: HashSet<_> = tokens2.iter().collect();
    let intersection = set1.intersection(&set2).count();
    let union = set1.union(&set2).count();
    if union == 0 {
        1.0
    } else {
        intersection as f64 / union as f64
    }
}

#[allow(unused_variables)]
pub fn find_best_block_match(
    lines: &[&str],
    expected: &str,
    fuzz_radius: usize,
    similarity_threshold: f64,
) -> Result<BlockMatchResult> {
    let expected_lines: Vec<&str> = expected.lines().collect();
    if expected_lines.is_empty() {
        anyhow::bail!("expected block cannot be empty");
    }
    let expected_tokens = tokenize_rust(expected);
    let window_size = expected_lines.len();
    let search_start = 0usize;
    let search_end = lines.len().saturating_sub(window_size);
    let mut best_score = 0.0;
    let mut best_index = None;
    let mut tie_count = 0;
    for i in search_start..=search_end {
        let candidate = lines[i..i + window_size].join("\n");
        let candidate_tokens = tokenize_rust(&candidate);
        let score = jaccard_similarity(&expected_tokens, &candidate_tokens);
        if score > best_score {
            best_score = score;
            best_index = Some(i);
            tie_count = 1;
        } else if (score - best_score).abs() < f64::EPSILON {
            tie_count += 1;
        }
    }
    let idx = best_index.ok_or_else(|| anyhow!("no block match found"))?;
    if best_score < similarity_threshold {
        anyhow::bail!(
            "block match score {:.2} below threshold {:.2}",
            best_score,
            similarity_threshold
        );
    }
    if tie_count > 1 {
        anyhow::bail!(
            "ambiguous block match: {} candidates with score {:.2}",
            tie_count,
            best_score
        );
    }
    Ok(BlockMatchResult {
        start_index: idx,
        end_index: idx + window_size - 1,
        score: best_score,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniqueness_unique_line() {
        let lines = vec!["unique line", "other", "other"];
        assert_eq!(compute_uniqueness("unique line", &lines), 1.0);
    }

    #[test]
    fn test_uniqueness_common_line() {
        let lines = vec!["a", "a", "a"];
        assert!((compute_uniqueness("a", &lines) - 1.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_adjusted_threshold_raises() {
        let adjusted = adjusted_threshold(0.9, 0.33, 0.2);
        assert!(adjusted > 0.9);
    }

    #[test]
    fn test_adjusted_threshold_lowers() {
        let adjusted = adjusted_threshold(0.9, 1.0, 0.2);
        assert!((adjusted - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_ellipsis_match_simple() {
        let content = "fn main() {\n    let x = 1;\n    println!(\"{}\", x);\n}\n";
        let lines: Vec<&str> = content.lines().collect();
        let expected = "fn main() {\n...\n}";
        let result = cascade_match(&lines, 0, expected, 2, 0.9, 0.2, content).unwrap();
        assert_eq!(result.strategy, "ellipsis");
    }

    #[test]
    fn test_anchor_pair_finds_match() {
        let content = "// comment\nfn foo() {\n    let x = 1;\n}\nfn bar() {\n    let y = 2;\n}\n";
        let lines: Vec<&str> = content.lines().collect();
        let expected = "fn foo() {\n    let x = 1;\n}";
        let result = cascade_match(&lines, 0, expected, 3, 0.9, 0.2, content).unwrap();
        assert!(["anchor", "anchor_pair"].contains(&result.strategy.as_str()));
    }

    #[test]
    fn test_exact_match() {
        let content = "a\nb\nc\n";
        let lines: Vec<&str> = content.lines().collect();
        let result = cascade_match(&lines, 2, "b", 0, 0.9, 0.2, content).unwrap();
        assert_eq!(result.strategy, "exact");
    }

    #[test]
    fn test_no_match() {
        let content = "apple\nbanana\n";
        let lines: Vec<&str> = content.lines().collect();
        let result = cascade_match(&lines, 0, "orange", 2, 0.9, 0.2, content);
        assert!(result.is_err());
    }

    #[test]
    fn test_byte_range_is_correct() {
        let content = "line zero\nmatch me\nline two\n";
        let lines: Vec<&str> = content.lines().collect();
        let result = cascade_match(&lines, 2, "match me", 0, 0.9, 0.2, content).unwrap();
        // "match me" starts at byte 10 (after "line zero\n")
        assert_eq!(result.match_byte_range.0, 10);
        assert_eq!(result.match_byte_range.1, 18); // 10 + 8 characters
    }
}
