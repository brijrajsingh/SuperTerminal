/// Configuration management for SuperTerminal
use crate::error::{Result, SuperTerminalError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration for the SuperTerminal application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// OpenAI API key
    pub api_key: String,
    /// OpenAI model to use
    pub model: String,
    /// Maximum tokens for response
    pub max_tokens: u32,
    /// Temperature for AI responses (0.0 to 2.0)
    pub temperature: f32,
}

impl Config {
    /// Creates a new configuration with default values
    pub fn new() -> Result<Self> {
        let api_key =
            std::env::var("OPENAI_API_KEY").map_err(|_| SuperTerminalError::MissingApiKey)?;

        Ok(Self {
            api_key,
            model: "gpt-4".to_string(),
            max_tokens: 150,
            temperature: 0.3,
        })
    }

    /// Loads configuration from file or creates default
    pub fn load_or_default() -> Result<Self> {
        match Self::load() {
            Ok(config) => Ok(config),
            Err(_) => {
                let config = Self::new()?;
                let _ = config.save(); // Ignore save errors for first-time users
                Ok(config)
            }
        }
    }

    /// Loads configuration from the config file
    fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        let content = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&content)
            .map_err(|e| SuperTerminalError::ConfigError(e.to_string()))?;
        Ok(config)
    }

    /// Saves configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| SuperTerminalError::ConfigError(e.to_string()))?;
        fs::write(config_path, content)?;
        Ok(())
    }

    /// Returns the path to the config file
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| {
            SuperTerminalError::ConfigError("Could not find config directory".to_string())
        })?;
        Ok(config_dir.join("superterminal").join("config.json"))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model: "gpt-4".to_string(),
            max_tokens: 150,
            temperature: 0.3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.max_tokens, 150);
        assert_eq!(config.temperature, 0.3);
    }
}
