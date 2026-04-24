// src/repair/search.rs
use crate::ast::{DelimiterError, Language};
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

/// An atomic edit action: insert a delimiter at a byte position, or delete a span.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RepairAction {
    Insert { ch: char, pos: usize },
    Delete { start: usize, end: usize },
}

/// State in the BFS search: current source string, accumulated cost,
/// and sequence of actions taken to reach this state.
#[derive(Debug, Clone)]
struct RepairState {
    source: String,
    cost: usize,
    actions: Vec<RepairAction>,
}

impl RepairState {
    fn new(source: String) -> Self {
        Self {
            source,
            cost: 0,
            actions: Vec::new(),
        }
    }
}

fn generate_next_states(state: &RepairState, error: &DelimiterError) -> Vec<RepairState> {
    let mut next_states = Vec::new();
    match error {
        DelimiterError::Extra { span, delimiter: _ } => {
            let mut new_source = state.source.clone();
            new_source.replace_range(span.start_byte..span.end_byte, "");
            let mut new_actions = state.actions.clone();
            new_actions.push(RepairAction::Delete {
                start: span.start_byte,
                end: span.end_byte,
            });
            next_states.push(RepairState {
                source: new_source,
                cost: state.cost + 1,
                actions: new_actions,
            });
        }
        DelimiterError::Missing {
            expected,
            insert_at,
            ..
        } => {
            let insert_pos = insert_at.end_byte;
            let mut new_source = state.source.clone();
            new_source.insert(insert_pos, *expected);
            let mut new_actions = state.actions.clone();
            new_actions.push(RepairAction::Insert {
                ch: *expected,
                pos: insert_pos,
            });
            next_states.push(RepairState {
                source: new_source,
                cost: state.cost + 1,
                actions: new_actions,
            });
        }
    }
    next_states
}

/// Attempt to find a minimal-cost sequence of delimiter insertions/deletions
/// that makes the source syntactically valid according to the language's parser.
///
/// Returns `Some((patched_source, actions, cost))` if a repair is found within `max_cost`.
/// Returns `None` if no repair is found.
pub fn minimum_cost_repair(
    source: &str,
    _initial_errors: &[DelimiterError],
    language: &mut dyn Language,
    max_cost: usize,
) -> Option<(String, Vec<RepairAction>, usize)> {
    // Quick check: if already valid, return unchanged.
    let initial_parse = language.parse(source);
    if language.is_valid(&initial_parse) {
        return Some((source.to_string(), Vec::new(), 0));
    }

    let mut queue = VecDeque::new();
    let initial_state = RepairState::new(source.to_string());
    queue.push_back(initial_state);

    let mut visited = HashSet::new();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    source.hash(&mut hasher);
    visited.insert(hasher.finish());

    while let Some(state) = queue.pop_front() {
        let parse_result = language.parse(&state.source);
        if language.is_valid(&parse_result) {
            return Some((state.source, state.actions, state.cost));
        }

        if state.cost >= max_cost {
            continue;
        }

        let errors = language.find_delimiter_errors(&parse_result);
        for error in errors {
            let next_states = generate_next_states(&state, &error);
            for next_state in next_states {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                next_state.source.hash(&mut hasher);
                let hash = hasher.finish();
                if !visited.contains(&hash) {
                    visited.insert(hash);
                    queue.push_back(next_state);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Language, RustLanguage};

    #[test]
    fn test_repair_single_missing_brace() {
        let mut lang = RustLanguage::new();
        let source = "fn main() {";
        let parse_result = lang.parse(source);
        let errors = lang.find_delimiter_errors(&parse_result);
        assert!(!errors.is_empty());

        let result = minimum_cost_repair(source, &errors, &mut lang, 2);
        assert!(result.is_some());
        let (patched, actions, cost) = result.unwrap();
        assert_eq!(patched, "fn main() {}");
        assert_eq!(cost, 1);
        assert_eq!(actions.len(), 1);
        match &actions[0] {
            RepairAction::Insert { ch, pos: _ } => assert_eq!(*ch, '}'),
            _ => panic!("Expected Insert action"),
        }
    }
}
