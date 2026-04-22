use anyhow::{anyhow, Result};
use strsim::normalized_levenshtein;

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub index: usize,
    pub score: f64,
}

/// Normalize a line for comparison: trim whitespace and collapse multiple spaces.
fn normalize_line(s: &str) -> String {
    s.trim().split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Fuzzy match a single expected line within a radius around target_line.
/// Returns best match index and score if above threshold and unambiguous.
pub fn fuzzy_match_line(
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

    let normalized_expected = normalize_line(expected);
    let mut best_match: Option<MatchResult> = None;
    let mut best_score = 0.0;
    let mut tie_count = 0;

    for i in start..=end {
        let normalized_actual = normalize_line(lines[i]);
        let score = normalized_levenshtein(&normalized_expected, &normalized_actual);
        if score > best_score {
            best_score = score;
            best_match = Some(MatchResult { index: i, score });
            tie_count = 1;
        } else if (score - best_score).abs() < f64::EPSILON {
            tie_count += 1;
        }
    }

    let best = best_match.ok_or_else(|| anyhow!("no lines in search range"))?;
    if best_score < similarity_threshold {
        anyhow::bail!(
            "no match found with similarity >= {} (best was {:.2} at line {})",
            similarity_threshold,
            best_score,
            best.index + 1
        );
    }
    if tie_count > 1 {
        anyhow::bail!(
            "ambiguous match: {} candidates with score {:.2}",
            tie_count,
            best_score
        );
    }
    Ok(best)
}

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
    if union == 0 {
        1.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Find best matching contiguous block of lines for expected multi-line content.
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
        anyhow::bail!("ambiguous block match: {} candidates with score {:.2}", tie_count, best_score);
    }
    Ok(BlockMatchResult {
        start_index: idx,
        end_index: idx + window_size - 1,
        score: best_score,
    })
}
