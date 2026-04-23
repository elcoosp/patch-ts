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

/// Multi‑strategy cascade match: exact → anchor → similarity → fuzzy.
/// Returns the first match whose confidence meets the threshold.
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
            index: target_idx,
            score: 1.0,
            confidence: 1.0,
            strategy: "exact".to_string(),
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
            });
        }
    }

    // 3. Similarity – token‑based Jaccard
    let expected_tokens: HashSet<&str> = expected.split_whitespace().collect();
    let mut best_conf = 0.0f64;
    let mut best_idx = start;
    for i in start..=end {
        let actual_tokens: HashSet<&str> = lines[i].split_whitespace().collect();
        let intersection = expected_tokens.intersection(&actual_tokens).count();
        let union = expected_tokens.union(&actual_tokens).count();
        let conf = if union == 0 { 0.0 } else { intersection as f64 / union as f64 };
        if conf > best_conf {
            best_conf = conf;
            best_idx = i;
        }
    }
    if best_conf >= similarity_threshold {
        return Ok(MatchResult {
            index: best_idx,
            score: best_conf,
            confidence: best_conf,
            strategy: "similarity".to_string(),
        });
    }

    // 4. Fuzzy – normalized Levenshtein
    let normalized_expected = normalize_line(expected);
    for i in start..=end {
        let normalized_actual = normalize_line(lines[i]);
        let score = normalized_levenshtein(&normalized_expected, &normalized_actual);
        if score >= similarity_threshold {
            return Ok(MatchResult {
                index: i,
                score,
                confidence: score,
                strategy: "fuzzy".to_string(),
            });
        }
    }

    anyhow::bail!(
        "no match found with any strategy (best similarity confidence was {:.2})",
        best_conf
    )
}

/// Legacy fuzzy match – now delegates to cascade for consistency.
pub fn fuzzy_match_line(
    lines: &[&str],
    target_line: usize,
    expected: &str,
    fuzz_radius: usize,
    similarity_threshold: f64,
) -> Result<MatchResult> {
    cascade_match(lines, target_line, expected, fuzz_radius, similarity_threshold)
}

// ---------------------------------------------------------------------------
// Block matching (unchanged from before)
// ---------------------------------------------------------------------------
use crate::ast::{Language, RustLanguage};
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct BlockMatchResult {
    pub start_index: usize,
    pub end_index: usize,
    pub score: f64,
}

/// Tokenize a string using tree-sitter Rust parser.
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

/// Compute Jaccard similarity between two token sets.
fn jaccard_similarity(tokens1: &[String], tokens2: &[String]) -> f64 {
    use std::collections::HashSet;
    let set1: HashSet<_> = tokens1.iter().collect();
    let set2: HashSet<_> = tokens2.iter().collect();
    let intersection = set1.intersection(&set2).count();
    let union = set1.union(&set2).count();
    if union == 0 { 1.0 } else { intersection as f64 / union as f64 }
}

/// Find best matching contiguous block of lines for expected multi-line content.
pub fn find_best_block_match(
    lines: &[&str],
    expected: &str,
    _fuzz_radius: usize,
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
    fn test_exact_match() {
        let lines = vec!["a", "b", "c"];
        let result = cascade_match(&lines, 2, "b", 0, 0.9).unwrap();
        assert_eq!(result.strategy, "exact");
        assert_eq!(result.confidence, 1.0);
    }

    #[test]
    fn test_anchor_match() {
        let lines = vec!["x", "b", "y"];
        let result = cascade_match(&lines, 1, "b", 2, 0.9).unwrap();
        assert_eq!(result.strategy, "anchor");
        assert_eq!(result.index, 1);
    }

    #[test]
    fn test_similarity_match() {
        let lines = vec!["hello world", "foo bar"];
        let result = cascade_match(&lines, 0, "hello world!", 2, 0.5).unwrap();
        // Either similarity or fuzzy can win; both are acceptable
        assert!(result.strategy == "similarity" || result.strategy == "fuzzy");
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_fuzzy_match_fallback() {
        let lines = vec!["hello wrld", "foo bar"];
        let result = cascade_match(&lines, 0, "hello world", 2, 0.8).unwrap();
        assert_eq!(result.strategy, "fuzzy");
    }

    #[test]
    fn test_no_match() {
        let lines = vec!["apple", "banana"];
        let result = cascade_match(&lines, 0, "orange", 2, 0.9);
        assert!(result.is_err());
    }
}
