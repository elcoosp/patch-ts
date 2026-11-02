use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct ReliabilityScore {
    pub overall: u8,
    pub dimensions: HashMap<String, u8>,
}

#[derive(Debug, Clone)]
pub struct ScoreContext {
    pub syntax_valid: bool,
    pub compile_success: bool,
    pub confidence: f64,
    pub uniqueness_score: f64,
    pub cross_file_impact: usize, // number of callers affected
    pub historical_success_rate: f64,
}

/// Calculate a multi‑dimensional reliability score.
#[must_use]
pub fn calculate_score(context: &ScoreContext) -> ReliabilityScore {
    let syntax = if context.syntax_valid { 100 } else { 0 };
    let compile = if context.compile_success { 100 } else { 0 };
    let confidence = crate::num::percent(context.confidence);
    let uniqueness = crate::num::percent(context.uniqueness_score);
    let cross_file = if context.cross_file_impact == 0 {
        100
    } else {
        crate::num::to_u8(100usize.saturating_sub(context.cross_file_impact.saturating_mul(20)))
    };
    let historical = crate::num::percent(context.historical_success_rate);
    // Weighted average: syntax (20%), compile (25%), confidence (15%), uniqueness (10%), cross‑file (20%), historical (10%)
    let weighted = (f64::from(syntax) * 0.20)
        + (f64::from(compile) * 0.25)
        + (f64::from(confidence) * 0.15)
        + (f64::from(uniqueness) * 0.10)
        + (f64::from(cross_file) * 0.20)
        + (f64::from(historical) * 0.10);
    let overall = crate::num::percent(weighted / 100.0);

    let mut dimensions = HashMap::new();
    dimensions.insert("syntax".to_string(), syntax);
    dimensions.insert("compile".to_string(), compile);
    dimensions.insert("confidence".to_string(), confidence);
    dimensions.insert("uniqueness".to_string(), uniqueness);
    dimensions.insert("cross_file_impact".to_string(), cross_file);
    dimensions.insert("historical".to_string(), historical);

    ReliabilityScore {
        overall,
        dimensions,
    }
}

/// Helper to compute historical success rate from provenance records.
#[must_use]
pub fn historical_success_rate() -> f64 {
    let path = Path::new(".patch-ts").join("provenance.jsonl");
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let mut total = 0usize;
    let mut successes = 0usize;
    for line in content.lines() {
        if let Ok(record) = serde_json::from_str::<serde_json::Value>(line) {
            total += 1;
            if let Some(results) = record.get("validation_results")
                && results.get("syntax").and_then(serde_json::Value::as_bool) == Some(true)
                && results.get("compile").and_then(serde_json::Value::as_bool) == Some(true)
            {
                successes += 1;
            }
        }
    }
    if total == 0 {
        0.5
    } else {
        crate::num::to_f64(successes) / crate::num::to_f64(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_score() {
        let ctx = ScoreContext {
            syntax_valid: true,
            compile_success: true,
            confidence: 1.0,
            uniqueness_score: 1.0,
            cross_file_impact: 0,
            historical_success_rate: 1.0,
        };
        let score = calculate_score(&ctx);
        assert!(score.overall >= 90);
    }

    #[test]
    fn test_risky_patch() {
        let ctx = ScoreContext {
            syntax_valid: true,
            compile_success: true,
            confidence: 0.7,
            uniqueness_score: 0.3,
            cross_file_impact: 3,
            historical_success_rate: 0.5,
        };
        let score = calculate_score(&ctx);
        assert!(score.overall < 75);
    }
}
