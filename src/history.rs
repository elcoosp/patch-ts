use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchRecord {
    pub timestamp: String,
    pub file: String,
    pub original_content: String,
    pub patched_content: String,
}

pub struct HistoryManager {
    history_dir: PathBuf,
}

impl HistoryManager {
    pub fn new() -> Self {
        let history_dir = PathBuf::from(".patch-ts");
        fs::create_dir_all(&history_dir).ok();
        Self { history_dir }
    }

    pub fn save(&self, file: &str, original: &str, patched: &str) -> Result<()> {
        let record = PatchRecord {
            timestamp: Utc::now().to_rfc3339(),
            file: file.to_string(),
            original_content: original.to_string(),
            patched_content: patched.to_string(),
        };
        let json = serde_json::to_string(&record)?;
        let history_file = self.history_dir.join("history.jsonl");
        let mut content = fs::read_to_string(&history_file).unwrap_or_default();
        content.push_str(&json);
        content.push('\n');
        fs::write(&history_file, content)?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<PatchRecord>> {
        let history_file = self.history_dir.join("history.jsonl");
        let content = fs::read_to_string(&history_file).unwrap_or_default();
        let mut records = Vec::new();
        for line in content.lines() {
            if let Ok(record) = serde_json::from_str::<PatchRecord>(line) {
                records.push(record);
            }
        }
        Ok(records)
    }

    pub fn undo_last(&self) -> Result<Option<PatchRecord>> {
        let mut records = self.list()?;
        if records.is_empty() {
            return Ok(None);
        }
        let last = records.pop().unwrap();
        let history_file = self.history_dir.join("history.jsonl");
        let mut new_content = String::new();
        for record in &records {
            let json = serde_json::to_string(record)?;
            new_content.push_str(&json);
            new_content.push('\n');
        }
        fs::write(&history_file, new_content)?;
        Ok(Some(last))
    }
}
