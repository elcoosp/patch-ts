use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub fuzz: Option<usize>,
    pub backup: Option<bool>,
    pub auto_repair: Option<bool>,
    pub similarity_threshold: Option<f64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fuzz: Some(5),
            backup: Some(true),
            auto_repair: Some(true),
            similarity_threshold: Some(0.9),
        }
    }
}

///
/// # Errors
///
/// Returns an error if the file or record cannot be read.
pub fn load_config(start_path: &Path) -> Result<Config> {
    let config_path = find_config_file(start_path);
    if let Some(path) = config_path {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

fn find_config_file(start_path: &Path) -> Option<PathBuf> {
    let mut current = if start_path.is_dir() {
        start_path.to_path_buf()
    } else {
        start_path.parent().unwrap_or(Path::new(".")).to_path_buf()
    };

    loop {
        let config_file = current.join("patch-ts.toml");
        if config_file.exists() {
            return Some(config_file);
        }
        if !current.pop() {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn load_config_returns_default_when_missing() {
        let dir = tempdir().unwrap();
        let config = load_config(dir.path()).unwrap();
        let default = Config::default();
        assert_eq!(config.fuzz, default.fuzz);
        assert_eq!(config.backup, default.backup);
        assert_eq!(config.similarity_threshold, default.similarity_threshold);
    }

    #[test]
    fn load_config_reads_toml_and_walks_up() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("patch-ts.toml"),
            "fuzz = 7\nbackup = false\n",
        )
        .unwrap();
        let nested = dir.path().join("deep").join("deeper");
        fs::create_dir_all(&nested).unwrap();
        let config = load_config(&nested).unwrap();
        assert_eq!(config.fuzz, Some(7));
        assert_eq!(config.backup, Some(false));
    }

    #[test]
    fn load_config_reports_invalid_toml() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("patch-ts.toml");
        fs::write(&file, "fuzz = not a number\n").unwrap();
        assert!(load_config(&file).is_err());
    }
}
