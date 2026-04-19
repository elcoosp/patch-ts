use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use strsim::normalized_levenshtein;

use crate::ast::Language;
use crate::diagnostics::SyntaxErrorDiagnostic;
use miette::NamedSource;

#[derive(Debug, Clone)]
pub struct PatchOptions {
    pub fuzz_radius: usize,
    pub dry_run: bool,
    pub force: bool,
    pub no_backup: bool,
    pub similarity_threshold: f64,
}

impl Default for PatchOptions {
    fn default() -> Self {
        Self {
            fuzz_radius: 0,
            dry_run: false,
            force: false,
            no_backup: false,
            similarity_threshold: 0.9,
        }
    }
}

/// Apply a literal replacement patch (exact or fuzzy) with AST validation
pub fn apply_literal_patch(
    file_path: &Path,
    line_num: usize,
    expected: &str,
    new: &str,
    options: PatchOptions,
    language: &mut dyn Language,
) -> Result<()> {
    let original_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read {}", file_path.display()))?;
    let lines: Vec<&str> = original_content.lines().collect();

    if lines.is_empty() {
        anyhow::bail!("file is empty, cannot apply patch at line {}", line_num);
    }

    let target_idx = line_num.saturating_sub(1);

    let start = if options.fuzz_radius > 0 {
        target_idx.saturating_sub(options.fuzz_radius)
    } else {
        target_idx
    };
    let end = if options.fuzz_radius > 0 {
        (target_idx + options.fuzz_radius).min(lines.len().saturating_sub(1))
    } else {
        target_idx
    };
    let end = end.min(lines.len().saturating_sub(1));

    let mut best_match = None;
    let mut best_score = 0.0;
    for i in start..=end {
        let actual = lines[i];
        let score = normalized_levenshtein(expected, actual);
        if score > best_score {
            best_score = score;
            best_match = Some(i);
        }
    }

    let match_idx = best_match.ok_or_else(|| anyhow::anyhow!("no lines in range"))?;
    if best_score < options.similarity_threshold {
        anyhow::bail!(
            "no match found with similarity >= {} (best was {:.2} at line {})",
            options.similarity_threshold,
            best_score,
            match_idx + 1
        );
    }

    let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    new_lines[match_idx] = new.to_string();
    let new_content = new_lines.join("\n") + if original_content.ends_with('\n') { "\n" } else { "" };

    // AST validation (unless --force)
    if !options.force {
        let original_parse = language.parse(&original_content);
        let was_valid = language.is_valid(&original_parse);

        let new_parse = language.parse(&new_content);
        let is_valid = language.is_valid(&new_parse);

        if was_valid && !is_valid {
            let diag = language.explain_error(&new_parse, 1)
                .unwrap_or_else(|| SyntaxErrorDiagnostic {
                    src: NamedSource::new(file_path.to_string_lossy(), new_content.clone()),
                    error_span: (0, 0).into(),
                    details: "Unknown syntax error".to_string(),
                });
            anyhow::bail!(diag);
        }
    }

    if options.dry_run {
        println!("{}", new_content);
    } else {
        fs::write(file_path, new_content)?;
    }
    Ok(())
}

/// Delete a line after verifying its content
pub fn delete_line(
    file_path: &Path,
    line_num: usize,
    expected: &str,
    options: PatchOptions,
) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();
    if line_num == 0 || line_num > lines.len() {
        anyhow::bail!("line {} out of range", line_num);
    }
    let actual = lines[line_num - 1].trim();
    let expected_trimmed = expected.trim();
    if actual != expected_trimmed {
        anyhow::bail!(
            "expected line {} to contain '{}', but found '{}'",
            line_num,
            expected_trimmed,
            actual
        );
    }
    let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    new_lines.remove(line_num - 1);
    let new_content = new_lines.join("\n") + if content.ends_with('\n') { "\n" } else { "" };

    if options.dry_run {
        println!("{}", new_content);
    } else {
        fs::write(file_path, new_content)?;
    }
    Ok(())
}

/// Insert lines after a given line number
pub fn insert_lines(
    file_path: &Path,
    after_line_num: usize,
    content_to_insert: &str,
    options: PatchOptions,
) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();
    if after_line_num > lines.len() {
        anyhow::bail!("line {} out of range (file has {} lines)", after_line_num, lines.len());
    }
    let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    let insert_idx = after_line_num;
    for line in content_to_insert.lines().rev() {
        new_lines.insert(insert_idx, line.to_string());
    }
    let new_content = new_lines.join("\n") + if content.ends_with('\n') { "\n" } else { "" };

    if options.dry_run {
        println!("{}", new_content);
    } else {
        fs::write(file_path, new_content)?;
    }
    Ok(())
}

/// Apply a unified diff using the `flickzeug` crate
pub fn apply_unified_diff(
    file_path: &Path,
    diff_text: &str,
    options: PatchOptions,
) -> Result<()> {
    let original = fs::read_to_string(file_path)?;
    let diffs = flickzeug::patch_from_str(diff_text)
        .map_err(|e| anyhow::anyhow!("Failed to parse diff: {}", e))?;

    let mut current_content = original;
    for diff in diffs {
        match flickzeug::apply(&current_content, &diff) {
            Ok((patched, _stats)) => {
                current_content = patched;
            }
            Err(e) => {
                anyhow::bail!("Failed to apply diff: {}", e);
            }
        }
    }

    if options.dry_run {
        println!("{}", current_content);
    } else {
        fs::write(file_path, current_content)?;
    }
    Ok(())
}
