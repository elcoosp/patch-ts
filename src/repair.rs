use crate::ast::DelimiterError;
use anyhow::Result;
use line_index::LineIndex;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::ast::Language;
use crate::diagnostics::{
    BalanceAction, BalanceResult, JsonError, JsonSpan, SyntaxErrorDiagnostic,
};

/// Determine the optimal insertion span for a missing delimiter based on parent context.
fn compute_insertion_span(
    error: &DelimiterError,
    _content: &str,
    _index: &LineIndex,
) -> crate::ast::Span {
    match error {
        DelimiterError::Missing {
            insert_at,
            parent_kind,
            ..
        } => {
            if let Some(kind) = parent_kind {
                if kind == "block" || kind == "function_body" || kind == "statement_block" {
                    return insert_at.clone();
                }
            }
            insert_at.clone()
        }
        _ => unreachable!(),
    }
}

pub fn apply_repair(content: &str, error: &DelimiterError, index: &LineIndex) -> String {
    match error {
        DelimiterError::Extra { span, .. } => {
            let mut new_content = content.to_string();
            new_content.replace_range(span.start_byte..span.end_byte, "");
            new_content
        }
        DelimiterError::Missing { expected, .. } => {
            let span = compute_insertion_span(error, content, index);
            let mut new_content = content.to_string();
            let insert_byte = span.end_byte;
            new_content.insert(insert_byte, *expected);
            new_content
        }
    }
}

pub fn balance_file(
    file_path: &Path,
    function_name: Option<&str>,
    dry_run: bool,
    language: &mut dyn Language,
) -> Result<BalanceResult> {
    let original_content = fs::read_to_string(file_path)?;
    let parse_result = language.parse(&original_content);
    let index = &parse_result.index;

    // Validate function existence upfront (Rust only)
    if let Some(func_name) = function_name {
        let rust_lang = language
            .as_any_mut()
            .downcast_mut::<crate::ast::RustLanguage>()
            .ok_or_else(|| anyhow::anyhow!("Function scoping only supported for Rust"))?;
        if rust_lang
            .find_function_body_range(&original_content, func_name)
            .is_none()
        {
            anyhow::bail!("Function '{}' not found or ambiguous", func_name);
        }
    }

    // Collect all errors
    let errors = if let Some(func_name) = function_name {
        find_delimiter_errors_in_function(language, &parse_result, func_name)?
    } else {
        language.find_delimiter_errors(&parse_result)
    };

    if errors.is_empty() {
        if language.is_valid(&parse_result) {
            if dry_run {
                println!("File is already valid; no changes needed.");
            }
            return Ok(BalanceResult {
                success: true,
                actions: vec![],
                rolled_back: vec![],
                error: None,
            });
        } else {
            anyhow::bail!("File has syntax errors but no delimiter errors were identified");
        }
    }

    // Sort errors by descending start byte to minimize offset interference
    let mut sorted_errors = errors.clone();
    sorted_errors.sort_by(|a, b| {
        let span_a = a.span();
        let span_b = b.span();
        span_b.start_byte.cmp(&span_a.start_byte)
    });

    let mut current_content = original_content.clone();
    let mut actions = Vec::new();
    let mut rolled_back = Vec::new();
    let mut offset_shift: isize = 0;

    for error in &sorted_errors {
        // Store state before repair
        let before_content = current_content.clone();
        let before_parse = language.parse(&before_content);
        let before_error_count = language.find_delimiter_errors(&before_parse).len();

        // Create a mutable copy of the error with adjusted span
        let mut adjusted_error = error.clone();
        *adjusted_error.span_mut() = error.span().shift(offset_shift);

        // Apply repair
        current_content = apply_repair(&current_content, &adjusted_error, index);
        let after_parse = language.parse(&current_content);
        let after_error_count = language.find_delimiter_errors(&after_parse).len();

        // Validate: rollback if error count increased
        if after_error_count > before_error_count {
            current_content = before_content;
            rolled_back.push(error.clone());
            continue;
        }

        // Update offset and record successful action
        offset_shift += error.delta();
        let action = match &adjusted_error {
            DelimiterError::Extra { span, delimiter } => BalanceAction {
                action_type: "remove".to_string(),
                delimiter: *delimiter,
                line: span.start_line,
                column: span.start_column,
                message: format!("Removed extra '{}'", delimiter),
            },
            DelimiterError::Missing {
                expected,
                insert_at,
                ..
            } => BalanceAction {
                action_type: "insert".to_string(),
                delimiter: *expected,
                line: insert_at.end_line,
                column: insert_at.end_column,
                message: format!("Inserted missing '{}'", expected),
            },
        };
        actions.push(action);

        if dry_run {
            println!(
                "Would {} at {}:{}",
                actions.last().unwrap().message,
                actions.last().unwrap().line,
                actions.last().unwrap().column
            );
        }
    }

    let final_parse = language.parse(&current_content);
    if !language.is_valid(&final_parse) {
        let err = anyhow::anyhow!("Repair completed but file is still invalid");
        return Ok(BalanceResult {
            success: false,
            actions,
            rolled_back,
            error: Some(JsonError {
                code: "patch_ts::balance_incomplete".to_string(),
                message: err.to_string(),
                span: JsonSpan {
                    file: file_path.to_string_lossy().to_string(),
                    line: 0,
                    column: 0,
                },
                context: String::new(),
                suggestion: Some("Manual intervention required".to_string()),
                best_score: None,
                best_match_line: None,
                candidates: None,
            }),
        });
    }

    if !dry_run {
        fs::write(file_path, current_content)?;
    }

    Ok(BalanceResult {
        success: true,
        actions,
        rolled_back,
        error: None,
    })
}

fn find_delimiter_errors_in_function(
    language: &mut dyn Language,
    parse_result: &crate::ast::ParseResult,
    function_name: &str,
) -> Result<Vec<DelimiterError>> {
    let rust_lang = language
        .as_any_mut()
        .downcast_mut::<crate::ast::RustLanguage>()
        .ok_or_else(|| anyhow::anyhow!("Function scoping only supported for Rust"))?;

    let source = parse_result.text();
    let (start_byte, end_byte) = rust_lang
        .find_function_body_range(source, function_name)
        .ok_or_else(|| anyhow::anyhow!("Function '{}' not found or ambiguous", function_name))?;

    let all_errors = language.find_delimiter_errors(parse_result);
    let filtered: Vec<DelimiterError> = all_errors
        .into_iter()
        .filter(|e| {
            let span = e.span();
            span.start_byte <= end_byte && span.end_byte >= start_byte
        })
        .collect();

    if filtered.is_empty() {
        anyhow::bail!("No delimiter errors found in function '{}'", function_name);
    }
    Ok(filtered)
}

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

pub fn explain_error(
    file_path: &Path,
    line: usize,
    _json: bool,
    language: &mut dyn Language,
) -> Result<Option<SyntaxErrorDiagnostic>> {
    let content = fs::read_to_string(file_path)?;
    let parse_result = language.parse(&content);
    Ok(language.explain_error(&parse_result, line))
}

// Parallel balance for multiple files
pub fn balance_files(
    files: &[PathBuf],
    function_name: Option<&str>,
    dry_run: bool,
    language: &mut dyn Language,
) -> Result<Vec<BalanceResult>> {
    use rayon::prelude::*;
    files
        .par_iter()
        .map(|file| balance_file(file, function_name, dry_run, language))
        .collect()
}

// Parallel balance for multiple files
pub fn balance_files(
    files: &[PathBuf],
    function_name: Option<&str>,
    dry_run: bool,
    language: &mut dyn Language,
) -> Result<Vec<BalanceResult>> {
    use rayon::prelude::*;
    files
        .par_iter()
        .map(|file| balance_file(file, function_name, dry_run, language))
        .collect()
}
