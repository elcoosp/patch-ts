use crate::ast::{DelimiterError, Language};
use crate::repair::search::{RepairAction, minimum_cost_repair};
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

///
/// # Errors
///
/// Returns an error if the patch cannot be applied to the file.
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
            heuristic_repair(
                &original,
                &errors,
                language,
                options.max_cost,
                &options.heuristic,
            )
        }
        Heuristic::Balanced => minimum_cost_repair(&original, &errors, language, options.max_cost)
            .unwrap_or_else(|| (original.clone(), vec![], 0)),
    };

    if options.apply && !actions.is_empty() {
        std::fs::write(file_path, &repaired)?;
    }

    Ok(HealResult {
        success: !errors.is_empty() && !actions.is_empty(),
        actions: actions.clone(),
        repaired_content: if repaired == original {
            None
        } else {
            Some(repaired)
        },
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
        if let Some(actions) =
            crate::repair::history::query_repair_history(&error_pattern, "rs", 100)
        {
            let mut candidate = content.to_string();
            for action in &actions {
                match action {
                    RepairAction::Insert { ch, pos } => {
                        if *pos <= candidate.len() {
                            candidate.insert(*pos, *ch);
                        }
                    }
                    RepairAction::Delete { start, end: _ } => {
                        if *start < candidate.len() {
                            candidate.remove(*start);
                        }
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
        let weighted_errors: Vec<DelimiterError> = errors
            .iter()
            .map(|e| match e {
                DelimiterError::Missing {
                    expected,
                    insert_at,
                    parent_kind,
                } => DelimiterError::Missing {
                    expected: *expected,
                    insert_at: insert_at.clone(),
                    parent_kind: parent_kind.clone(),
                },
                DelimiterError::Extra { .. } => e.clone(),
            })
            .collect();
        if let Some((repaired, actions, cost)) =
            minimum_cost_repair(content, &weighted_errors, language, max_cost)
        {
            return (repaired, actions, cost);
        }
    }

    // Fallback to BFS
    if let Some((repaired, actions, cost)) =
        minimum_cost_repair(content, errors, language, max_cost)
    {
        return (repaired, actions, cost);
    }
    (content.to_string(), vec![], 0)
}

/// Describe the error pattern for history lookup.
#[must_use]
pub fn describe_errors(errors: &[DelimiterError]) -> String {
    let mut missing = 0;
    let mut extra = 0;
    for e in errors {
        match e {
            DelimiterError::Missing { .. } => missing += 1,
            DelimiterError::Extra { .. } => extra += 1,
        }
    }
    format!("missing:{missing} extra:{extra}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::RustLanguage;
    use tempfile::tempdir;

    fn broken_file(dir: &std::path::Path) -> std::path::PathBuf {
        let path = dir.join("broken.rs");
        std::fs::write(&path, "fn main() {\n").unwrap();
        path
    }

    #[test]
    fn heal_already_valid_file_is_noop() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("ok.rs");
        std::fs::write(&path, "fn main() {}\n").unwrap();
        let mut lang = RustLanguage::new();
        let options = HealOptions {
            max_cost: 3,
            heuristic: Heuristic::Balanced,
            apply: true,
            incremental: false,
        };
        let result = heal_file(&path, &options, &mut lang).unwrap();
        assert!(result.success);
        assert_eq!(result.actions.len(), 0);
        assert!(result.repaired_content.is_none());
        assert_eq!(result.total_cost, 0);
        assert!(result.heuristic_used.contains("already valid"));
    }

    #[test]
    fn heal_balanced_heuristic_repairs_and_applies() {
        let dir = tempdir().unwrap();
        let path = broken_file(dir.path());
        let mut lang = RustLanguage::new();
        let options = HealOptions {
            max_cost: 3,
            heuristic: Heuristic::Balanced,
            apply: true,
            incremental: false,
        };
        let result = heal_file(&path, &options, &mut lang).unwrap();
        assert!(result.success);
        assert_eq!(result.actions.len(), 1);
        assert!(result.repaired_content.is_some());
        let healed = std::fs::read_to_string(&path).unwrap();
        assert!(healed.contains("fn main() {}"));
        assert_eq!(result.heuristic_used, "balanced");
    }

    #[test]
    fn heal_language_aware_heuristic_without_apply() {
        let dir = tempdir().unwrap();
        let path = broken_file(dir.path());
        let mut lang = RustLanguage::new();
        let options = HealOptions {
            max_cost: 3,
            heuristic: Heuristic::LanguageAware,
            apply: false,
            incremental: false,
        };
        let result = heal_file(&path, &options, &mut lang).unwrap();
        assert!(result.success);
        assert!(result.repaired_content.is_some());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "fn main() {\n");
    }

    #[test]
    fn heal_cost_weighted_and_history_guided_heuristics() {
        for heuristic in [Heuristic::CostWeighted, Heuristic::HistoryGuided] {
            let dir = tempdir().unwrap();
            let path = broken_file(dir.path());
            let mut lang = RustLanguage::new();
            let options = HealOptions {
                max_cost: 3,
                heuristic,
                apply: false,
                incremental: false,
            };
            let result = heal_file(&path, &options, &mut lang).unwrap();
            assert!(
                result.success,
                "heuristic {:?} should repair",
                options.heuristic
            );
            assert!(result.repaired_content.is_some());
        }
    }

    #[test]
    fn describe_errors_lists_missing_and_extra() {
        let span = crate::ast::Span {
            start_byte: 0,
            end_byte: 1,
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 2,
        };
        let errors = vec![
            DelimiterError::Extra {
                span,
                delimiter: ')',
            },
            DelimiterError::Missing {
                expected: '}',
                insert_at: crate::ast::Span {
                    start_byte: 5,
                    end_byte: 5,
                    start_line: 1,
                    start_column: 6,
                    end_line: 1,
                    end_column: 6,
                },
                parent_kind: None,
            },
        ];
        let described = describe_errors(&errors);
        assert!(described.contains("missing:1"), "{described}");
        assert!(described.contains("extra:1"), "{described}");
    }
}
