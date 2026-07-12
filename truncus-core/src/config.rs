use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub token: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing config at {0}: set TRUNCUS_URL/TRUNCUS_TOKEN or run `truncus install`")]
    Missing(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse: {0}")]
    Parse(#[from] toml::de::Error),
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        if let (Ok(url), Ok(token)) = (std::env::var("TRUNCUS_URL"), std::env::var("TRUNCUS_TOKEN"))
        {
            return Ok(Self { url, token });
        }
        let path = Self::path();
        let raw = std::fs::read_to_string(&path)
            .map_err(|_| ConfigError::Missing(path.display().to_string()))?;
        Ok(toml::from_str(&raw)?)
    }

    pub fn save(&self) -> Result<PathBuf, ConfigError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let body = toml::to_string_pretty(self).expect("config serializes");
        std::fs::write(&path, body)?;
        Ok(path)
    }

    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("truncus")
            .join("config.toml")
    }
}
