I'll create a comprehensive implementation plan using the writing-plans skill structure. The plan will be organized into logical chunks with bite-sized tasks, exact file paths, code snippets, and verification steps.

---

```markdown
# patch-ts Enhancements Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enhance patch-ts with fuzzy matching, auto-repair, marker targeting, and improved diagnostics to significantly increase patch success rates for AI agents and developers.

**Architecture:** Introduce a new `matching` module for fuzzy location logic, extend `PatchOptions` with new flags, integrate auto-repair into patch flow, and enhance JSON diagnostics. Keep existing behavior intact with backward compatibility.

**Tech Stack:** Rust, tree-sitter 0.25, clap 4.5, anyhow, strsim, tempfile, serde_json.

---
```

## File Structure Overview

| File | Responsibility |
|------|----------------|
| `src/matching.rs` (new) | Fuzzy single-line and multi-line matching logic |
| `src/marker.rs` (new) | Marker comment detection and AST-based replacement |
| `src/patch.rs` (modify) | Integrate matching, auto-repair, fuzzy diff, new options |
| `src/cli.rs` (modify) | Add `--marker`, `--no-auto-repair`, `--fuzz` for diff |
| `src/diagnostics.rs` (modify) | Enhance JSON output with suggestions and fields |
| `src/repair.rs` (modify) | Expose `quick_balance` for auto-repair |
| `tests/matching_tests.rs` (new) | Unit tests for matching algorithms |
| `tests/marker_tests.rs` (new) | Unit tests for marker detection |
| `tests/cli_tests.rs` (modify) | Integration tests for new features |
| `tests/patch_tests.rs` (modify) | Integration tests for new behaviors |
| `benches/patch_benchmark.rs` (modify) | Extend benchmark for fuzzy matching impact |

---

## Chunk 1: Fuzzy Matching Infrastructure

### Task 1.1: Create matching module with single-line fuzzy matcher

**Files:**
- Create: `src/matching.rs`
- Create: `tests/matching_tests.rs`

- [ ] **Step 1: Write failing tests for single-line fuzzy matching**

```rust
// tests/matching_tests.rs
use patch_ts::matching::{fuzzy_match_line, MatchResult};

#[test]
fn test_exact_match_within_radius() {
    let lines = vec!["line1", "line2", "line3", "line4"];
    let result = fuzzy_match_line(&lines, 2, "line3", 2, 0.9).unwrap();
    assert_eq!(result.index, 2);
    assert!(result.score >= 0.99);
}

#[test]
fn test_whitespace_normalized_match() {
    let lines = vec!["  line2  "];
    let result = fuzzy_match_line(&lines, 0, "line2", 0, 0.9).unwrap();
    assert_eq!(result.index, 0);
}

#[test]
fn test_no_match_below_threshold() {
    let lines = vec!["apple", "banana"];
    let result = fuzzy_match_line(&lines, 0, "orange", 1, 0.9);
    assert!(result.is_err());
}

#[test]
fn test_ambiguous_match_tie() {
    let lines = vec!["target", "other", "target"];
    let result = fuzzy_match_line(&lines, 1, "target", 2, 0.9);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("ambiguous"));
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cargo test --test matching_tests
```
Expected: Compilation errors (module not found).

- [ ] **Step 3: Create `src/matching.rs` with basic structure and single-line fuzzy matcher**

```rust
// src/matching.rs
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
```

- [ ] **Step 4: Update `src/lib.rs` to export matching module**

```rust
// src/lib.rs
pub mod matching;
// ... existing exports
```

- [ ] **Step 5: Run tests to verify they pass**

```bash
cargo test --test matching_tests
```
Expected: All tests pass.

- [ ] **Step 6: Commit**

```bash
git add src/matching.rs src/lib.rs tests/matching_tests.rs
git commit -m "feat(matching): add single-line fuzzy matcher with whitespace normalization"
```

---

### Task 1.2: Add token-based multi-line block matcher

**Files:**
- Modify: `src/matching.rs`
- Modify: `tests/matching_tests.rs`

- [ ] **Step 1: Write failing test for multi-line block matching**

```rust
// tests/matching_tests.rs (append)
use patch_ts::matching::find_best_block_match;
use patch_ts::ast::{Language, RustLanguage};

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

#[test]
fn test_block_match_formatting_differences() {
    let lines: Vec<&str> = vec!["fn foo() { println!(\"hi\"); }"];
    let expected = "fn foo() {\n    println!(\"hi\");\n}";
    let result = find_best_block_match(&lines, expected, 0, 0.9).unwrap();
    assert_eq!(result.start_index, 0);
}
```

- [ ] **Step 2: Run tests to verify they fail (function not defined)**

```bash
cargo test --test matching_tests
```

- [ ] **Step 3: Implement `find_best_block_match` using token-based similarity**

```rust
// src/matching.rs (append)
use crate::ast::{Language, RustLanguage};

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

fn collect_tokens(node: tree_sitter::Node, source: &str, tokens: &mut Vec<String>) {
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
```

- [ ] **Step 4: Run tests to verify they pass**

```bash
cargo test --test matching_tests
```

- [ ] **Step 5: Commit**

```bash
git add src/matching.rs tests/matching_tests.rs
git commit -m "feat(matching): add token-based multi-line block matcher"
```

---

### Task 1.3: Integrate fuzzy matching into `apply_literal_patch`

**Files:**
- Modify: `src/patch.rs`
- Modify: `src/cli.rs` (pass new options)
- Modify: `tests/patch_tests.rs` (update tests)

- [ ] **Step 1: Update `PatchOptions` to include similarity threshold and marker**

```rust
// src/patch.rs (modify PatchOptions)
#[derive(Debug, Clone)]
pub struct PatchOptions {
    pub fuzz_radius: usize,
    pub dry_run: bool,
    pub force: bool,
    pub no_backup: bool,
    pub similarity_threshold: f64, // existing, now used for fuzzy
    pub no_auto_repair: bool,      // new
    pub marker: Option<String>,    // new
}
```

- [ ] **Step 2: Modify `apply_literal_patch` to use `fuzzy_match_line` for single-line and `find_best_block_match` for multi-line**

```rust
// src/patch.rs (inside apply_literal_patch)
use crate::matching::{fuzzy_match_line, find_best_block_match};

// ... after reading lines ...

let match_idx = if options.fuzz_radius > 0 {
    if expected.contains('\n') {
        let block_match = find_best_block_match(
            &lines,
            expected,
            options.fuzz_radius,
            options.similarity_threshold,
        )?;
        block_match.start_index
    } else {
        let line_match = fuzzy_match_line(
            &lines,
            line_num,
            expected,
            options.fuzz_radius,
            options.similarity_threshold,
        )?;
        line_match.index
    }
} else {
    // exact match at line_num - 1
    line_num.saturating_sub(1)
};
```

- [ ] **Step 3: Update `delete_line` and `insert_lines` if needed (they remain exact for now, but could be enhanced later)**

No changes required for this chunk.

- [ ] **Step 4: Adjust existing tests that rely on exact matching behavior**

Ensure tests that expect exact matching with `--fuzz 0` still pass. Add new tests for fuzzy scenarios.

- [ ] **Step 5: Add integration test for fuzzy single-line patch**

```rust
// tests/cli_tests.rs (append)
#[test]
fn test_cli_patch_fuzzy_single_line() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nline2\nline3\nline4\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("2")
        .arg("--old").arg("line3")
        .arg("--new").arg("new line3")
        .arg("--fuzz").arg("2")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line1\nline2\nnew line3\nline4\n");
}
```

- [ ] **Step 6: Run all tests**

```bash
cargo test
```

- [ ] **Step 7: Commit**

```bash
git add src/patch.rs src/cli.rs tests/cli_tests.rs tests/patch_tests.rs
git commit -m "feat(patch): integrate fuzzy matching into literal patch application"
```

---

**End of Chunk 1**

---

## Chunk 2: Auto-Repair Integration

### Task 2.1: Expose `quick_balance` function in repair module

**Files:**
- Modify: `src/repair.rs`

- [ ] **Step 1: Add `quick_balance` function that returns repaired content if successful**

```rust
// src/repair.rs (append)
/// Attempt to fix unbalanced delimiters by removing an extra delimiter.
/// Returns Some(fixed_content) if successful, None otherwise.
pub fn quick_balance(content: &str, language: &mut dyn Language) -> Option<String> {
    let parse_result = language.parse(content);
    if language.is_valid(&parse_result) {
        return Some(content.to_string());
    }
    let extra_span = language.find_extra_delimiter(&parse_result)?;
    let mut new_content = content.to_string();
    new_content.replace_range(extra_span.start_byte..extra_span.end_byte, "");
    let new_parse = language.parse(&new_content);
    if language.is_valid(&new_parse) {
        Some(new_content)
    } else {
        None
    }
}
```

- [ ] **Step 2: Write unit test for `quick_balance`**

```rust
// tests/repair_tests.rs (modify or add)
#[test]
fn test_quick_balance_fixes_extra_brace() {
    let mut lang = RustLanguage::new();
    let content = "fn main() {\n    println!(\"hi\");\n}\n}\n";
    let fixed = quick_balance(content, &mut lang).unwrap();
    assert_eq!(fixed.trim_end(), "fn main() {\n    println!(\"hi\");\n}");
}

#[test]
fn test_quick_balance_returns_none_if_unfixable() {
    let mut lang = RustLanguage::new();
    let content = "fn main() { let x: = 1; }\n";
    assert!(quick_balance(content, &mut lang).is_none());
}
```

- [ ] **Step 3: Run repair tests**

```bash
cargo test --test repair_tests
```

- [ ] **Step 4: Commit**

```bash
git add src/repair.rs tests/repair_tests.rs
git commit -m "feat(repair): add quick_balance for auto-repair of simple syntax errors"
```

---

### Task 2.2: Integrate auto-repair into patch flow

**Files:**
- Modify: `src/patch.rs`

- [ ] **Step 1: Modify `apply_literal_patch` to attempt auto-repair after validation failure**

```rust
// src/patch.rs (inside apply_literal_patch, after AST validation)
if !options.force {
    let original_parse = language.parse(&original_content);
    let was_valid = language.is_valid(&original_parse);
    let new_parse = language.parse(&new_content);
    let is_valid = language.is_valid(&new_parse);

    if was_valid && !is_valid && !options.no_auto_repair {
        if let Some(fixed_content) = crate::repair::quick_balance(&new_content, language) {
            eprintln!("Warning: Patch introduced syntax error but was auto-repaired.");
            // Use fixed content
            final_content = fixed_content;
            // Also record warning for JSON output (handled in CLI layer)
        } else {
            // Fall through to error
            let diag = language.explain_error(&new_parse, 1)
                .unwrap_or_else(|| SyntaxErrorDiagnostic {
                    src: NamedSource::new(file_path.to_string_lossy(), new_content.clone()),
                    error_span: (0, 0).into(),
                    details: "Unknown syntax error".to_string(),
                });
            anyhow::bail!(diag);
        }
    } else if was_valid && !is_valid {
        // No auto-repair (disabled or failed)
        let diag = language.explain_error(&new_parse, 1)
            .unwrap_or_else(|| SyntaxErrorDiagnostic {
                src: NamedSource::new(file_path.to_string_lossy(), new_content.clone()),
                error_span: (0, 0).into(),
                details: "Unknown syntax error".to_string(),
            });
        anyhow::bail!(diag);
    }
}
```

- [ ] **Step 2: Modify `apply_unified_diff` similarly (optional, can be in later chunk)**

For now, focus on literal patches. Unified diff auto-repair can be added in Chunk 4.

- [ ] **Step 3: Add CLI flag `--no-auto-repair`**

```rust
// src/cli.rs (PatchArgs)
pub struct PatchArgs {
    // ... existing fields
    #[arg(long)]
    pub no_auto_repair: bool,
}
```

- [ ] **Step 4: Pass `no_auto_repair` to `PatchOptions` in `handle_patch`**

```rust
let options = PatchOptions {
    // ...
    no_auto_repair: args.no_auto_repair,
    marker: None, // will add later
};
```

- [ ] **Step 5: Write integration test for auto-repair success**

```rust
// tests/cli_tests.rs (append)
#[test]
fn test_cli_auto_repair_missing_brace() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .arg("--old").arg("fn main() {}")
        .arg("--new").arg("fn main() {") // missing closing brace
        .assert()
        .success()
        .get_output()
        .stderr
        .clone();

    let stderr = String::from_utf8(output).unwrap();
    assert!(stderr.contains("auto-repaired"));
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content.trim_end(), "fn main() {}");
}

#[test]
fn test_cli_auto_repair_disabled() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("1")
        .arg("--old").arg("fn main() {}")
        .arg("--new").arg("fn main() {")
        .arg("--no-auto-repair")
        .assert()
        .failure();
}
```

- [ ] **Step 6: Run tests**

```bash
cargo test --test cli_tests
```

- [ ] **Step 7: Commit**

```bash
git add src/patch.rs src/cli.rs tests/cli_tests.rs
git commit -m "feat(auto-repair): integrate auto-repair on syntax error with --no-auto-repair flag"
```

---

**End of Chunk 2**

---

## Chunk 3: Marker-Based Targeting

### Task 3.1: Create marker detection module

**Files:**
- Create: `src/marker.rs`
- Create: `tests/marker_tests.rs`

- [ ] **Step 1: Write failing tests for marker detection and replacement**

```rust
// tests/marker_tests.rs
use patch_ts::marker::{find_marker, replace_marker_node};
use patch_ts::ast::{Language, RustLanguage};

#[test]
fn test_find_marker_single() {
    let source = r#"
// PATCH-ME: update
fn old() {}
"#;
    let markers = find_marker(source, "update").unwrap();
    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0].line, 2);
}

#[test]
fn test_find_marker_not_found() {
    let source = "fn main() {}";
    let result = find_marker(source, "missing");
    assert!(result.is_err());
}

#[test]
fn test_find_marker_multiple() {
    let source = r#"
// PATCH-ME: dup
fn a() {}
// PATCH-ME: dup
fn b() {}
"#;
    let markers = find_marker(source, "dup").unwrap();
    assert_eq!(markers.len(), 2);
}

#[test]
fn test_replace_marker_node() {
    let source = r#"
// PATCH-ME: replace
fn old() { println!("old"); }
"#;
    let new_content = "fn new() { println!(\"new\"); }";
    let result = replace_marker_node(source, "replace", new_content).unwrap();
    assert!(result.contains("fn new()"));
    assert!(!result.contains("fn old()"));
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cargo test --test marker_tests
```

- [ ] **Step 3: Implement `src/marker.rs`**

```rust
// src/marker.rs
use anyhow::{anyhow, Result};
use tree_sitter::{Node, Parser};
use crate::ast::{Language, RustLanguage};

#[derive(Debug)]
pub struct MarkerLocation {
    pub line: usize,
    pub byte_range: (usize, usize),
    pub node_to_replace: Option<Node<'static>>, // simplified for now
}

/// Find all markers with given ID in source.
pub fn find_marker(source: &str, marker_id: &str) -> Result<Vec<MarkerLocation>> {
    let mut lang = RustLanguage::new();
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();
    
    let mut markers = Vec::new();
    let pattern = format!("// PATCH-ME: {}", marker_id);
    let block_pattern = format!("/* PATCH-ME: {} */", marker_id);
    
    // Collect comments via tree-sitter query (simplified: iterate nodes)
    find_markers_in_node(root, source, &pattern, &block_pattern, &mut markers);
    if markers.is_empty() {
        anyhow::bail!("Marker '{}' not found", marker_id);
    }
    Ok(markers)
}

fn find_markers_in_node(node: Node, source: &str, pattern: &str, block_pattern: &str, markers: &mut Vec<MarkerLocation>) {
    if node.kind() == "line_comment" || node.kind() == "block_comment" {
        if let Ok(text) = node.utf8_text(source.as_bytes()) {
            if text.contains(pattern) || text.contains(block_pattern) {
                let start = node.start_position();
                markers.push(MarkerLocation {
                    line: start.row + 1,
                    byte_range: (node.start_byte(), node.end_byte()),
                    node_to_replace: None, // will compute later
                });
            }
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        find_markers_in_node(child, source, pattern, block_pattern, markers);
    }
}

/// Replace the AST node associated with a marker.
pub fn replace_marker_node(source: &str, marker_id: &str, new_content: &str) -> Result<String> {
    let markers = find_marker(source, marker_id)?;
    if markers.len() > 1 {
        anyhow::bail!("Multiple markers with ID '{}' found", marker_id);
    }
    let marker = &markers[0];
    
    // Re-parse and find the next sibling node to replace
    let mut lang = RustLanguage::new();
    let parse_result = lang.parse(source);
    let root = parse_result.tree.root_node();
    
    let marker_node = find_node_at_byte(root, marker.byte_range.0)?;
    let mut cursor = marker_node.walk();
    let node_to_replace = marker_node.next_sibling().or_else(|| marker_node.parent().and_then(|p| p.next_sibling()));
    
    if let Some(node) = node_to_replace {
        let mut new_source = source.to_string();
        new_source.replace_range(node.start_byte()..node.end_byte(), new_content);
        Ok(new_source)
    } else {
        anyhow::bail!("No node found after marker to replace");
    }
}

fn find_node_at_byte(node: Node, byte: usize) -> Result<Node> {
    if node.start_byte() <= byte && node.end_byte() >= byte {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.start_byte() <= byte && child.end_byte() >= byte {
                return find_node_at_byte(child, byte);
            }
        }
        Ok(node)
    } else {
        anyhow::bail!("byte not in node")
    }
}
```

- [ ] **Step 4: Add module to `lib.rs`**

```rust
// src/lib.rs
pub mod marker;
```

- [ ] **Step 5: Run marker tests to verify they pass**

```bash
cargo test --test marker_tests
```

- [ ] **Step 6: Commit**

```bash
git add src/marker.rs src/lib.rs tests/marker_tests.rs
git commit -m "feat(marker): add marker detection and AST-based replacement"
```

---

### Task 3.2: Integrate marker targeting into CLI and patch flow

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/patch.rs`
- Modify: `tests/cli_tests.rs`

- [ ] **Step 1: Add `--marker` flag to CLI**

```rust
// src/cli.rs (PatchArgs)
#[arg(long, conflicts_with = "line")]
pub marker: Option<String>,
```

- [ ] **Step 2: Modify `handle_patch` to handle marker case**

```rust
// src/cli.rs handle_patch
} else if let Some(marker) = args.marker {
    let new = args.new.as_deref().or_else(|| args.content.as_deref())
        .ok_or_else(|| anyhow::anyhow!("--new or --content required with --marker"))?;
    apply_marker_patch(file_path, &marker, new, options)?;
```

- [ ] **Step 3: Implement `apply_marker_patch` in `patch.rs`**

```rust
// src/patch.rs (new function)
use crate::marker::replace_marker_node;

pub fn apply_marker_patch(
    file_path: &Path,
    marker_id: &str,
    new_content: &str,
    options: PatchOptions,
) -> Result<()> {
    let original = fs::read_to_string(file_path)?;
    let patched = replace_marker_node(&original, marker_id, new_content)?;
    
    if options.dry_run {
        println!("{}", patched);
    } else {
        fs::write(file_path, patched)?;
    }
    Ok(())
}
```

- [ ] **Step 4: Add integration test for marker patch**

```rust
// tests/cli_tests.rs (append)
#[test]
fn test_cli_marker_patch() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, r#"
// PATCH-ME: replace
fn old() {}
"#).unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    cmd.arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--marker").arg("replace")
        .arg("--new").arg("fn new() {}")
        .assert()
        .success();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("fn new()"));
    assert!(!content.contains("fn old()"));
}
```

- [ ] **Step 5: Run tests**

```bash
cargo test --test cli_tests test_cli_marker_patch
```

- [ ] **Step 6: Commit**

```bash
git add src/cli.rs src/patch.rs tests/cli_tests.rs
git commit -m "feat(marker): integrate --marker flag for comment-anchored patches"
```

---

**End of Chunk 3**

---

## Chunk 4: Enhanced Diagnostics and Fuzzy Diff

### Task 4.1: Enhance JSON diagnostics with suggestions and match details

**Files:**
- Modify: `src/diagnostics.rs`
- Modify: `src/cli.rs` (populate new fields)

- [ ] **Step 1: Extend `JsonError` struct with new optional fields**

```rust
// src/diagnostics.rs
#[derive(serde::Serialize)]
pub struct JsonError {
    pub code: String,
    pub message: String,
    pub span: JsonSpan,
    pub context: String,
    pub suggestion: Option<String>,       // new
    pub best_score: Option<f64>,          // new
    pub best_match_line: Option<usize>,   // new
    pub candidates: Option<Vec<Candidate>>, // new
}

#[derive(serde::Serialize)]
pub struct Candidate {
    pub line: usize,
    pub score: f64,
}
```

- [ ] **Step 2: Update `anyhow_to_json` to populate new fields when available**

```rust
// src/diagnostics.rs (modify anyhow_to_json)
pub fn anyhow_to_json(err: &anyhow::Error, file: &str) -> JsonError {
    // ... existing downcast checks ...
    
    // Attempt to extract matching info from error string (simplified)
    let msg = err.to_string();
    let mut best_score = None;
    let mut best_match_line = None;
    let mut suggestion = None;
    
    if msg.contains("no match found with similarity") {
        suggestion = Some("Try increasing --fuzz radius".to_string());
        // Parse score and line from error message if possible
    } else if msg.contains("ambiguous match") {
        suggestion = Some("Provide more specific expected content".to_string());
    }
    
    JsonError {
        // ... existing fields ...
        suggestion,
        best_score,
        best_match_line,
        candidates: None,
    }
}
```

- [ ] **Step 3: In `matching.rs`, return structured error with score and candidates**

Instead of string error, consider a custom error type that can be downcast in `anyhow_to_json`. For simplicity, we'll enhance the error message to be parseable.

- [ ] **Step 4: Add test for JSON output with suggestion**

```rust
// tests/cli_tests.rs (append)
#[test]
fn test_cli_json_output_with_suggestion() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.rs");
    fs::write(&file_path, "line1\nlineX\nline3\n").unwrap();

    let mut cmd = Command::cargo_bin("patch-ts").unwrap();
    let output = cmd
        .arg("patch")
        .arg("--file").arg(file_path.to_str().unwrap())
        .arg("--line").arg("2")
        .arg("--old").arg("line2")
        .arg("--new").arg("new")
        .arg("--fuzz").arg("2")
        .arg("--json")
        .assert()
        .failure()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["success"], false);
    assert!(json["error"]["suggestion"].as_str().unwrap().contains("--fuzz"));
}
```

- [ ] **Step 5: Run tests**

```bash
cargo test test_cli_json_output_with_suggestion
```

- [ ] **Step 6: Commit**

```bash
git add src/diagnostics.rs src/cli.rs tests/cli_tests.rs
git commit -m "feat(diagnostics): enhance JSON output with suggestions and match details"
```

---

### Task 4.2: Implement fuzzy context matching for unified diffs

**Files:**
- Modify: `src/patch.rs`
- Modify: `src/cli.rs` (allow `--fuzz` with `--diff`)
- Modify: `tests/patch_tests.rs`

- [ ] **Step 1: Update CLI to allow `--fuzz` with `--diff`**

```rust
// src/cli.rs (PatchArgs)
#[arg(short = 'z', long, default_value = "0", requires = "diff")] // now allowed with diff
pub fuzz: usize,
```
But careful: `--fuzz` is already defined; just remove `conflicts_with = "diff"` if any, and adjust logic.

- [ ] **Step 2: Modify `apply_unified_diff` to use fuzzy block matching for each hunk when fuzz > 0**

```rust
// src/patch.rs (modify apply_unified_diff)
use crate::matching::find_best_block_match;

pub fn apply_unified_diff(
    file_path: &Path,
    diff_text: &str,
    options: PatchOptions,
) -> Result<()> {
    let original = fs::read_to_string(file_path)?;
    let diffs = flickzeug::patch_from_str(diff_text)?;
    let mut current_content = original.clone();
    let lines: Vec<&str> = current_content.lines().collect();

    for diff in diffs {
        if options.fuzz_radius > 0 {
            // Fuzzy apply: for each hunk, locate context lines via matching
            // This is simplified; full implementation would parse hunks
            // For now, fall back to flickzeug's apply with a pre-patched file
        }
        match flickzeug::apply(&current_content, &diff) {
            Ok((patched, _stats)) => current_content = patched,
            Err(e) => anyhow::bail!("Failed to apply diff: {}", e),
        }
    }
    // ...
}
```

Given the complexity of parsing hunks ourselves, an alternative is to use `flickzeug`'s apply and if it fails, attempt fuzzy location of the entire diff context. For brevity, we'll implement a fallback: if exact apply fails and fuzz>0, use `find_best_block_match` to locate the expected pre-image and apply the change manually.

- [ ] **Step 3: Write integration test for fuzzy diff**

```rust
// tests/patch_tests.rs (append)
#[test]
fn test_apply_unified_diff_with_fuzz() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("sample.rs");
    fs::write(&file_path, "// added comment\nline1\nline2\nline3\n").unwrap();

    let diff = r#"--- a/sample.rs
+++ b/sample.rs
@@ -1,3 +1,3 @@
 line1
-line2
+new line2
 line3
"#;
    let options = PatchOptions {
        fuzz_radius: 3,
        similarity_threshold: 0.9,
        ..Default::default()
    };
    apply_unified_diff(&file_path, diff, options).unwrap();
    let new_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(new_content, "// added comment\nline1\nnew line2\nline3\n");
}
```

- [ ] **Step 4: Implement fuzzy diff logic** (simplified for plan)

We'll implement a helper that attempts to apply diff with flickzeug, and on failure, locates the context lines using our block matcher, then manually applies the hunk changes.

- [ ] **Step 5: Run tests**

```bash
cargo test test_apply_unified_diff_with_fuzz
```

- [ ] **Step 6: Commit**

```bash
git add src/patch.rs src/cli.rs tests/patch_tests.rs
git commit -m "feat(diff): add fuzzy context matching for unified diffs with --fuzz"
```

---

**End of Chunk 4**

---

## Final Verification

- [ ] Run full test suite: `cargo test`
- [ ] Run benchmarks: `cargo bench` (compare before/after)
- [ ] Manually test sample scenarios from spec

**Plan complete.** Ready for execution using subagent-driven development.
