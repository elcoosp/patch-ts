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

/// Main entry point for intelligently repairing delimiter errors.
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

    let (repaired, actions, cost) = match options.heuristic {
        Heuristic::LanguageAware | Heuristic::CostWeighted | Heuristic::HistoryGuided => {
            heuristic_repair(&original, &errors, language, options.max_cost)
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

/// Heuristic‑driven repair: currently delegates to BFS with max_cost,
/// but can be extended with language‑aware pruning in Sprint 2.
fn heuristic_repair(
    content: &str,
    errors: &[DelimiterError],
    language: &mut dyn Language,
    max_cost: usize,
) -> (String, Vec<RepairAction>, usize) {
    if let Some((repaired, actions, cost)) = minimum_cost_repair(content, errors, language, max_cost) {
        return (repaired, actions, cost);
    }
    (content.to_string(), vec![], 0)
}
