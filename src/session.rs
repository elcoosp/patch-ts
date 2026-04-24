use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub attempt_number: usize,
    pub timestamp: String,
    pub strategy_used: Option<String>,
    pub success: bool,
    pub error_code: String,
    pub file: String,
    pub token_count: Option<usize>,
}

pub struct Session {
    id: String,
    dir: PathBuf,
}

impl Session {
    /// Open or create a session by ID.
    pub fn open(session_id: &str) -> Result<Self> {
        let dir = PathBuf::from(".patch-ts/sessions");
        std::fs::create_dir_all(&dir)?;
        Ok(Self {
            id: session_id.to_string(),
            dir,
        })
    }

    /// Append a new record.
    pub fn record_attempt(&self, record: &SessionRecord) -> Result<()> {
        let path = self.dir.join(format!("{}.jsonl", self.id));
        let mut existing = std::fs::read_to_string(&path).unwrap_or_default();
        existing.push_str(&serde_json::to_string(record)?);
        existing.push('\n');
        std::fs::write(&path, existing)?;
        Ok(())
    }

    /// Read all attempts in this session.
    pub fn list_attempts(&self) -> Result<Vec<SessionRecord>> {
        let path = self.dir.join(format!("{}.jsonl", self.id));
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let records = content
            .lines()
            .filter_map(|line| serde_json::from_str::<SessionRecord>(line).ok())
            .collect();
        Ok(records)
    }

    /// Garbage collect old sessions (> 7 days).
    pub fn prune_old() -> Result<()> {
        let dir = PathBuf::from(".patch-ts/sessions");
        if !dir.exists() {
            return Ok(());
        }
        let now = chrono::Utc::now();
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if let Ok(meta) = entry.metadata() {
                if let Ok(modified) = meta.modified() {
                    let age = now.signed_duration_since(chrono::DateTime::<chrono::Utc>::from(modified));
                    if age.num_days() > 7 {
                        std::fs::remove_file(entry.path()).ok();
                    }
                }
            }
        }
        Ok(())
    }
}
