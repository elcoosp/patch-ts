use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use crate::ast::Language;
use crate::session::Session;

#[derive(Debug, Clone, Serialize)]
pub struct RecallContext {
    pub recall_id: String,
    pub file: String,
    pub attempt: AttemptInfo,
    pub error: ErrorInfo,
    pub context: ContextInfo,
    pub strategies: Vec<StrategyInfo>,
    pub history: HistoryInfo,
    pub strategy_used: Option<String>,
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
    pub containing_body: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StrategyInfo {
    pub name: String,
    pub description: String,
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HistoryInfo {
    pub similar_failures: usize,
    pub suggested_approach: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MinimalRecallContext {
    pub error: String,
    pub message: String,
    pub line: String,
    pub best_strategy: String,
}

// Static strategy database
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

static RECALL_STRATEGIES: std::sync::LazyLock<Vec<RecallStrategyDef>> = std::sync::LazyLock::new(
    || {
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
                priority: 1,
            },
        ]
    },
);

// --- Entropy helpers ---
fn compute_line_entropy(line: &str) -> f64 {
    if line.is_empty() {
        return 0.0;
    }
    let mut counts: HashMap<char, u32> = HashMap::new();
    let total = crate::num::to_f64(line.chars().count());
    for ch in line.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
        .values()
        .map(|&c| {
            let p = f64::from(c) / total;
            -p * p.log2()
        })
        .sum()
}

fn select_high_entropy_lines(
    lines: &[String],
    target_idx: usize,
    max_lines: usize,
    threshold: f64,
) -> Vec<usize> {
    let mut scored: Vec<(usize, f64)> = lines
        .iter()
        .enumerate()
        .map(|(i, l)| (i, compute_line_entropy(l)))
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut selected = vec![target_idx];
    for (idx, score) in scored {
        if selected.len() >= max_lines {
            break;
        }
        if score >= threshold && idx != target_idx {
            selected.push(idx);
        }
    }
    selected.sort_unstable();
    selected
}

// --- Strategy helpers ---
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

fn get_strategies(code: &str, message: &str) -> Vec<StrategyInfo> {
    let defs = RECALL_STRATEGIES
        .iter()
        .filter(|s| s.error_code == code && (s.pattern.is_empty() || message.contains(s.pattern)))
        .min_by_key(|s| s.priority);
    if let Some(def) = defs {
        def.strategies
            .iter()
            .map(|s| StrategyInfo {
                name: s.name.to_string(),
                description: s.description.to_string(),
                score: None,
            })
            .collect()
    } else {
        vec![StrategyInfo {
            name: "manual".to_string(),
            description: "Manually inspect the file and craft a patch".to_string(),
            score: None,
        }]
    }
}

fn build_retry_prompt(code: &str, message: &str, lines: &[String], symbol: Option<&str>) -> String {
    let mut prompt = String::new();
    let _ = writeln!(prompt, "Error {code}: {message}");
    prompt.push_str("Surrounding context:\n");
    for line in lines {
        let _ = writeln!(prompt, "  {line}");
    }
    if let Some(s) = symbol {
        let _ = writeln!(prompt, "\nTarget symbol: {s}");
    }
    prompt.push_str("\nSuggestions:\n");
    for s in get_strategies(code, message) {
        let _ = writeln!(prompt, "- {}: {}", s.name, s.description);
    }
    prompt
}

// --- Core recall generation ---
///
/// # Errors
///
/// Returns an error if the file or record cannot be read.
/// Everything the recall-context generator needs to know about a failure.
#[derive(Debug, Clone)]
pub struct RecallQuery<'a> {
    pub file_path: &'a Path,
    pub line: usize,
    pub old_content: &'a str,
    pub new_content: &'a str,
    pub error_code: &'a str,
    pub error_message: Option<&'a str>,
    pub context_lines: usize,
    pub use_entropy: bool,
    pub entropy_threshold: f64,
    pub pre_fetch: bool,
    pub session_id: Option<&'a str>,
    pub max_tokens: Option<usize>,
}

impl<'a> RecallQuery<'a> {
    /// Minimal query with sensible defaults (no entropy, no session).
    #[must_use]
    pub fn new(
        file_path: &'a Path,
        line: usize,
        old_content: &'a str,
        new_content: &'a str,
        error_code: &'a str,
    ) -> Self {
        Self {
            file_path,
            line,
            old_content,
            new_content,
            error_code,
            error_message: None,
            context_lines: 5,
            use_entropy: false,
            entropy_threshold: 2.5,
            pre_fetch: false,
            session_id: None,
            max_tokens: None,
        }
    }
}

/// Build a full recall context for a failed patch attempt.
///
/// # Errors
///
/// Returns an error if the file or record cannot be read.
pub fn generate_recall_context(
    query: &RecallQuery<'_>,
    language: &mut dyn Language,
) -> Result<RecallContext> {
    let file_content = fs::read_to_string(query.file_path)?;
    let all_lines: Vec<&str> = file_content.lines().collect();
    let target_idx = query.line.saturating_sub(1);
    let start = target_idx.saturating_sub(query.context_lines);
    let end = (target_idx + query.context_lines + 1).min(all_lines.len());
    let window_lines: Vec<String> = all_lines[start..end]
        .iter()
        .map(std::string::ToString::to_string)
        .collect();

    let surrounding_lines = if query.use_entropy {
        let selected = select_high_entropy_lines(
            &window_lines,
            target_idx - start,
            query.context_lines.min(window_lines.len()),
            query.entropy_threshold,
        );
        selected
            .into_iter()
            .map(|i| window_lines[i].clone())
            .collect()
    } else {
        window_lines
    };

    let (symbol, containing_body) = if query.pre_fetch {
        if let Some((sym_name, body)) =
            find_containing_symbol_rich(&file_content, query.line, language)
        {
            (Some(sym_name), Some(body))
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    let category = error_code_to_category(query.error_code);
    let message = query.error_message.unwrap_or("").to_string();
    let retry_prompt = build_retry_prompt(
        query.error_code,
        &message,
        &surrounding_lines,
        symbol.as_deref(),
    );
    let strategies = get_strategies(query.error_code, &message);
    let similar_failures = count_similar_failures(query.file_path, query.error_code);
    let suggested_approach = strategies
        .first()
        .map_or_else(|| "manual".to_string(), |s| s.name.clone());

    Ok(RecallContext {
        recall_id: format!("recall-{}", chrono::Utc::now().format("%Y-%m-%d-%H%M%S")),
        file: query.file_path.to_string_lossy().to_string(),
        attempt: AttemptInfo {
            line: query.line,
            old: query.old_content.to_string(),
            new: query.new_content.to_string(),
            strategy_used: None,
            confidence: None,
        },
        error: ErrorInfo {
            code: query.error_code.to_string(),
            category: category.to_string(),
            message,
            retry_prompt,
        },
        context: ContextInfo {
            surrounding_lines,
            symbol,
            language: query
                .file_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string(),
            containing_body,
        },
        strategies,
        history: HistoryInfo {
            similar_failures,
            suggested_approach,
        },
        strategy_used: None,
    })
}

///
/// # Errors
///
/// Returns an error if the file or record cannot be read.
pub fn generate_recall_context_with_session(
    query: &RecallQuery<'_>,
    language: &mut dyn Language,
) -> Result<RecallContext> {
    let mut ctx = generate_recall_context(query, language)?;
    if let Some(sid) = query.session_id
        && let Ok(session) = Session::open(sid)
    {
        let attempts = session.list_attempts().unwrap_or_default();
        ctx.history.similar_failures = attempts
            .iter()
            .filter(|a| a.error_code == query.error_code)
            .count();
        for s in &mut ctx.strategies {
            let successes = attempts
                .iter()
                .filter(|a| a.strategy_used.as_deref() == Some(&s.name) && a.success)
                .count();
            let total = attempts
                .iter()
                .filter(|a| a.strategy_used.as_deref() == Some(&s.name))
                .count();
            if total > 0 {
                s.score = Some(crate::num::to_f64(successes) / crate::num::to_f64(total));
            }
        }
    }
    if let Some(max) = query.max_tokens {
        ctx = enforce_token_budget(ctx, max);
    }
    Ok(ctx)
}

#[must_use]
pub fn build_minimal_context(
    _file_path: &Path,
    line: usize,
    error_code: &str,
    error_message: Option<&str>,
    file_content: &str,
) -> MinimalRecallContext {
    let target_line = file_content
        .lines()
        .nth(line.saturating_sub(1))
        .unwrap_or("")
        .to_string();
    let strategies = get_strategies(error_code, error_message.unwrap_or(""));
    let best_strategy = strategies.first().map_or_else(
        || "Manual inspection".to_string(),
        |s| s.description.clone(),
    );
    MinimalRecallContext {
        error: error_code.to_string(),
        message: error_message.unwrap_or("").to_string(),
        line: target_line,
        best_strategy,
    }
}

#[must_use]
pub fn enforce_token_budget(ctx: RecallContext, max_tokens: usize) -> RecallContext {
    let max_chars = max_tokens * 4;
    let mut ctx = ctx;
    if serde_json::to_string(&ctx).unwrap_or_default().len() <= max_chars {
        return ctx;
    }
    let mut new_lines = Vec::new();
    let mut total = 0;
    for line in &ctx.context.surrounding_lines {
        total += line.len();
        if total > max_chars {
            break;
        }
        new_lines.push(line.clone());
    }
    ctx.context.surrounding_lines = new_lines;
    ctx.strategies.truncate(3);
    ctx
}

fn find_containing_symbol(
    _source: &str,
    _line: usize,
    _language: &mut dyn Language,
) -> Option<(String, usize)> {
    None
}

fn find_containing_symbol_rich(
    source: &str,
    line: usize,
    language: &mut dyn Language,
) -> Option<(String, String)> {
    let (name, _) = find_containing_symbol(source, line, language)?;
    let _parse_result = language.parse(source);
    if let Some(rust_lang) = language
        .as_any_mut()
        .downcast_ref::<crate::ast::RustLanguage>()
    {
        let (start, end) = rust_lang.find_function_body_range(source, &name)?;
        let body = source[start..end].to_string();
        let truncated: String = body.lines().take(20).collect::<Vec<_>>().join("\n");
        return Some((
            name,
            if truncated.len() > 500 {
                format!("{}...", &truncated[..500])
            } else {
                truncated
            },
        ));
    }
    Some((name, String::new()))
}

fn count_similar_failures(file_path: &Path, error_code: &str) -> usize {
    let recall_file = std::path::PathBuf::from(".patch-ts/recall.jsonl");
    if !recall_file.exists() {
        return 0;
    }
    let content = std::fs::read_to_string(&recall_file).unwrap_or_default();
    let file_name = file_path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    content
        .lines()
        .filter(|l| {
            if let Ok(rec) = serde_json::from_str::<serde_json::Value>(l) {
                rec.get("error_code").and_then(|e| e.as_str()) == Some(error_code)
                    && rec
                        .get("file")
                        .and_then(|f| f.as_str())
                        .is_some_and(|f| f.contains(file_name))
            } else {
                false
            }
        })
        .count()
}

/// Return the strategy with best historical success rate for an error code.
#[must_use]
pub fn get_best_strategy(error_code: &str) -> Option<String> {
    let recall_file = std::path::PathBuf::from(".patch-ts/recall.jsonl");
    if !recall_file.exists() {
        return None;
    }
    let content = std::fs::read_to_string(&recall_file).unwrap_or_default();
    let mut strategy_stats: std::collections::HashMap<String, (usize, usize)> =
        std::collections::HashMap::new();
    for line in content.lines() {
        if let Ok(rec) = serde_json::from_str::<serde_json::Value>(line)
            && rec.get("error_code").and_then(|e| e.as_str()) == Some(error_code)
            && let Some(strat) = rec.get("strategy_used").and_then(|s| s.as_str())
        {
            let entry = strategy_stats.entry(strat.to_string()).or_insert((0, 0));
            entry.1 += 1;
            if rec.get("success").and_then(serde_json::Value::as_bool) == Some(true) {
                entry.0 += 1;
            }
        }
    }
    strategy_stats
        .into_iter()
        .max_by(|(_, (s1, t1)), (_, (s2, t2))| {
            let r1 = if *t1 > 0 {
                crate::num::to_f64(*s1) / crate::num::to_f64(*t1)
            } else {
                0.0
            };
            let r2 = if *t2 > 0 {
                crate::num::to_f64(*s2) / crate::num::to_f64(*t2)
            } else {
                0.0
            };
            r1.partial_cmp(&r2).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(name, _)| name)
}

///
/// # Panics
///
/// Panics if an internal `unwrap` fails (invariant violation).
pub fn record_recall(
    recall_id: &str,
    file: &str,
    error_code: &str,
    success: bool,
    strategy_used: Option<&str>,
    agent: Option<&str>,
    model: Option<&str>,
) {
    let record = serde_json::json!({"recall_id": recall_id, "file": file, "error_code": error_code, "success": success, "strategy_used": strategy_used, "agent": agent, "model": model, "timestamp": chrono::Utc::now().to_rfc3339()});
    let dir = std::path::PathBuf::from(".patch-ts");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("recall.jsonl");
    let mut existing = std::fs::read_to_string(&path).unwrap_or_default();
    existing.push_str(&serde_json::to_string(&record).unwrap());
    existing.push('\n');
    let _ = std::fs::write(&path, existing);
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
        let mut query = RecallQuery::new(&file_path, 2, "let x = 1;", "let x = 2;", "E002");
        query.error_message = Some("expected line 2");
        query.context_lines = 2;
        query.use_entropy = true;
        let ctx = generate_recall_context(&query, &mut lang).unwrap();
        assert_eq!(ctx.attempt.line, 2);
        assert_eq!(ctx.error.code, "E002");
        assert!(!ctx.strategies.is_empty());
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

    #[test]
    fn test_entropy_calculation() {
        let high = compute_line_entropy("fn main() { let x = 42; println!(\"{}\", x); }");
        let low = compute_line_entropy("//////////////////////////////");
        assert!(high > low);
    }

    #[test]
    fn test_entropy_selection() {
        let lines: Vec<String> = vec![
            "////".into(),
            "// comment".into(),
            "fn main() { let port = 3000; }".into(),
            "    server.start(host, port);".into(),
            "////".into(),
        ];
        let selected = select_high_entropy_lines(&lines, 2, 3, 2.5);
        assert!(selected.contains(&2));
        assert!(selected.len() <= 3);
    }
}
