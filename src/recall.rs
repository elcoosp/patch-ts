use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::fs;
use std::path::Path;

use crate::ast::Language;

#[derive(Debug, Clone, Serialize)]
pub struct RecallContext {
    pub recall_id: String,
    pub file: String,
    pub attempt: AttemptInfo,
    pub error: ErrorInfo,
    pub context: ContextInfo,
    pub strategies: Vec<StrategyInfo>,
    pub history: HistoryInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct AttemptInfo {
    pub line: usize,
    pub old: String,
    pub new: String,
    pub strategy_used: Option<String>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorInfo {
    pub code: String,
    pub category: String,
    pub message: String,
    pub retry_prompt: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContextInfo {
    pub surrounding_lines: Vec<String>,
    pub symbol: Option<String>,
    pub language: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StrategyInfo {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HistoryInfo {
    pub similar_failures: usize,
    pub suggested_approach: String,
}

/// Static database mapping error codes to retry strategies.
#[derive(Debug, Clone)]
struct RecallStrategyDef {
    error_code: &'static str,
    pattern: &'static str,
    strategies: Vec<RetrySuggestion>,
    priority: u8,
}

#[derive(Debug, Clone)]
struct RetrySuggestion {
    name: &'static str,
    description: &'static str,
}

static RECALL_STRATEGIES: Lazy<Vec<RecallStrategyDef>> = Lazy::new(|| {
    vec![
        RecallStrategyDef {
            error_code: "E001",
            pattern: "file not found",
            strategies: vec![RetrySuggestion {
                name: "check_path",
                description: "Check the file path; use --files for glob patterns",
            }],
            priority: 0,
        },
        RecallStrategyDef {
            error_code: "E002",
            pattern: "content_mismatch",
            strategies: vec![
                RetrySuggestion {
                    name: "increase_fuzz",
                    description: "Increase --fuzz radius to 3 or more",
                },
                RetrySuggestion {
                    name: "use_anchor",
                    description: "Match on unique surrounding lines as anchors",
                },
                RetrySuggestion {
                    name: "use_symbol",
                    description: "Use --symbol <FUNCTION_NAME> to target the function",
                },
            ],
            priority: 0,
        },
        RecallStrategyDef {
            error_code: "E003",
            pattern: "confidence_below_threshold",
            strategies: vec![
                RetrySuggestion {
                    name: "provide_context",
                    description: "Provide more unique or specific content for matching",
                },
                RetrySuggestion {
                    name: "use_symbol",
                    description: "Use --symbol <FUNCTION_NAME> to bypass content matching",
                },
            ],
            priority: 0,
        },
        RecallStrategyDef {
            error_code: "E004",
            pattern: "ambiguous_match",
            strategies: vec![
                RetrySuggestion {
                    name: "narrow_context",
                    description: "Add more surrounding lines to disambiguate",
                },
                RetrySuggestion {
                    name: "use_symbol",
                    description: "Use --symbol <FUNCTION_NAME> to target uniquely",
                },
            ],
            priority: 0,
        },
        RecallStrategyDef {
            error_code: "E005",
            pattern: "syntax_error",
            strategies: vec![
                RetrySuggestion {
                    name: "balance",
                    description: "Run `patch-ts balance --file <FILE> --apply` first",
                },
                RetrySuggestion {
                    name: "force",
                    description: "Use --force to apply despite syntax error (then balance)",
                },
            ],
            priority: 0,
        },
        RecallStrategyDef {
            error_code: "E006",
            pattern: "compilation_error",
            strategies: vec![
                RetrySuggestion {
                    name: "iterative",
                    description: "Use --no-compile-check for interim patches; compile after all fixes",
                },
                RetrySuggestion {
                    name: "fix_errors",
                    description: "Review the compiler errors and correct the patch",
                },
            ],
            priority: 0,
        },
        RecallStrategyDef {
            error_code: "E000",
            pattern: "",
            strategies: vec![
                RetrySuggestion {
                    name: "explain",
                    description: "Run `patch-ts explain` to diagnose the issue",
                },
                RetrySuggestion {
                    name: "manual",
                    description: "Manually inspect the file and craft a patch",
                },
            ],
            priority: 1, // default low priority
        },
    ]
});

/// Generate a recall context when a patch fails.
pub fn generate_recall_context(
    file_path: &Path,
    line: usize,
    old_content: &str,
    new_content: &str,
    error_code: &str,
    error_message: Option<&str>,
    context_lines: usize,
    language: &mut dyn Language,
) -> Result<RecallContext> {
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    // Extract surrounding lines
    let all_lines: Vec<&str> = file_content.lines().collect();
    let target_idx = line.saturating_sub(1);
    let start = target_idx.saturating_sub(context_lines);
    let end = (target_idx + context_lines + 1).min(all_lines.len());
    let surrounding_lines: Vec<String> = all_lines[start..end].iter().map(|s| s.to_string()).collect();

    // Try to identify the containing symbol (function/class)
    let symbol = if let Some((sym_name, _)) = find_containing_symbol(&file_content, line, language) {
        Some(sym_name)
    } else {
        None
    };

    // Build error info
    let category = error_code_to_category(error_code);
    let message = error_message.unwrap_or("").to_string();
    let retry_prompt = build_retry_prompt(error_code, &message, &surrounding_lines, symbol.as_deref());

    // Select strategies
    let strategies: Vec<StrategyInfo> = get_strategies(error_code, &message);

    // Query history (simplified: count past failures with same error code)
    let similar_failures = count_similar_failures(file_path, error_code);
    let suggested_approach = if let Some(first) = strategies.first() {
        first.name.clone()
    } else {
        "manual".to_string()
    };

    let recall_id = format!("recall-{}", chrono::Utc::now().format("%Y-%m-%d-%H%M%S"));

    Ok(RecallContext {
        recall_id,
        file: file_path.to_string_lossy().to_string(),
        attempt: AttemptInfo {
            line,
            old: old_content.to_string(),
            new: new_content.to_string(),
            strategy_used: None,
            confidence: None,
        },
        error: ErrorInfo {
            code: error_code.to_string(),
            category: category.to_string(),
            message,
            retry_prompt,
        },
        context: ContextInfo {
            surrounding_lines,
            symbol,
            language: file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string(),
        },
        strategies,
        history: HistoryInfo {
            similar_failures,
            suggested_approach,
        },
    })
}

/// Map error code to category name.
fn error_code_to_category(code: &str) -> &str {
    match code {
        "E001" => "file_not_found",
        "E002" => "content_mismatch",
        "E003" => "confidence_below_threshold",
        "E004" => "ambiguous_match",
        "E005" => "syntax_error",
        "E006" => "compilation_error",
        _ => "unknown",
    }
}

/// Build a human‑readable retry prompt.
fn build_retry_prompt(code: &str, message: &str, lines: &[String], symbol: Option<&str>) -> String {
    let mut prompt = String::new();
    prompt.push_str(&format!("Error {}: {}\n", code, message));
    prompt.push_str("Surrounding context:\n");
    for line in lines {
        prompt.push_str(&format!("  {}\n", line));
    }
    if let Some(sym) = symbol {
        prompt.push_str(&format!("\nTarget symbol: {}\n", sym));
    }
    prompt.push_str("\nSuggestions:\n");
    for strategy in get_strategies(code, message) {
        prompt.push_str(&format!("- {}: {}\n", strategy.name, strategy.description));
    }
    prompt
}

/// Retrieve the appropriate strategies for the error code and message pattern.
fn get_strategies(code: &str, message: &str) -> Vec<StrategyInfo> {
    let defs = RECALL_STRATEGIES.iter()
        .filter(|s| s.error_code == code && (s.pattern.is_empty() || message.contains(s.pattern)))
        .min_by_key(|s| s.priority);
    if let Some(def) = defs {
        def.strategies.iter()
            .map(|s| StrategyInfo { name: s.name.to_string(), description: s.description.to_string() })
            .collect()
    } else {
        // fallback: generic advice
        vec![StrategyInfo {
            name: "manual".to_string(),
            description: "Manually inspect the file and craft a patch".to_string(),
        }]
    }
}

/// Find the name of the function/struct/class that contains a given line.
fn find_containing_symbol(_source: &str, _line: usize, _language: &mut dyn crate::ast::Language) -> Option<(String, usize)> {
    None
}


fn count_similar_failures(file_path: &std::path::Path, error_code: &str) -> usize {
    let recall_file = std::path::PathBuf::from(".patch-ts/recall.jsonl");
    if !recall_file.exists() { return 0; }
    let content = std::fs::read_to_string(&recall_file).unwrap_or_default();
    let file_name = file_path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    content.lines()
        .filter(|line| {
            if let Ok(rec) = serde_json::from_str::<serde_json::Value>(line) {
                rec.get("error_code").and_then(|e| e.as_str()) == Some(error_code)
                    && rec.get("file").and_then(|f| f.as_str()).map(|f| f.contains(file_name)).unwrap_or(false)
            } else { false }
        })
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::RustLanguage;
    use tempfile::tempdir;

    #[test]
    fn test_generate_recall_context_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() {\n    let x = 1;\n}\n").unwrap();

        let mut lang = RustLanguage::new();
        let ctx = generate_recall_context(
            &file_path,
            2,
            "let x = 1;",
            "let x = 2;",
            "E002",
            Some("expected line 2 to contain 'let x = 1;' but found 'let x = 1'"),
            2,
            &mut lang,
        ).unwrap();

        assert_eq!(ctx.attempt.line, 2);
        assert_eq!(ctx.error.code, "E002");
        assert!(!ctx.strategies.is_empty());
        // Check JSON serialization
        let json = serde_json::to_string(&ctx).unwrap();
        assert!(json.contains("recall-"));
    }

    #[test]
    fn test_strategy_matching() {
        let strategies = get_strategies("E002", "content_mismatch");
        assert!(!strategies.is_empty());
        assert!(strategies.iter().any(|s| s.name == "increase_fuzz"));

        let unknown = get_strategies("E999", "unknown");
        assert_eq!(unknown[0].name, "manual");
    }
}
