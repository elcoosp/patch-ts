use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use crate::matching::{fuzzy_match_line, find_best_block_match};

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
    pub no_auto_repair: bool,
    pub marker: Option<String>,
}

impl Default for PatchOptions {
    fn default() -> Self {
        Self {
            fuzz_radius: 0,
            dry_run: false,
            force: false,
            no_backup: false,
            similarity_threshold: 0.9,
            no_auto_repair: false,
            marker: None,
        }
    }
}

/// Apply a literal replacement patch (exact or fuzzy) with AST validation and optional auto-repair
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
        target_idx
    };

    let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    new_lines[match_idx] = new.to_string();
    let mut new_content = new_lines.join("\n") + if original_content.ends_with('\n') { "\n" } else { "" };

    // AST validation (unless --force)
    if !options.force {
        let original_parse = language.parse(&original_content);
        let was_valid = language.is_valid(&original_parse);

        let new_parse = language.parse(&new_content);
        let is_valid = language.is_valid(&new_parse);

        if was_valid && !is_valid {
            // Attempt auto-repair if enabled
            if !options.no_auto_repair {
                if let Some(fixed_content) = crate::repair::quick_balance(&new_content, language) {
                    eprintln!("Warning: Patch introduced syntax error but was auto-repaired.");
                    new_content = fixed_content;
                } else {
                    // Auto-repair failed, bail with diagnostic
                    let diag = language.explain_error(&new_parse, 1)
                        .unwrap_or_else(|| SyntaxErrorDiagnostic {
                            src: NamedSource::new(file_path.to_string_lossy(), new_content.clone()),
                            error_span: (0, 0).into(),
                            details: "Unknown syntax error".to_string(),
                        });
                    anyhow::bail!(diag);
                }
            } else {
                // Auto-repair disabled, fail with error
                let diag = language.explain_error(&new_parse, 1)
                    .unwrap_or_else(|| SyntaxErrorDiagnostic {
                        src: NamedSource::new(file_path.to_string_lossy(), new_content.clone()),
                        error_span: (0, 0).into(),
                        details: "Unknown syntax error".to_string(),
                    });
                anyhow::bail!(diag);
            }
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

use crate::marker::replace_marker_node;

/// Apply a patch targeted by a marker comment.
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
