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
    ///
    /// # Errors
    ///
    /// Returns an error if the file or record cannot be read.
    pub fn open(session_id: &str) -> Result<Self> {
        let dir = PathBuf::from(".patch-ts/sessions");
        std::fs::create_dir_all(&dir)?;
        Ok(Self {
            id: session_id.to_string(),
            dir,
        })
    }

    /// Append a new record.
    ///
    /// # Errors
    ///
    /// Returns an error if the file or record cannot be written.
    pub fn record_attempt(&self, record: &SessionRecord) -> Result<()> {
        let path = self.dir.join(format!("{}.jsonl", self.id));
        let mut existing = std::fs::read_to_string(&path).unwrap_or_default();
        existing.push_str(&serde_json::to_string(record)?);
        existing.push('\n');
        std::fs::write(&path, existing)?;
        Ok(())
    }

    /// Read all attempts in this session.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
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
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub fn prune_old() -> Result<()> {
        let dir = PathBuf::from(".patch-ts/sessions");
        if !dir.exists() {
            return Ok(());
        }
        let now = chrono::Utc::now();
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if let Ok(meta) = entry.metadata()
                && let Ok(modified) = meta.modified()
            {
                let age =
                    now.signed_duration_since(chrono::DateTime::<chrono::Utc>::from(modified));
                if age.num_days() > 7 {
                    std::fs::remove_file(entry.path()).ok();
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cleanup(id: &str) {
        let _ = std::fs::remove_file(
            std::path::Path::new(".patch-ts/sessions").join(format!("{id}.jsonl")),
        );
    }

    #[test]
    fn session_record_and_list_roundtrip() {
        let id = "unit-test-session";
        cleanup(id);
        let session = Session::open(id).unwrap();
        let record = SessionRecord {
            attempt_number: 1,
            timestamp: "2026-01-01T00:00:00Z".to_string(),
            strategy_used: Some("literal".to_string()),
            success: true,
            error_code: String::new(),
            file: "sample.rs".to_string(),
            token_count: Some(120),
        };
        session.record_attempt(&record).unwrap();
        let listed = session.list_attempts().unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].attempt_number, 1);
        assert_eq!(listed[0].file, "sample.rs");
        cleanup(id);
    }

    #[test]
    fn list_attempts_on_missing_session_is_empty() {
        let session = Session::open("no-such-session-unit-test").unwrap();
        assert!(session.list_attempts().unwrap().is_empty());
    }

    #[test]
    fn prune_old_survives_missing_dir() {
        // Runs against the repo-local dir; must not fail either way.
        Session::prune_old().unwrap();
    }
}
