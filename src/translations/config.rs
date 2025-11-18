use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationConfig {
    /// Starting surah number (1-114)
    pub start_surah: u8,
    /// Ending surah number (1-114)
    pub end_surah: u8,
    /// Arabic text configuration
    pub arabic: Option<ArabicConfig>,
    /// Include word-by-word translation
    pub word_by_word: bool,
    /// Translation configuration
    pub translation: Option<TranslationDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArabicConfig {
    /// Script type: "indopak" or "uthmani"
    pub script: ArabicScript,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArabicScript {
    IndoPak,
    Uthmani,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationDetails {
    /// Translation ID from Quran.com (e.g., 20 for Sahih International)
    pub id: u8,
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

impl TranslationConfig {
    /// Load configuration from a JSON file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)?;
        let config: TranslationConfig = serde_json::from_str(&content)?;
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

        // Validate that at least one content type is requested
        if self.arabic.is_none() && !self.word_by_word && self.translation.is_none() {
            return Err(Error::InvalidConfig(
                "Must include at least one of: arabic, word_by_word, or translation".to_string()
            ));
        }

        Ok(())
    }
}
