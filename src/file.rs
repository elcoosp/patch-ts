use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

pub struct FileManager {
    backup: bool,
}

impl FileManager {
    pub fn new(backup: bool) -> Self {
        Self { backup }
    }

    /// Read entire file to string.
    pub fn read(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path.display()))
    }

    /// Write content atomically using tempfile + rename.
    pub fn write_atomic(&self, path: &Path, content: &str) -> Result<()> {
        // Create backup if enabled
        if self.backup && path.exists() {
            let backup_path = path.with_extension("rs.bak");
            fs::copy(path, &backup_path)
                .with_context(|| format!("Failed to create backup: {}", backup_path.display()))?;
        }

        // Write to temp file in same directory
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let temp_file =
            NamedTempFile::new_in(parent).with_context(|| "Failed to create temp file")?;

        fs::write(temp_file.path(), content).with_context(|| "Failed to write to temp file")?;

        // Persist (atomic rename)
        temp_file
            .persist(path)
            .map_err(|e| anyhow::anyhow!("Failed to persist file: {}", e))?;

        Ok(())
    }
}
