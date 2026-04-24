use anyhow::Result;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Abstraction over file system operations for cross‑platform compatibility.
pub trait FileSystem: Send + Sync {
    fn read_to_string(&self, path: &Path) -> Result<String>;
    fn write(&self, path: &Path, content: &str) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>>;
}

/// Implementation using the standard library `std::fs`.
#[derive(Default)]
pub struct StdFileSystem;

impl FileSystem for StdFileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        let mut content = String::new();
        std::fs::File::open(path)?.read_to_string(&mut content)?;
        Ok(content)
    }

    fn write(&self, path: &Path, content: &str) -> Result<()> {
        std::fs::write(path, content)?;
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        Ok(())
    }

    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let entries: Vec<PathBuf> = std::fs::read_dir(path)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        Ok(entries)
    }
}

/// In‑memory file system for WASM environments (browser sandboxes).
#[derive(Default)]
pub struct VirtualFileSystem {
    files: std::sync::Mutex<std::collections::HashMap<PathBuf, String>>,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        Self {
            files: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl FileSystem for VirtualFileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        let map = self.files.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        map.get(path).cloned().ok_or_else(|| anyhow::anyhow!("File not found: {}", path.display()))
    }

    fn write(&self, path: &Path, content: &str) -> Result<()> {
        let mut map = self.files.lock().map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
        map.insert(path.to_path_buf(), content.to_string());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.lock().map(|m| m.contains_key(path)).unwrap_or(false)
    }

    fn create_dir_all(&self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn read_dir(&self, _path: &Path) -> Result<Vec<PathBuf>> {
        Ok(vec![])
    }
}
