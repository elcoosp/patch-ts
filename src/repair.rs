use anyhow::Result;
use std::fs;
use std::path::Path;
use crate::ast::DelimiterError;

use crate::ast::Language;
use crate::diagnostics::SyntaxErrorDiagnostic;

pub fn apply_repair(content: &str, error: &DelimiterError) -> String {
    match error {
        DelimiterError::Extra { span, .. } => {
            let mut new_content = content.to_string();
            new_content.replace_range(span.start_byte..span.end_byte, "");
            new_content
        }
        DelimiterError::Missing { expected, insert_at } => {
            let mut new_content = content.to_string();
            let insert_byte = insert_at.end_byte;
            new_content.insert(insert_byte, *expected);
            new_content
        }
    }
}

pub fn balance_file(
    file_path: &Path,
    function_name: Option<&str>,
    dry_run: bool,
    json_output: bool,
    language: &mut dyn Language,
) -> Result<()> {
    let original_content = fs::read_to_string(file_path)?;
    let mut current_content = original_content.clone();
    const MAX_ITERATIONS: usize = 10;

    if let Some(func_name) = function_name {
        let rust_lang = language
            .as_any_mut()
            .downcast_mut::<crate::ast::RustLanguage>()
            .ok_or_else(|| anyhow::anyhow!("Function scoping only supported for Rust"))?;
        if rust_lang.find_function_body_range(&original_content, func_name).is_none() {
            anyhow::bail!("Function '{}' not found or ambiguous", func_name);
        }
    }

    for iteration in 0..MAX_ITERATIONS {
        let parse_result = language.parse(&current_content);
        if language.is_valid(&parse_result) {
            if iteration == 0 {
                if !json_output && dry_run {
                    println!("File is already valid; no changes needed.");
                }
                return Ok(());
            }
            break;
        }

        let errors = if let Some(func_name) = function_name {
            find_delimiter_errors_in_function(language, &parse_result, func_name)?
        } else {
            language.find_delimiter_errors(&parse_result)
        };

        if errors.is_empty() {
            anyhow::bail!("Could not identify any delimiter errors to fix");
        }

        current_content = apply_repair(&current_content, &errors[0]);

        if !json_output && dry_run {
            let action = match &errors[0] {
                DelimiterError::Extra { span, delimiter } => {
                    format!("Would remove extra '{}' at {}:{}", delimiter, span.start_line, span.start_column)
                }
                DelimiterError::Missing { expected, insert_at } => {
                    format!("Would insert missing '{}' at {}:{}", expected, insert_at.end_line, insert_at.end_column)
                }
            };
            println!("{}", action);
        }
    }

    let final_parse = language.parse(&current_content);
    if !language.is_valid(&final_parse) {
        anyhow::bail!("Repair loop completed but file is still invalid");
    }

    if !dry_run {
        fs::write(file_path, current_content)?;
    }
    Ok(())
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
            let span = match e {
                DelimiterError::Extra { span, .. } => span,
                DelimiterError::Missing { insert_at, .. } => insert_at,
            };
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
