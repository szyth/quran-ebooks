use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TafsirConfig {
    /// Starting surah number (1-114)
    pub start_surah: u8,
    /// Ending surah number (1-114)
    pub end_surah: u8,
    /// Tafsir resource ID from Quran.com
    pub resource_id: u16,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
    #[error("JSONParseError: {0}")]
    JSONParseError(#[from] serde_json::Error),
    #[error("InvalidConfig: {0}")]
    InvalidConfig(String),
}

impl TafsirConfig {
    /// Load configuration from a JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)?;
        let config: TafsirConfig = serde_json::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    fn validate(&self) -> Result<(), Error> {
        // Validate surah range
        if self.start_surah < 1 || self.start_surah > 114 {
            return Err(Error::InvalidConfig(format!(
                "start_surah must be between 1 and 114, got {}",
                self.start_surah
            )));
        }
        if self.end_surah < 1 || self.end_surah > 114 {
            return Err(Error::InvalidConfig(format!(
                "end_surah must be between 1 and 114, got {}",
                self.end_surah
            )));
        }
        if self.start_surah > self.end_surah {
            return Err(Error::InvalidConfig(format!(
                "start_surah ({}) must be less than or equal to end_surah ({})",
                self.start_surah, self.end_surah
            )));
        }

        Ok(())
    }
}
