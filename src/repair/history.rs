use serde::{Deserialize, Serialize};
use crate::repair::search::RepairAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairRecord {
    pub error_pattern: String,
    pub language: String,
    pub repair_sequence: Vec<RepairAction>,
    pub success: bool,
    pub timestamp: String,
    pub agent: Option<String>,
    pub model: Option<String>,
}

/// Record a successful repair to the history file.
pub fn record_repair_success(
    error_pattern: &str,
    language: &str,
    actions: &[RepairAction],
    agent: Option<&str>,
    model: Option<&str>,
) {
    let record = RepairRecord {
        error_pattern: error_pattern.to_string(),
        language: language.to_string(),
        repair_sequence: actions.to_vec(),
        success: true,
        timestamp: chrono::Utc::now().to_rfc3339(),
        agent: agent.map(|s| s.to_string()),
        model: model.map(|s| s.to_string()),
    };
    let dir = std::path::PathBuf::from(".patch-ts");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("repair-history.jsonl");
    let mut existing = std::fs::read_to_string(&path).unwrap_or_default();
    existing.push_str(&serde_json::to_string(&record).unwrap());
    existing.push('\n');
    let _ = std::fs::write(&path, &existing);
    // Prune if > 10,000 lines
    let lines: Vec<&str> = existing.lines().collect();
    if lines.len() > 10000 {
        let trimmed = lines[lines.len() - 10000..].join("\n");
        let _ = std::fs::write(&path, trimmed);
    }
}

/// Query repair history for a given error pattern, returning the most frequent successful repair sequence.
pub fn query_repair_history(
    error_pattern: &str,
    _language: &str,
    limit: usize,
) -> Option<Vec<RepairAction>> {
    let path = std::path::PathBuf::from(".patch-ts/repair-history.jsonl");
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let mut sequences: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for line in content.lines().take(limit) {
        if let Ok(rec) = serde_json::from_str::<RepairRecord>(line) {
            if rec.error_pattern.contains(error_pattern) && rec.success {
                let key = serde_json::to_string(&rec.repair_sequence).unwrap_or_default();
                *sequences.entry(key).or_insert(0) += 1;
            }
        }
    }
    let best = sequences.into_iter().max_by_key(|(_, count)| *count);
    best.and_then(|(json, _)| serde_json::from_str(&json).ok())
}
