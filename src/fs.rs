use anyhow::Result;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Abstraction over file system operations for cross‑platform compatibility.
pub trait FileSystem: Send + Sync {
    ///
    /// # Errors
    ///
    /// Returns an error if the file or record cannot be read.
    fn read_to_string(&self, path: &Path) -> Result<String>;
    ///
    /// # Errors
    ///
    /// Returns an error if the file or record cannot be written.
    fn write(&self, path: &Path, content: &str) -> Result<()>;
    fn exists(&self, path: &Path) -> bool;
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    fn create_dir_all(&self, path: &Path) -> Result<()>;
    ///
    /// # Errors
    ///
    /// Returns an error if the file or record cannot be read.
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            files: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl FileSystem for VirtualFileSystem {
    fn read_to_string(&self, path: &Path) -> Result<String> {
        let map = self
            .files
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {e}"))?;
        map.get(path)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("File not found: {}", path.display()))
    }

    fn write(&self, path: &Path, content: &str) -> Result<()> {
        let mut map = self
            .files
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock error: {e}"))?;
        map.insert(path.to_path_buf(), content.to_string());
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.lock().is_ok_and(|m| m.contains_key(path))
    }

    fn create_dir_all(&self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn read_dir(&self, _path: &Path) -> Result<Vec<PathBuf>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn std_file_system_roundtrip() {
        let fs = StdFileSystem;
        let dir = tempdir().unwrap();
        let path = dir.path().join("file.txt");
        assert!(!fs.exists(&path));
        fs.write(&path, "hello").unwrap();
        assert!(fs.exists(&path));
        assert_eq!(fs.read_to_string(&path).unwrap(), "hello");
        let sub = dir.path().join("nested");
        fs.create_dir_all(&sub).unwrap();
        fs.write(&sub.join("inner.txt"), "x").unwrap();
        let entries = fs.read_dir(&sub).unwrap();
        assert_eq!(entries.len(), 1);
        assert!(fs.read_to_string(&path.join("missing")).is_err());
    }

    #[test]
    fn virtual_file_system_roundtrip() {
        let vfs = VirtualFileSystem::new();
        let path = std::path::Path::new("/virtual/file.txt");
        assert!(!vfs.exists(path));
        vfs.write(path, "content").unwrap();
        assert!(vfs.exists(path));
        assert_eq!(vfs.read_to_string(path).unwrap(), "content");
        assert!(vfs.read_to_string(std::path::Path::new("/nope")).is_err());
        assert_eq!(vfs.read_dir(path).unwrap().len(), 0);
        vfs.create_dir_all(path).unwrap();
    }
}
