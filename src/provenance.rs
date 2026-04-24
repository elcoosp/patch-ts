use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProvenanceRecord {
    pub timestamp: String,
    pub tool: String,
    pub agent: Option<String>,
    pub model: Option<String>,
    pub file: String,
    pub operation: String,
    pub validation_results: serde_json::Value,
}

impl ProvenanceRecord {
    pub fn new(file: &str, operation: &str, agent: Option<&str>, model: Option<&str>, validation_results: serde_json::Value) -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339(),
            tool: "patch-ts".to_string(),
            agent: agent.map(|s| s.to_string()),
            model: model.map(|s| s.to_string()),
            file: file.to_string(),
            operation: operation.to_string(),
            validation_results,
        }
    }
}

/// Emit a provenance record to .patch‑ts/provenance.jsonl
pub fn emit_record(record: &ProvenanceRecord) -> Result<()> {
    let dir = PathBuf::from(".patch-ts");
    fs::create_dir_all(&dir)?;
    let path = dir.join("provenance.jsonl");
    let mut content = fs::read_to_string(&path).unwrap_or_default();
    let json = serde_json::to_string(record)?;
    content.push_str(&json);
    content.push('\n');
    fs::write(&path, content)?;
    Ok(())
}

/// Query provenance records, optionally filtered by date and file.
pub fn query_provenance(since: Option<&str>, file: Option<&str>) -> Result<Vec<ProvenanceRecord>> {
    let path = PathBuf::from(".patch-ts").join("provenance.jsonl");
    let content = fs::read_to_string(&path).unwrap_or_default();
    let mut records = Vec::new();
    for line in content.lines() {
        if let Ok(record) = serde_json::from_str::<ProvenanceRecord>(line) {
            if let Some(since_str) = since {
                if record.timestamp.as_str() < since_str { continue; }
            }
            if let Some(file_filter) = file {
                if record.file != file_filter { continue; }
            }
            records.push(record);
        }
    }
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::env;

    #[test]
    fn test_emit_and_query() {
        let dir = tempdir().unwrap();
        env::set_current_dir(dir.path()).unwrap();
        let _ = fs::create_dir_all(".patch-ts");
        let record = ProvenanceRecord::new("src/main.rs", "patch", Some("Claude Code"), Some("sonnet"), serde_json::json!({"syntax": true}));
        emit_record(&record).unwrap();
        let records = query_provenance(None, None).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].file, "src/main.rs");
        assert_eq!(records[0].agent.as_deref(), Some("Claude Code"));
    }
}
