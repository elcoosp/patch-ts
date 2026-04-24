use crate::ast::{DelimiterError, Language};
use crate::repair::search::{minimum_cost_repair, RepairAction};
use anyhow::Result;
use std::path::Path;

pub struct HealOptions {
    pub max_cost: usize,
    pub heuristic: Heuristic,
    pub apply: bool,
    pub incremental: bool,
}

pub enum Heuristic {
    LanguageAware,
    CostWeighted,
    HistoryGuided,
    Balanced,
}

impl std::fmt::Debug for Heuristic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LanguageAware => write!(f, "language-aware"),
            Self::CostWeighted => write!(f, "cost-weighted"),
            Self::HistoryGuided => write!(f, "history-guided"),
            Self::Balanced => write!(f, "balanced"),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct HealResult {
    pub success: bool,
    pub actions: Vec<RepairAction>,
    pub repaired_content: Option<String>,
    pub total_cost: usize,
    pub heuristic_used: String,
}

pub fn heal_file(
    file_path: &Path,
    options: &HealOptions,
    language: &mut dyn Language,
) -> Result<HealResult> {
    let original = std::fs::read_to_string(file_path)?;
    let parse_result = language.parse(&original);
    let errors = language.find_delimiter_errors(&parse_result);

    if errors.is_empty() {
        return Ok(HealResult {
            success: true,
            actions: vec![],
            repaired_content: None,
            total_cost: 0,
            heuristic_used: "none (already valid)".to_string(),
        });
    }

    let (repaired, actions, cost) = match &options.heuristic {
        Heuristic::LanguageAware | Heuristic::CostWeighted | Heuristic::HistoryGuided => {
            heuristic_repair(&original, &errors, language, options.max_cost, &options.heuristic)
        }
        Heuristic::Balanced => {
            minimum_cost_repair(&original, &errors, language, options.max_cost)
                .map(|(s, a, c)| (s, a, c))
                .unwrap_or_else(|| (original.clone(), vec![], 0))
        }
    };

    if options.apply && !actions.is_empty() {
        std::fs::write(file_path, &repaired)?;
    }

    Ok(HealResult {
        success: !errors.is_empty() && !actions.is_empty(),
        actions: actions.clone(),
        repaired_content: if repaired != original { Some(repaired) } else { None },
        total_cost: cost,
        heuristic_used: format!("{:?}", options.heuristic),
    })
}

// ── Heuristic‑driven repair with cost‑weighted and history‑guided variants ──
fn heuristic_repair(
    content: &str,
    errors: &[DelimiterError],
    language: &mut dyn Language,
    max_cost: usize,
    heuristic: &Heuristic,
) -> (String, Vec<RepairAction>, usize) {
    // History‑guided: try to reuse past successful repairs
    if let Heuristic::HistoryGuided = heuristic {
        let error_pattern = describe_errors(errors);
        if let Some(actions) = crate::repair::history::query_repair_history(&error_pattern, "rs", 100) {
            let mut candidate = content.to_string();
            for action in &actions {
                match action {
                    RepairAction::Insert { ch, pos } => {
                        if *pos <= candidate.len() { candidate.insert(*pos, *ch); }
                    }
                    RepairAction::Delete { start, end: _ } => {
                        if *start < candidate.len() { candidate.remove(*start); }
                    }
                }
            }
            let parse_result = language.parse(&candidate);
            if language.is_valid(&parse_result) {
                return (candidate, actions.clone(), actions.len());
            }
        }
    }

    // Cost‑weighted: prefer insertions that close known blocks
    if let Heuristic::CostWeighted = heuristic {
        let weighted_errors: Vec<DelimiterError> = errors.iter().map(|e| match e {
            DelimiterError::Missing { expected, insert_at, parent_kind } => {
                DelimiterError::Missing { expected: *expected, insert_at: insert_at.clone(), parent_kind: parent_kind.clone() }
            }
            other => other.clone(),
        }).collect();
        if let Some((repaired, actions, cost)) = minimum_cost_repair(content, &weighted_errors, language, max_cost) {
            return (repaired, actions, cost);
        }
    }

    // Fallback to BFS
    if let Some((repaired, actions, cost)) = minimum_cost_repair(content, errors, language, max_cost) {
        return (repaired, actions, cost);
    }
    (content.to_string(), vec![], 0)
}

/// Describe the error pattern for history lookup.
pub fn describe_errors(errors: &[DelimiterError]) -> String {
    let mut missing = 0;
    let mut extra = 0;
    for e in errors {
        match e {
            DelimiterError::Missing { .. } => missing += 1,
            DelimiterError::Extra { .. } => extra += 1,
        }
    }
    format!("missing:{} extra:{}", missing, extra)
}
