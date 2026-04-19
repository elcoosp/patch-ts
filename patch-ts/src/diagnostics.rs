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
#[error("patch introduces syntax error")]
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
pub struct JsonError {
    pub code: String,
    pub message: String,
    pub span: JsonSpan,
    pub context: String,
    pub suggestion: Option<String>,
}

#[derive(serde::Serialize)]
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
