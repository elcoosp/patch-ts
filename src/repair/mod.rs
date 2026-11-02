pub mod history;
pub mod search;

use crate::ast::DelimiterError;
use crate::plugin::PluginHost;
use crate::repair::search::{RepairAction, minimum_cost_repair};
use anyhow::Result;
use line_index::LineIndex;
use std::fs;
use std::path::{Path, PathBuf};

use crate::ast::Language;
use crate::diagnostics::{
    BalanceAction, BalanceResult, JsonError, JsonSpan, SyntaxErrorDiagnostic,
};

fn compute_insertion_span(
    error: &DelimiterError,
    _content: &str,
    _index: &LineIndex,
) -> crate::ast::Span {
    error.span().clone()
}

#[must_use]
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

///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
pub fn balance_file(
    file_path: &Path,
    function_name: Option<&str>,
    dry_run: bool,
    language: &mut dyn Language,
    plugin_path: Option<&str>,
    max_cost: usize,
) -> Result<BalanceResult> {
    let original_content = fs::read_to_string(file_path)?;
    let parse_result = language.parse(&original_content);

    if let Some(func_name) = function_name {
        let rust_lang = language
            .as_any_mut()
            .downcast_mut::<crate::ast::RustLanguage>()
            .ok_or_else(|| anyhow::anyhow!("Function scoping only supported for Rust"))?;
        if rust_lang
            .find_function_body_range(&original_content, func_name)
            .is_none()
        {
            anyhow::bail!("Function '{func_name}' not found or ambiguous");
        }
    }

    let errors = if let Some(func_name) = function_name {
        find_delimiter_errors_in_function(language, &parse_result, func_name)?
    } else {
        language.find_delimiter_errors(&parse_result)
    };

    let repaired_content = if let Some(path) = plugin_path {
        let host = PluginHost::load(&PathBuf::from(path))?;
        host.repair(&errors, &original_content)?
    } else {
        original_content.clone()
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
        }
        anyhow::bail!("File has syntax errors but no delimiter errors were identified");
    }

    let search_result = minimum_cost_repair(&repaired_content, &errors, language, max_cost);

    let Some((patched_content, actions, _cost)) = search_result else {
        return Ok(BalanceResult {
            success: false,
            actions: vec![],
            rolled_back: vec![],
            error: Some(JsonError {
                code: "patch_ts::balance_max_cost_exceeded".to_string(),
                message: format!("Unable to find repair within max cost {max_cost}"),
                span: JsonSpan {
                    file: file_path.to_string_lossy().to_string(),
                    line: 0,
                    column: 0,
                },
                context: String::new(),
                suggestion: Some("Try increasing max_cost or manually fix".to_string()),
                best_score: None,
                best_match_line: None,
                candidates: None,
                error_code: Some("E000".to_string()),
                retry_prompt: None,
            }),
        });
    };

    let balance_actions: Vec<BalanceAction> = actions
        .iter()
        .map(|action| match action {
            RepairAction::Insert { ch, pos } => {
                let (line, col) = offset_to_line_col(&repaired_content, *pos);
                BalanceAction {
                    action_type: "insert".to_string(),
                    delimiter: *ch,
                    line,
                    column: col,
                    message: format!("Inserted missing '{ch}'"),
                }
            }
            RepairAction::Delete { start, end: _ } => {
                let (line, col) = offset_to_line_col(&repaired_content, *start);
                let delimiter = repaired_content.chars().nth(*start).unwrap_or('?');
                BalanceAction {
                    action_type: "remove".to_string(),
                    delimiter,
                    line,
                    column: col,
                    message: format!("Removed extra '{delimiter}'"),
                }
            }
        })
        .collect();

    if dry_run {
        println!("{patched_content}");
    } else {
        fs::write(file_path, &patched_content)?;
    }

    Ok(BalanceResult {
        success: true,
        actions: balance_actions,
        rolled_back: vec![],
        error: None,
    })
}

fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    use line_index::{LineIndex, TextSize};
    let index = LineIndex::new(source);
    let pos = index.line_col(TextSize::from(crate::num::to_u32(offset)));
    (pos.line as usize + 1, pos.col as usize + 1)
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
        .ok_or_else(|| anyhow::anyhow!("Function '{function_name}' not found or ambiguous"))?;
    let all_errors = language.find_delimiter_errors(parse_result);
    let filtered: Vec<DelimiterError> = all_errors
        .into_iter()
        .filter(|e| {
            let span = e.span();
            span.start_byte <= end_byte && span.end_byte >= start_byte
        })
        .collect();
    if filtered.is_empty() {
        anyhow::bail!("No delimiter errors found in function '{function_name}'");
    }
    Ok(filtered)
}

pub fn quick_balance(content: &str, language: &mut dyn Language) -> Option<String> {
    let parse_result = language.parse(content);
    if language.is_valid(&parse_result) {
        return Some(content.to_string());
    }
    let errors = language.find_delimiter_errors(&parse_result);
    minimum_cost_repair(content, &errors, language, 3).map(|(s, _, _)| s)
}

///
/// # Errors
///
/// Returns an error if the operation fails.
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

///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
pub fn balance_files(
    files: &[PathBuf],
    function_name: Option<&str>,
    dry_run: bool,
    language: &mut dyn Language,
) -> Result<Vec<BalanceResult>> {
    files
        .iter()
        .map(|file| balance_file(file, function_name, dry_run, language, None, 10))
        .collect()
}
