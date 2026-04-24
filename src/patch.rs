use crate::ast::Language;
use crate::diagnostics::SyntaxErrorDiagnostic;
use crate::matching::{cascade_match, find_best_block_match, MatchResult};
use anyhow::{Context, Result};
use miette::NamedSource;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PatchOptions {
    pub fuzz_radius: usize,
    pub dry_run: bool,
    pub force: bool,
    pub no_backup: bool,
    pub similarity_threshold: f64,
    pub confidence_threshold: f64,
    pub uniqueness_weight: f64,
    pub strict_whitespace: bool,
    pub no_auto_repair: bool,
    pub marker: Option<String>,
    pub plugin: Option<String>,
    pub match_info: Option<MatchResult>,
    pub fix_indent: bool,
}

impl Default for PatchOptions {
    fn default() -> Self {
        Self {
            fuzz_radius: 0,
            dry_run: false,
            force: false,
            no_backup: false,
            similarity_threshold: 0.9,
            confidence_threshold: 0.9,
            uniqueness_weight: 0.2,
            strict_whitespace: false,
            no_auto_repair: false,
            marker: None,
            plugin: None,
            match_info: None,
            fix_indent: false,
        }
    }
}

pub fn apply_literal_patch(
    file_path: &Path,
    line_num: usize,
    expected: &str,
    new: &str,
    options: &mut PatchOptions,
    language: &mut dyn Language,
) -> Result<()> {
    let original_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read {}", file_path.display()))?;
    let lines: Vec<&str> = original_content.lines().collect();

    if lines.is_empty() {
        anyhow::bail!("file is empty, cannot apply patch at line {}", line_num);
    }
    if line_num > lines.len() {
        anyhow::bail!(
            "line {} out of range (file has {} lines)",
            line_num,
            lines.len()
        );
    }

    let target_idx = line_num.saturating_sub(1);

    let (match_idx, match_info, matched_line) = if options.fuzz_radius > 0 {
        if expected.contains('\n') {
            let block_match = find_best_block_match(
                &lines,
                expected,
                options.fuzz_radius,
                options.similarity_threshold,
            )?;
            (block_match.start_index, None, None)
        } else {
            let line_match = cascade_match(
                &lines,
                line_num,
                expected,
                options.fuzz_radius,
                options.confidence_threshold,
                options.uniqueness_weight,
            )?;
            let matched = lines[line_match.index].to_string();
            let info = Some(line_match.clone());
            (line_match.index, info, Some(matched))
        }
    } else {
        (target_idx, None, None)
    };
    options.match_info = match_info;

    if options.fuzz_radius == 0 {
        if lines[match_idx] != expected {
            anyhow::bail!(
                "expected line {} to contain '{}', but found '{}'",
                line_num,
                expected,
                lines[match_idx]
            );
        }
    }

    let new_to_use = if options.fix_indent {
        if let Some(ref matched) = matched_line {
            let (uses_tabs, count) = crate::indent::detect_indent(matched);
            crate::indent::apply_indent(new, uses_tabs, count)
        } else {
            new.to_string()
        }
    } else {
        new.to_string()
    };

    let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    new_lines[match_idx] = new_to_use;
    let mut new_content = new_lines.join("\n")
        + if original_content.ends_with('\n') {
            "\n"
        } else {
            ""
        };

    if !options.force {
        let original_parse = language.parse(&original_content);
        let was_valid = language.is_valid(&original_parse);
        let new_parse = language.parse(&new_content);
        let is_valid = language.is_valid(&new_parse);
        if was_valid && !is_valid {
            if !options.no_auto_repair {
                if let Some(fixed_content) = crate::repair::quick_balance(&new_content, language) {
                    eprintln!("Warning: Patch introduced syntax error but was auto-repaired.");
                    new_content = fixed_content;
                } else {
                    let diag = language.explain_error(&new_parse, 1).unwrap_or_else(|| {
                        SyntaxErrorDiagnostic {
                            src: NamedSource::new(file_path.to_string_lossy(), new_content.clone()),
                            error_span: (0, 0).into(),
                            details: "Unknown syntax error".to_string(),
                        }
                    });
                    anyhow::bail!(diag);
                }
            } else {
                let diag = language.explain_error(&new_parse, 1).unwrap_or_else(|| {
                    SyntaxErrorDiagnostic {
                        src: NamedSource::new(file_path.to_string_lossy(), new_content.clone()),
                        error_span: (0, 0).into(),
                        details: "Unknown syntax error".to_string(),
                    }
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

pub fn insert_lines(
    file_path: &Path,
    after_line_num: usize,
    content_to_insert: &str,
    options: PatchOptions,
) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();
    if after_line_num > lines.len() {
        anyhow::bail!(
            "line {} out of range (file has {} lines)",
            after_line_num,
            lines.len()
        );
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

/// Strip leading whitespace from all context lines in a unified diff.
pub fn normalize_diff_whitespace(diff_text: &str) -> String {
    let mut result = String::new();
    for line in diff_text.lines() {
        if line.starts_with(' ') || line.starts_with('-') || line.starts_with('+') {
            let control = &line[0..1];
            let content = line[1..].trim_start();
            result.push_str(&format!("{}{}\n", control, content));
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

pub fn apply_unified_diff(file_path: &Path, diff_text: &str, options: PatchOptions) -> Result<()> {
    let original = fs::read_to_string(file_path)?;
    let text = if options.strict_whitespace {
        diff_text.to_string()
    } else {
        normalize_diff_whitespace(diff_text)
    };
    let diffs = flickzeug::patch_from_str(&text)
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

pub fn apply_symbol_patch(
    file_path: &Path,
    symbol: &str,
    new: &str,
    options: &PatchOptions,
    language: &mut dyn Language,
) -> Result<()> {
    let original = std::fs::read_to_string(file_path)?;
    let parse_result = language.parse(&original);
    let (start, end) = language.find_symbol_node(&parse_result, symbol)
        .ok_or_else(|| anyhow::anyhow!("Symbol {} not found", symbol))?;
    let mut patched = original.clone();
    patched.replace_range(start..end, new);
    if !options.dry_run {
        std::fs::write(file_path, &patched)?;
    } else {
        println!("{}", patched);
    }
    Ok(())
}

/// Replace all occurrences of old_text with new_text in the file.
/// Used for file types that tree‑sitter cannot parse.
pub fn full_file_replace(file_path: &Path, old_text: &str, new_text: &str, dry_run: bool) -> Result<()> {
    let original = std::fs::read_to_string(file_path)?;
    let patched = original.replace(old_text, new_text);
    if dry_run {
        println!("{}", patched);
    } else {
        std::fs::write(file_path, patched)?;
    }
    Ok(())
}
