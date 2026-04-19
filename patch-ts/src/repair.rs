use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::ast::Language;
use crate::diagnostics::SyntaxErrorDiagnostic;

/// Attempt to fix unbalanced delimiters by removing an extra delimiter
pub fn balance_file(
    file_path: &Path,
    _function_name: Option<&str>, // reserved for future scoping
    dry_run: bool,
    language: &mut dyn Language,
) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let parse_result = language.parse(&content);

    if language.is_valid(&parse_result) {
        // Already valid, nothing to do
        return Ok(());
    }

    let extra_span = language.find_extra_delimiter(&parse_result)
        .ok_or_else(|| anyhow::anyhow!("Could not identify extra delimiter"))?;

    let mut new_content = content.clone();
    new_content.replace_range(extra_span.start_byte..extra_span.end_byte, "");

    // Validate the fix
    let new_parse = language.parse(&new_content);
    if !language.is_valid(&new_parse) {
        anyhow::bail!("Removing delimiter did not fix the syntax error; manual intervention required");
    }

    if dry_run {
        println!("Would remove extra delimiter at {}:{}-{}:{}",
            extra_span.start_line, extra_span.start_column,
            extra_span.end_line, extra_span.end_column);
    } else {
        fs::write(file_path, new_content)?;
    }
    Ok(())
}

/// Explain syntax error at a specific line
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
