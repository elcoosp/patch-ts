use anyhow::{anyhow, Result};
use std::collections::HashSet;
use strsim::normalized_levenshtein;

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub index: usize,
    pub score: f64,
    pub confidence: f64,
    pub strategy: String,
}

/// Normalize a line for comparison: trim whitespace and collapse multiple spaces.
fn normalize_line(s: &str) -> String {
    s.trim().split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Try to match when `...` appears as a line in the expected block.
/// `...` acts as a wildcard that matches zero or more lines between the
/// surrounding explicit lines.
fn try_ellipsis_match(
    lines: &[&str],
    expected: &str,
    fuzz_radius: usize,
) -> Option<MatchResult> {
    let expected_lines: Vec<&str> = expected.lines().collect();
    // Find `...` on a line by itself
    let ellipsis_pos = expected_lines.iter().position(|l| l.trim() == "...")?;
    let before: Vec<&str> = expected_lines[..ellipsis_pos].to_vec();
    let after: Vec<&str> = expected_lines[ellipsis_pos + 1..].to_vec();

    let target_idx = 0usize; // We'll search from the top
    let start = target_idx.saturating_sub(fuzz_radius);
    let end = (target_idx + fuzz_radius).min(lines.len().saturating_sub(1));

    // Find the first line that matches the "before" block's first line
    for i in start..=end {
        let mut matches = true;
        // Match all "before" lines starting at i
        if i + before.len() > lines.len() { continue; }
        for (j, bline) in before.iter().enumerate() {
            if lines[i + j] != *bline { matches = false; break; }
        }
        if !matches { continue; }
        // Now find where the "after" block matches after some gap
        let after_start = i + before.len();
        for gap in 0..(lines.len() - after_start) {
            let aj = after_start + gap;
            if aj + after.len() > lines.len() { break; }
            let mut after_matches = true;
            for (k, aline) in after.iter().enumerate() {
                if lines[aj + k] != *aline { after_matches = false; break; }
            }
            if after_matches {
                return Some(MatchResult {
                    index: i,
                    score: 1.0,
                    confidence: 0.95,
                    strategy: "ellipsis".to_string(),
                });
            }
        }
    }
    None
}

/// Multi‑strategy cascade match: exact → anchor → ellipsis → similarity → fuzzy.
pub fn cascade_match(
    lines: &[&str],
    target_line: usize,
    expected: &str,
    fuzz_radius: usize,
    similarity_threshold: f64,
) -> Result<MatchResult> {
    let target_idx = target_line.saturating_sub(1);
    let start = target_idx.saturating_sub(fuzz_radius);
    let end = (target_idx + fuzz_radius).min(lines.len().saturating_sub(1));

    if start > end {
        anyhow::bail!("empty search range");
    }

    // 1. Exact match at target line
    if target_idx < lines.len() && lines[target_idx] == expected {
        return Ok(MatchResult {
            index: target_idx, score: 1.0, confidence: 1.0,
            strategy: "exact".to_string(),
        });
    }

    // 2. Anchor‑based: any line in range matches exactly
    for i in start..=end {
        if lines[i] == expected {
            return Ok(MatchResult {
                index: i, score: 1.0, confidence: 1.0,
                strategy: "anchor".to_string(),
            });
        }
    }

    // 3. Ellipsis pattern (if applicable)
    if expected.contains("\n...\n") || expected.lines().any(|l| l.trim() == "...") {
        if let Some(m) = try_ellipsis_match(lines, expected, fuzz_radius) {
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
        let conf = if union == 0 { 0.0 } else { intersection as f64 / union as f64 };
        if conf > best_conf { best_conf = conf; best_idx = i; }
    }
    if best_conf >= similarity_threshold {
        return Ok(MatchResult {
            index: best_idx, score: best_conf, confidence: best_conf,
            strategy: "similarity".to_string(),
        });
    }

    // 5. Fuzzy – normalized Levenshtein
    let normalized_expected = normalize_line(expected);
    for i in start..=end {
        let normalized_actual = normalize_line(lines[i]);
        let score = normalized_levenshtein(&normalized_expected, &normalized_actual);
        if score >= similarity_threshold {
            return Ok(MatchResult {
                index: i, score, confidence: score,
                strategy: "fuzzy".to_string(),
            });
        }
    }

    anyhow::bail!("no match found with any strategy (best similarity confidence was {:.2})", best_conf)
}

pub fn fuzzy_match_line(
    lines: &[&str],
    target_line: usize,
    expected: &str,
    fuzz_radius: usize,
    similarity_threshold: f64,
) -> Result<MatchResult> {
    cascade_match(lines, target_line, expected, fuzz_radius, similarity_threshold)
}

// Block matching and tests remain below (same as before)
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
    if union == 0 { 1.0 } else { intersection as f64 / union as f64 }
}

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
            best_score = score; best_index = Some(i); tie_count = 1;
        } else if (score - best_score).abs() < f64::EPSILON {
            tie_count += 1;
        }
    }
    let idx = best_index.ok_or_else(|| anyhow!("no block match found"))?;
    if best_score < similarity_threshold {
        anyhow::bail!("block match score {:.2} below threshold {:.2}", best_score, similarity_threshold);
    }
    if tie_count > 1 {
        anyhow::bail!("ambiguous block match: {} candidates with score {:.2}", tie_count, best_score);
    }
    Ok(BlockMatchResult { start_index: idx, end_index: idx + window_size - 1, score: best_score })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipsis_match_simple() {
        let lines = vec!["fn main() {", "    let x = 1;", "    println!(\"{}\", x);", "}"];
        let expected = "fn main() {\n...\n}";
        let result = cascade_match(&lines, 0, expected, 2, 0.9).unwrap();
        assert_eq!(result.strategy, "ellipsis");
        assert_eq!(result.confidence, 0.95);
    }

    #[test]
    fn test_exact_match() {
        let lines = vec!["a", "b", "c"];
        let result = cascade_match(&lines, 2, "b", 0, 0.9).unwrap();
        assert_eq!(result.strategy, "exact");
    }

    #[test]
    fn test_anchor_match() {
        let lines = vec!["x", "b", "y"];
        let result = cascade_match(&lines, 1, "b", 2, 0.9).unwrap();
        assert_eq!(result.strategy, "anchor");
    }

    #[test]
    fn test_no_match() {
        let lines = vec!["apple", "banana"];
        let result = cascade_match(&lines, 0, "orange", 2, 0.9);
        assert!(result.is_err());
    }
}
