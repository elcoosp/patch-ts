//! High-level, in-memory API for library consumers.
//!
//! Every other patching entry point in this crate is file-based; this module
//! exposes the core engines — literal search/replace, delimiter balancing and
//! syntax validation — over plain strings, so embedders (editors, bots, AI
//! agents) never need to round-trip content through temporary files.
//!
//! # Examples
//!
//! ```
//! use patch_ts::api;
//! use patch_ts::ast::PythonLanguage;
//!
//! // Unique search/replace over raw content.
//! let patched = api::apply_search_replace("let a = 1;\nlet b = 2;\n", "let a = 1;", "let a = 42;").unwrap();
//! assert_eq!(patched, "let a = 42;\nlet b = 2;\n");
//!
//! // Repair unbalanced delimiters in memory.
//! let mut lang = PythonLanguage::new();
//! let fixed = api::balance_content("x = (1 + 2\n", &mut lang, 3).unwrap();
//! assert_eq!(fixed.repaired, "x = (1 + 2)\n");
//!
//! // Validate without repairing.
//! let mut lang = PythonLanguage::new();
//! let report = api::validate_content("def f(:\n", &mut lang);
//! assert!(!report.valid);
//! ```
use crate::ast::{DelimiterError, Language};
use crate::repair::search::minimum_cost_repair;
use anyhow::{Context, Result};

/// The result of a successful [`balance_content`] call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceOutcome {
    /// The repaired source text.
    pub repaired: String,
    /// Number of delimiter edits (insertions + deletions) applied.
    pub actions: usize,
    /// Total cost of the applied repair sequence.
    pub cost: usize,
}

/// The result of a [`validate_content`] call.
#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxReport {
    /// Whether the content parses without ERROR or MISSING nodes.
    pub valid: bool,
    /// Delimiter-level problems found by the repair engine.
    pub delimiter_errors: Vec<DelimiterError>,
}

/// Replace the unique occurrence of `search` in `content` with `replace`.
///
/// # Errors
///
/// Returns an error when `search` is empty, or when it matches zero or
/// several locations — ambiguity is refused rather than guessed at, which is
/// the safe default for automated patching.
pub fn apply_search_replace(content: &str, search: &str, replace: &str) -> Result<String> {
    if search.is_empty() {
        anyhow::bail!("search text must not be empty");
    }
    let matches = content.matches(search).count();
    match matches {
        0 => anyhow::bail!("search text not found: {search:?} ({} bytes)", search.len()),
        1 => Ok(content.replacen(search, replace, 1)),
        n => anyhow::bail!(
            "search text matches {n} locations; refine the context or use apply_search_replace_all"
        ),
    }
}

/// Replace every occurrence of `search` in `content` with `replace`,
/// returning the new content and the number of replacements.
///
/// # Errors
///
/// Returns an error when `search` is empty.
pub fn apply_search_replace_all(
    content: &str,
    search: &str,
    replace: &str,
) -> Result<(String, usize)> {
    if search.is_empty() {
        anyhow::bail!("search text must not be empty");
    }
    let count = content.matches(search).count();
    Ok((content.replace(search, replace), count))
}

/// Balance unbalanced delimiters in `content` in memory.
///
/// Runs the same minimum-cost repair search as the `balance` command, but
/// without touching the filesystem.
///
/// # Errors
///
/// Returns an error when the content has syntax errors that are not delimiter
/// problems, or when no repair exists within `max_cost` edits.
pub fn balance_content(
    content: &str,
    language: &mut dyn Language,
    max_cost: usize,
) -> Result<BalanceOutcome> {
    let parse_result = language.parse(content);
    if language.is_valid(&parse_result) {
        return Ok(BalanceOutcome {
            repaired: content.to_string(),
            actions: 0,
            cost: 0,
        });
    }
    let errors = language.find_delimiter_errors(&parse_result);
    if errors.is_empty() {
        anyhow::bail!("content has syntax errors that are not delimiter problems");
    }
    let Some((repaired, actions, cost)) = minimum_cost_repair(content, &errors, language, max_cost)
    else {
        anyhow::bail!("unable to balance content within max_cost {max_cost}");
    };
    Ok(BalanceOutcome {
        repaired,
        actions: actions.len(),
        cost,
    })
}

/// Validate `content` without repairing it.
#[must_use]
pub fn validate_content(content: &str, language: &mut dyn Language) -> SyntaxReport {
    let parse_result = language.parse(content);
    let valid = language.is_valid(&parse_result);
    let delimiter_errors = if valid {
        Vec::new()
    } else {
        language.find_delimiter_errors(&parse_result)
    };
    SyntaxReport {
        valid,
        delimiter_errors,
    }
}

/// Build a language backend from a file extension such as `"rs"` or `"py"`.
///
/// Thin convenience wrapper over [`crate::ast::detect_language`] for callers
/// that hold an extension rather than a path.
///
/// # Errors
///
/// Returns an error for unsupported extensions.
pub fn language_from_extension(extension: &str) -> Result<Box<dyn Language>> {
    crate::ast::detect_language(std::path::Path::new(&format!("inline.{extension}")))
        .with_context(|| format!("unsupported extension {extension:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{DelimiterError, PythonLanguage, RustLanguage, Span};

    #[test]
    fn search_replace_unique() {
        let out = apply_search_replace("a\nb\na\n", "b", "B").unwrap();
        assert_eq!(out, "a\nB\na\n");
    }

    #[test]
    fn search_replace_not_found() {
        let err = apply_search_replace("abc", "zzz", "q").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn search_replace_ambiguous() {
        let err = apply_search_replace("x\nx\n", "x", "y").unwrap_err();
        assert!(err.to_string().contains("2 locations"));
    }

    #[test]
    fn search_replace_empty_search() {
        let err = apply_search_replace("abc", "", "y").unwrap_err();
        assert!(err.to_string().contains("must not be empty"));
    }

    #[test]
    fn search_replace_all_counts() {
        let (out, n) = apply_search_replace_all("x x x", "x", "y").unwrap();
        assert_eq!(out, "y y y");
        assert_eq!(n, 3);
        let err = apply_search_replace_all("abc", "", "y").unwrap_err();
        assert!(err.to_string().contains("must not be empty"));
    }

    #[test]
    fn balance_valid_content_is_unchanged() {
        let mut lang = RustLanguage::new();
        let out = balance_content("fn main() {}\n", &mut lang, 3).unwrap();
        assert_eq!(
            out,
            BalanceOutcome {
                repaired: "fn main() {}\n".to_string(),
                actions: 0,
                cost: 0,
            }
        );
    }

    #[test]
    fn balance_fixes_missing_paren() {
        let mut lang = PythonLanguage::new();
        let out = balance_content("x = (1 + 2\n", &mut lang, 3).unwrap();
        // The closer lands before the trailing newline.
        assert_eq!(out.repaired, "x = (1 + 2)\n");
        assert_eq!(out.actions, 1);
        assert_eq!(out.cost, 1);
    }

    #[test]
    fn balance_non_delimiter_error_bails() {
        let mut lang = PythonLanguage::new();
        // "def 1bad():" is a syntax error tree-sitter cannot express as a
        // pure delimiter problem.
        let err = balance_content("def 1bad():\n", &mut lang, 3).unwrap_err();
        assert!(err.to_string().contains("not delimiter problems"));
    }

    #[test]
    fn balance_max_cost_exceeded_bails() {
        let mut lang = RustLanguage::new();
        let err = balance_content("fn main() {\n", &mut lang, 0).unwrap_err();
        assert!(err.to_string().contains("max_cost 0"));
    }

    #[test]
    fn validate_reports_valid_and_invalid() {
        let mut lang = RustLanguage::new();
        let ok = validate_content("fn main() {}\n", &mut lang);
        assert!(ok.valid);
        assert_eq!(ok.delimiter_errors.len(), 0);

        let bad = validate_content("fn main() { let x = (1 + 2)); }", &mut lang);
        assert!(!bad.valid);
        assert_ne!(bad.delimiter_errors.len(), 0);
    }

    #[test]
    fn syntax_report_constructors_cover_variants() {
        // Exercises DelimiterError construction paths used by api consumers
        // building reports in tests.
        let extra = DelimiterError::Extra {
            span: Span {
                start_byte: 0,
                end_byte: 1,
                start_line: 1,
                start_column: 1,
                end_line: 1,
                end_column: 2,
            },
            delimiter: ')',
        };
        assert_eq!(extra.delta(), -1);
    }

    #[test]
    fn language_from_extension_roundtrip() {
        let mut lang = language_from_extension("rs").unwrap();
        let report = validate_content("fn main() {}\n", &mut *lang);
        assert!(report.valid);

        let Err(err) = language_from_extension("exotic") else {
            panic!("exotic extension should be unsupported");
        };
        assert!(err.to_string().contains("exotic"));
    }
}
