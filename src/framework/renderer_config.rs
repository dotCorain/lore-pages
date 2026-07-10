use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct TomlConfig {
    pub renderer: RendererTable,
}

#[derive(Deserialize)]
pub struct RendererTable {
    pub site_title: Option<String>,
    pub from_lore_path: Option<String>,
    pub to_html_path: Option<String>,
    pub css_url: Option<String>,
    pub lang: Option<String>,
    pub link_base: Option<String>,
    pub scripts: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub site_title: String,
    pub from_lore_path: String,
    pub to_html_path: String,
    pub css_url: String,
    pub main_lang: String,
    pub link_base: String,
    pub scripts: Vec<String>,
}

impl RenderConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let toml_config: TomlConfig = toml::from_str(&content)?;
        let renderer = toml_config.renderer;

        // None 的字段使用 Default 实现中的值，而非空字符串
        let defaults = RenderConfig::default();
        Ok(Self {
            site_title: renderer.site_title.unwrap_or(defaults.site_title),
            from_lore_path: renderer.from_lore_path.unwrap_or(defaults.from_lore_path),
            to_html_path: renderer.to_html_path.unwrap_or(defaults.to_html_path),
            css_url: renderer.css_url.unwrap_or(defaults.css_url),
            main_lang: renderer.lang.unwrap_or(defaults.main_lang),
            link_base: renderer.link_base.unwrap_or(defaults.link_base),
            scripts: renderer.scripts.unwrap_or(defaults.scripts),
        })
    }

    pub fn from_file_or_default(path: &str) -> Self {
        Self::from_file(path).unwrap_or_else(|_| Self::default())
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            site_title: "LorePages".to_string(),
            from_lore_path: "./lore".to_string(),
            to_html_path: "./html".to_string(),
            css_url: "https://fleetinglore.github.io/css/style.css".to_string(),
            main_lang: "en-US".to_string(),
            link_base: "".to_string(),
            scripts: vec![],
        }
    }
}
