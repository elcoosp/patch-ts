use crate::ast::DelimiterError;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("file not found")]
#[diagnostic(code(patch_ts::file_not_found))]
pub struct FileNotFoundError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("file path")]
    pub path_span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("expected content not found")]
#[diagnostic(
    code(patch_ts::content_mismatch),
    help("try increasing --fuzz radius")
)]
pub struct ContentMismatchError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("expected content here")]
    pub expected_span: SourceSpan,
    #[label("actual content")]
    pub actual_span: SourceSpan,
    pub expected: String,
    pub actual: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("patch introduces syntax error: {details}")]
#[diagnostic(
    code(patch_ts::syntax_error),
    help("use --force to apply anyway, or run `patch-ts balance` to fix")
)]
pub struct SyntaxErrorDiagnostic {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("syntax error here")]
    pub error_span: SourceSpan,
    pub details: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("fuzzy match ambiguous: multiple matches found")]
#[diagnostic(
    code(patch_ts::ambiguous_match),
    help("provide more context in expected content")
)]
pub struct AmbiguousMatchError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("candidate 1")]
    pub candidate1_span: SourceSpan,
    #[label("candidate 2")]
    pub candidate2_span: SourceSpan,
}

/// JSON-serializable diagnostic output.
#[derive(serde::Serialize)]
pub struct JsonDiagnostic {
    pub success: bool,
    pub error: Option<JsonError>,
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct JsonError {
    pub code: String,
    pub message: String,
    pub span: JsonSpan,
    pub context: String,
    pub suggestion: Option<String>,
    pub best_score: Option<f64>,
    pub best_match_line: Option<usize>,
    pub candidates: Option<Vec<Candidate>>,
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct Candidate {
    pub line: usize,
    pub score: f64,
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct JsonSpan {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl JsonDiagnostic {
    pub fn success() -> Self {
        Self {
            success: true,
            error: None,
        }
    }

    pub fn error(error: JsonError) -> Self {
        Self {
            success: false,
            error: Some(error),
        }
    }
}

/// Convert anyhow error to JSON with enhanced fields.
pub fn anyhow_to_json(err: &anyhow::Error, file: &str) -> JsonError {
    let msg = err.to_string();

    // Attempt to extract matching info from error message
    let mut best_score = None;
    let mut best_match_line = None;
    let mut suggestion = None;

    if msg.contains("no match found with similarity") {
        suggestion = Some("Try increasing --fuzz radius".to_string());
        // Try to parse score and line
        if let Some(score_part) = msg.split("best was ").nth(1) {
            let parts: Vec<&str> = score_part.split_whitespace().collect();
            if parts.len() >= 3 {
                best_score = parts[0].parse().ok();
                best_match_line = parts[3].parse().ok();
            }
        }
    } else if msg.contains("ambiguous match") {
        suggestion = Some("Provide more specific expected content".to_string());
    }

    if let Some(diag) = err.downcast_ref::<SyntaxErrorDiagnostic>() {
        return JsonError {
            code: "patch_ts::syntax_error".to_string(),
            message: diag.to_string(),
            span: JsonSpan {
                file: file.to_string(),
                line: 1,
                column: 1,
            },
            context: diag.details.clone(),
            suggestion: Some("Run `patch-ts balance` to attempt automatic fix".to_string()),
            best_score: None,
            best_match_line: None,
            candidates: None,
        };
    }
    if let Some(diag) = err.downcast_ref::<ContentMismatchError>() {
        return JsonError {
            code: "patch_ts::content_mismatch".to_string(),
            message: diag.to_string(),
            span: JsonSpan {
                file: file.to_string(),
                line: 1,
                column: 1,
            },
            context: format!("expected '{}' but found '{}'", diag.expected, diag.actual),
            suggestion: Some("try increasing --fuzz radius".to_string()),
            best_score: None,
            best_match_line: None,
            candidates: None,
        };
    }
    JsonError {
        code: "patch_ts::error".to_string(),
        message: msg,
        span: JsonSpan {
            file: file.to_string(),
            line: 0,
            column: 0,
        },
        context: String::new(),
        suggestion,
        best_score,
        best_match_line,
        candidates: None,
    }
}

/// JSON output for the balance command.
#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct BalanceResult {
    pub success: bool,
    pub actions: Vec<BalanceAction>,
    #[serde(skip)] pub rolled_back: Vec<DelimiterError>,
    pub error: Option<JsonError>,
}

#[derive(serde::Serialize)]
#[derive(Debug)]
pub struct BalanceAction {
    #[serde(rename = "type")]
    pub action_type: String, // "remove" or "insert"
    pub delimiter: char,
    pub line: usize,
    pub column: usize,
    pub message: String,
}
