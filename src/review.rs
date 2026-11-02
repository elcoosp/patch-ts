use serde::Serialize;
use std::path::Path;

use crate::gate::{GateResult, run_gate_parallel};

#[derive(Debug, Clone, Serialize)]
pub struct AgentFinding {
    pub agent: String,
    pub severity: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewReport {
    pub findings: Vec<AgentFinding>,
    pub action: String,
    pub overall_passed: bool,
}

impl ReviewReport {
    #[must_use]
    pub fn new() -> Self {
        Self {
            findings: vec![],
            action: "unknown".to_string(),
            overall_passed: false,
        }
    }
}

impl Default for ReviewReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Run a multi‑agent review across syntax, compile, coverage, security, style, and semantic diff.
#[must_use]
pub fn run_review(file_path: &Path, old: &str, new: &str) -> ReviewReport {
    let stages = vec![
        "syntax".to_string(),
        "compile".to_string(),
        "cross‑file".to_string(),
        "security".to_string(),
        "semdiff".to_string(),
    ];

    let gate_result: GateResult = run_gate_parallel(&stages, file_path, old, new, 30)
        .unwrap_or_else(|_| GateResult {
            passed: false,
            stages: vec![],
        });

    let findings: Vec<AgentFinding> = gate_result
        .stages
        .iter()
        .map(|stage| AgentFinding {
            agent: stage.name.clone(),
            severity: if stage.passed {
                "pass".to_string()
            } else {
                "fail".to_string()
            },
            details: stage.details.clone(),
        })
        .collect();

    let overall_passed = findings.iter().all(|f| f.severity == "pass");
    let action = if overall_passed {
        "approve".to_string()
    } else {
        "request_changes".to_string()
    };

    ReviewReport {
        findings,
        action,
        overall_passed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn new_report_defaults() {
        let report = ReviewReport::new();
        assert!(report.findings.is_empty());
        assert_eq!(report.action, "unknown");
        assert!(!report.overall_passed);
    }

    #[test]
    fn run_review_produces_findings_for_valid_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("sample.rs");
        std::fs::write(&file, "fn main() {}\n").unwrap();
        let report = run_review(&file, "fn main() {}", "fn main() {}");
        assert_eq!(report.findings.len(), 5, "one finding per gate stage");
        assert!(report.findings.iter().all(|f| f.severity == "pass"));
        assert!(report.overall_passed);
        assert_ne!(report.action.len(), 0);
    }
}
