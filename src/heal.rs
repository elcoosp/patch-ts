pub struct HealContext {
    pub file: String,
    pub line: usize,
    pub expected: String,
    pub actual: String,
    pub confidence: f64,
    pub threshold: f64,
    pub strategy: Option<String>,
    pub context_lines: Vec<String>,
    pub error_code: String,
}

pub fn generate_feedback(error_msg: &str, category: &str, context: &HealContext) -> String {
    let mut parts = Vec::new();
    parts.push(format!("Error code: {}", context.error_code));
    parts.push(format!("File: {}, Line: {}", context.file, context.line));
    parts.push(format!("Confidence: {:.2} (threshold: {:.2})", context.confidence, context.threshold));
    if let Some(ref s) = context.strategy {
        parts.push(format!("Strategy attempted: {}", s));
    }
    if !context.expected.is_empty() {
        parts.push(format!("Expected: `{}`", context.expected));
    }
    if !context.actual.is_empty() {
        parts.push(format!("Actual: `{}`", context.actual));
    }
    if !context.context_lines.is_empty() {
        parts.push("Surrounding context:".to_string());
        for line in &context.context_lines {
            parts.push(format!("  {}", line));
        }
    }
    parts.push(format!("Suggestion: {}", match category {
        "content_mismatch" => format!(
            "At line {} of {}, replace `{}` with the new content.",
            context.line, context.file, context.expected
        ),
        "compilation_error" => format!("Fix these compilation errors before retrying: {}", error_msg),
        "confidence_below_threshold" => format!(
            "The match confidence ({:.2}) is below the threshold ({:.2}). Provide more context or increase --fuzz.",
            context.confidence, context.threshold
        ),
        "syntax_error" => "Run `patch-ts balance` to attempt automatic fix.".to_string(),
        _ => error_msg.to_string(),
    }));
    parts.join("\n")
}

/// Map error categories to machine‑readable error codes.
pub fn error_code_for_category(category: &str) -> &str {
    match category {
        "file_not_found" => "E001",
        "content_mismatch" => "E002",
        "confidence_below_threshold" => "E003",
        "ambiguous_match" => "E004",
        "syntax_error" => "E005",
        "compilation_error" => "E006",
        _ => "E000",
    }
}
