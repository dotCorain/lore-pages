use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct TomlConfig {
    pub parser: Option<ParserTable>,
}

#[derive(Deserialize)]
struct ParserTable {
    pub url_link_key: Option<String>,
    pub url_link_key_escape: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub url_link_key: String,
    pub url_link_key_escape: String,
}

impl ParserConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let toml_config: TomlConfig = toml::from_str(&content)?;
        let parser = toml_config.parser.unwrap_or(ParserTable {
            url_link_key: None,
            url_link_key_escape: None,
        });

        Ok(Self {
            url_link_key: parser.url_link_key.unwrap_or_else(|| "|".to_string()),
            url_link_key_escape: parser
                .url_link_key_escape
                .unwrap_or_else(|| "\\|".to_string()),
        })
    }

    pub fn from_file_or_default(path: &str) -> Self {
        Self::from_file(path).unwrap_or_else(|_| Self::default())
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            url_link_key: "|".to_string(),
            url_link_key_escape: "\\|".to_string(),
        }
    }
}
