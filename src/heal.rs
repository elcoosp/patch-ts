pub struct HealFeedback {
    pub category: String,
    pub retry_prompt: String,
}

pub fn generate_feedback(error_msg: &str, category: &str, context: &HealContext) -> HealFeedback {
    let retry = match category {
        "content_mismatch" => format!(
            "At line {} of {}, replace `{}` with the new content. Current line: `{}`.",
            context.line, context.file, context.expected, context.actual
        ),
        "compilation_error" => format!(
            "The patch caused compilation errors: {}. Fix these errors before retrying.",
            error_msg
        ),
        "confidence_below_threshold" => format!(
            "The patch confidence ({:.2}) is below the required threshold ({:.2}). Provide more context or increase --fuzz.",
            context.confidence, context.threshold
        ),
        _ => error_msg.to_string(),
    };
    HealFeedback { category: category.to_string(), retry_prompt: retry }
}

pub struct HealContext {
    pub file: String,
    pub line: usize,
    pub expected: String,
    pub actual: String,
    pub confidence: f64,
    pub threshold: f64,
}
