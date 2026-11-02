use crate::ast::Language;
use crate::diagnostics::SyntaxErrorDiagnostic;
use crate::marker::replace_marker_node;
use crate::matching::{MatchResult, cascade_match, find_best_block_match};
use anyhow::{Context, Result};
use miette::NamedSource;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

/// Execution-time toggles for applying a patch.
#[derive(Debug, Clone, Default)]
pub struct RuntimeFlags {
    pub dry_run: bool,
    pub force: bool,
    pub no_backup: bool,
}

/// Matching-behaviour toggles for applying a patch.
#[derive(Debug, Clone, Default)]
pub struct MatchFlags {
    pub strict_whitespace: bool,
    pub no_auto_repair: bool,
    pub fix_indent: bool,
}

#[derive(Debug, Clone)]
pub struct PatchOptions {
    pub fuzz_radius: usize,
    pub similarity_threshold: f64,
    pub confidence_threshold: f64,
    pub uniqueness_weight: f64,
    pub marker: Option<String>,
    pub plugin: Option<String>,
    pub match_info: Option<MatchResult>,
    pub run: RuntimeFlags,
    pub match_behavior: MatchFlags,
}

impl Default for PatchOptions {
    fn default() -> Self {
        Self {
            fuzz_radius: 0,
            similarity_threshold: 0.9,
            confidence_threshold: 0.9,
            uniqueness_weight: 0.2,
            marker: None,
            plugin: None,
            match_info: None,
            run: RuntimeFlags::default(),
            match_behavior: MatchFlags::default(),
        }
    }
}

///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
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
        anyhow::bail!("file is empty, cannot apply patch at line {line_num}");
    }
    if line_num > lines.len() {
        anyhow::bail!(
            "line {} out of range (file has {} lines)",
            line_num,
            lines.len()
        );
    }

    let target_idx = line_num.saturating_sub(1);
    let (match_idx, matched_line) = locate_match_target(
        &lines,
        line_num,
        expected,
        target_idx,
        options,
        &original_content,
    )?;

    if options.fuzz_radius == 0 && lines[match_idx] != expected {
        anyhow::bail!(
            "expected line {} to contain '{}', but found '{}'",
            line_num,
            expected,
            lines[match_idx]
        );
    }

    let new_to_use = if options.match_behavior.fix_indent {
        if let Some(ref matched) = matched_line {
            let (uses_tabs, count) = crate::indent::detect_indent(matched);
            crate::indent::apply_indent(new, uses_tabs, count)
        } else {
            new.to_string()
        }
    } else {
        new.to_string()
    };

    // A multi-line `expected` matches a window of `expected.lines().count()`
    // lines (see find_best_block_match); the replacement must consume the
    // whole window, not just the first line of it.
    let matched_span = if expected.contains('\n') {
        expected.lines().count()
    } else {
        1
    };
    let mut new_lines: Vec<String> = Vec::with_capacity(lines.len() + matched_span);
    new_lines.extend(
        lines[..match_idx]
            .iter()
            .map(std::string::ToString::to_string),
    );
    new_lines.extend(new_to_use.lines().map(std::string::ToString::to_string));
    new_lines.extend(
        lines[(match_idx + matched_span).min(lines.len())..]
            .iter()
            .map(std::string::ToString::to_string),
    );
    let mut new_content = new_lines.join("\n")
        + if original_content.ends_with('\n') {
            "\n"
        } else {
            ""
        };

    if !options.run.force {
        new_content =
            enforce_syntax_gate(&original_content, new_content, file_path, options, language)?;
    }

    if options.run.dry_run {
        println!("{new_content}");
    } else {
        fs::write(file_path, new_content)?;
    }
    Ok(())
}

/// Locate the target of the patch: fuzzy block/line matching when allowed,
/// otherwise the exact line.
///
/// # Errors
///
/// Returns an error if no sufficiently similar match can be found.
fn locate_match_target(
    lines: &[&str],
    line_num: usize,
    expected: &str,
    target_idx: usize,
    options: &mut PatchOptions,
    original_content: &str,
) -> Result<(usize, Option<String>)> {
    if options.fuzz_radius == 0 {
        if lines[target_idx] != expected {
            anyhow::bail!(
                "expected line {} to contain '{}', but found '{}'",
                line_num,
                expected,
                lines[target_idx]
            );
        }
        return Ok((target_idx, None));
    }
    if expected.contains('\n') {
        let block_match = find_best_block_match(
            lines,
            expected,
            options.fuzz_radius,
            options.similarity_threshold,
        )?;
        Ok((block_match.start_index, None))
    } else {
        let line_match = cascade_match(
            lines,
            line_num,
            expected,
            options.fuzz_radius,
            options.confidence_threshold,
            options.uniqueness_weight,
            original_content,
        )?;
        options.match_info = Some(line_match.clone());
        Ok((line_match.index, Some(lines[line_match.index].to_string())))
    }
}

/// Reject (or auto-repair) patches that turn valid syntax into invalid syntax.
///
/// # Errors
///
/// Returns an error with a syntax diagnostic when the patch cannot be repaired.
fn enforce_syntax_gate(
    original_content: &str,
    mut new_content: String,
    file_path: &Path,
    options: &PatchOptions,
    language: &mut dyn Language,
) -> Result<String> {
    let original_parse = language.parse(original_content);
    let was_valid = language.is_valid(&original_parse);
    let new_parse = language.parse(&new_content);
    let is_valid = language.is_valid(&new_parse);
    if was_valid && !is_valid {
        if options.match_behavior.no_auto_repair {
            return Err(syntax_error_diag(&new_content, file_path, language, &new_parse).into());
        }
        if let Some(fixed_content) = crate::repair::quick_balance(&new_content, language) {
            eprintln!("Warning: Patch introduced syntax error but was auto-repaired.");
            new_content = fixed_content;
        } else {
            return Err(syntax_error_diag(&new_content, file_path, language, &new_parse).into());
        }
    }
    Ok(new_content)
}

/// Build a rich syntax-error diagnostic for the patched content.
fn syntax_error_diag(
    new_content: &str,
    file_path: &Path,
    language: &mut dyn Language,
    new_parse: &crate::ast::ParseResult,
) -> SyntaxErrorDiagnostic {
    language
        .explain_error(new_parse, 1)
        .unwrap_or_else(|| SyntaxErrorDiagnostic {
            src: NamedSource::new(file_path.to_string_lossy(), new_content.to_string()),
            error_span: (0, 0).into(),
            details: "Unknown syntax error".to_string(),
        })
}

///
///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
pub fn delete_line(
    file_path: &Path,
    line_num: usize,
    expected: &str,
    options: &PatchOptions,
) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();
    if line_num == 0 || line_num > lines.len() {
        anyhow::bail!("line {line_num} out of range");
    }
    let actual = lines[line_num - 1].trim();
    let expected_trimmed = expected.trim();
    if actual != expected_trimmed {
        anyhow::bail!(
            "expected line {line_num} to contain '{expected_trimmed}', but found '{actual}'"
        );
    }
    let mut new_lines: Vec<String> = lines.iter().map(std::string::ToString::to_string).collect();
    new_lines.remove(line_num - 1);
    let new_content = new_lines.join("\n") + if content.ends_with('\n') { "\n" } else { "" };
    if options.run.dry_run {
        println!("{new_content}");
    } else {
        fs::write(file_path, new_content)?;
    }
    Ok(())
}

///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
pub fn insert_lines(
    file_path: &Path,
    after_line_num: usize,
    content_to_insert: &str,
    options: &PatchOptions,
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
    let mut new_lines: Vec<String> = lines.iter().map(std::string::ToString::to_string).collect();
    let insert_idx = after_line_num;
    for line in content_to_insert.lines().rev() {
        new_lines.insert(insert_idx, line.to_string());
    }
    let new_content = new_lines.join("\n") + if content.ends_with('\n') { "\n" } else { "" };
    if options.run.dry_run {
        println!("{new_content}");
    } else {
        fs::write(file_path, new_content)?;
    }
    Ok(())
}

/// Strip leading whitespace from all context lines in a unified diff.
#[must_use]
pub fn normalize_diff_whitespace(diff_text: &str) -> String {
    let mut result = String::new();
    for line in diff_text.lines() {
        if line.starts_with(' ') || line.starts_with('-') || line.starts_with('+') {
            let control = &line[0..1];
            let content = line[1..].trim_start();
            let _ = writeln!(result, "{control}{content}");
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
pub fn apply_unified_diff(file_path: &Path, diff_text: &str, options: &PatchOptions) -> Result<()> {
    let original = fs::read_to_string(file_path)?;
    let text = if options.match_behavior.strict_whitespace {
        diff_text.to_string()
    } else {
        normalize_diff_whitespace(diff_text)
    };

    // Before applying, correct any hallucinated line numbers by matching hunk context.
    let corrected_text =
        crate::matching_flex::fix_all_hunk_headers(&text, &original, options.fuzz_radius);
    if corrected_text != text {
        eprintln!("Note: Adjusted diff line numbers based on file context.");
    }

    let diffs = flickzeug::patch_from_str(&corrected_text)
        .map_err(|e| anyhow::anyhow!("Failed to parse diff: {e}"))?;
    let mut current_content = original;
    for diff in diffs {
        match flickzeug::apply(&current_content, &diff) {
            Ok((patched, _stats)) => {
                current_content = patched;
            }
            Err(e) => {
                anyhow::bail!("Failed to apply diff: {e}");
            }
        }
    }
    if options.run.dry_run {
        println!("{current_content}");
    } else {
        fs::write(file_path, current_content)?;
    }
    Ok(())
}

///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
pub fn apply_marker_patch(
    file_path: &Path,
    marker_id: &str,
    new_content: &str,
    options: &PatchOptions,
) -> Result<()> {
    let original = fs::read_to_string(file_path)?;
    let patched = replace_marker_node(&original, marker_id, new_content)?;
    if options.run.dry_run {
        println!("{patched}");
    } else {
        fs::write(file_path, patched)?;
    }
    Ok(())
}

///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
pub fn apply_symbol_patch(
    file_path: &Path,
    symbol: &str,
    new: &str,
    options: &PatchOptions,
    language: &mut dyn Language,
) -> Result<()> {
    let original = std::fs::read_to_string(file_path)?;
    let parse_result = language.parse(&original);
    let (start, end) = language
        .find_symbol_node(&parse_result, symbol)
        .ok_or_else(|| anyhow::anyhow!("Symbol {symbol} not found"))?;
    let mut patched = original.clone();
    patched.replace_range(start..end, new);
    if options.run.dry_run {
        println!("{patched}");
    } else {
        std::fs::write(file_path, &patched)?;
    }
    Ok(())
}

///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
pub fn apply_symbol_body_patch(
    file_path: &Path,
    symbol: &str,
    new_body: &str,
    options: &PatchOptions,
    language: &mut dyn Language,
) -> Result<()> {
    let original = std::fs::read_to_string(file_path)?;
    let parse_result = language.parse(&original);
    let (start, end) = language
        .entity_body_range(&parse_result, symbol)
        .ok_or_else(|| anyhow::anyhow!("Could not find body of symbol '{symbol}'"))?;
    // Preserve the body's original leading/trailing newlines so the braces
    // stay on their own lines regardless of how the caller formatted the
    // replacement.
    let original_body = &original[start..end];
    let mut replacement = new_body.to_string();
    if original_body.starts_with('\n') && !replacement.starts_with('\n') {
        replacement.insert(0, '\n');
    }
    if original_body.ends_with('\n') && !replacement.ends_with('\n') {
        replacement.push('\n');
    }
    let mut patched = original.clone();
    patched.replace_range(start..end, &replacement);
    if options.run.dry_run {
        println!("{patched}");
    } else {
        std::fs::write(file_path, &patched)?;
    }
    Ok(())
}

/// Replace all occurrences of `old_text` with `new_text` in the file.
/// Used for file types that tree‑sitter cannot parse.
///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
pub fn full_file_replace(
    file_path: &Path,
    old_text: &str,
    new_text: &str,
    dry_run: bool,
) -> Result<()> {
    let original = std::fs::read_to_string(file_path)?;
    let patched = original.replace(old_text, new_text);
    if dry_run {
        println!("{patched}");
    } else {
        std::fs::write(file_path, patched)?;
    }
    Ok(())
}
pub mod hunk_fix;
pub mod node_patch;

// Re-export the byte-span replacement primitive from patchex.
pub use crate::patchex::replace_byte_range;
