use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct RenderConfig {
    pub from_lore_path: String,
    pub to_html_path: String,
    pub css_url: String,
    pub main_lang: String,
}

impl RenderConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: RenderConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn from_file_or_default(path: &str) -> Self {
        Self::from_file(path).unwrap_or_else(|_| Self::default())
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            from_lore_path: "./lore".to_string(),
            to_html_path: "./html".to_string(),
            css_url: "style.css".to_string(),
            main_lang: "en-US".to_string(),
        }
    }
}
