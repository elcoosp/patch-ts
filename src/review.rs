use serde::Serialize;
use std::path::Path;

use crate::gate::{run_gate_parallel, GateResult};

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
    pub fn new() -> Self {
        Self { findings: vec![], action: "unknown".to_string(), overall_passed: false }
    }
}

/// Run a multi‑agent review across syntax, compile, coverage, security, and style.
pub fn run_review(file_path: &Path, old: &str, new: &str) -> ReviewReport {
    let stages = vec![
        "syntax".to_string(),
        "compile".to_string(),
        "cross‑file".to_string(),
        "security".to_string(),
        "style".to_string(),
    ];

    let gate_result: GateResult = run_gate_parallel(&stages, file_path, old, new, 30)
        .unwrap_or_else(|_| GateResult { passed: false, stages: vec![] });

    let findings: Vec<AgentFinding> = gate_result.stages.iter().map(|stage| {
        AgentFinding {
            agent: stage.name.clone(),
            severity: if stage.passed { "pass".to_string() } else { "fail".to_string() },
            details: stage.details.clone(),
        }
    }).collect();

    let overall_passed = findings.iter().all(|f| f.severity == "pass");
    let action = if overall_passed { "approve".to_string() } else { "request_changes".to_string() };

    ReviewReport { findings, action, overall_passed }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_run_review_simple() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        fs::write(&file_path, "fn main() {}").unwrap();

        let report = run_review(&file_path, "fn main() {}", "fn main() {}");
        assert!(report.findings.iter().any(|f| f.agent == "syntax"));
        assert!(report.findings.iter().any(|f| f.agent == "compile"));
    }
}
