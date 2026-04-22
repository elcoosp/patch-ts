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

pub fn load_config(start_path: &Path) -> Result<Config> {
    let config_path = find_config_file(start_path)?;
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

fn find_config_file(start_path: &Path) -> Result<Option<PathBuf>> {
    let mut current = if start_path.is_dir() {
        start_path.to_path_buf()
    } else {
        start_path.parent().unwrap_or(Path::new(".")).to_path_buf()
    };

    loop {
        let config_file = current.join("patch-ts.toml");
        if config_file.exists() {
            return Ok(Some(config_file));
        }
        if !current.pop() {
            break;
        }
    }
    Ok(None)
}
