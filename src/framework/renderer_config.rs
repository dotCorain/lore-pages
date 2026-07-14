use serde::Deserialize;
use std::fs;
use std::path::Path;

// ── TOML deserialization helpers ──────────────────────────────────

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

// ── Public configuration ─────────────────────────────────────────

/// Renderer configuration: controls the output HTML structure and paths.
///
/// Loaded from the `[renderer]` section of `Lore.toml`. Every field has a
/// sensible default (see [`RenderConfig::default`]).
///
/// # Example
///
/// ```toml
/// [renderer]
/// site_title = "My Documentation"
/// from_lore_path = "./docs-src"
/// to_html_path = "./docs"
/// lang = "zh-CN"
/// ```
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Site title used as the HTML `<title>` fallback.
    pub site_title: String,
    /// Source directory for `.lore` files.
    pub from_lore_path: String,
    /// Output directory for generated `.html` files.
    pub to_html_path: String,
    /// URL to the CSS stylesheet.
    pub css_url: String,
    /// HTML `lang` attribute value (e.g. `"en-US"`, `"zh-CN"`).
    pub main_lang: String,
    /// Base URL prefix applied to all relative Lore links.
    pub link_base: String,
    /// List of JavaScript files to include via `<script>` tags.
    pub scripts: Vec<String>,
}

impl RenderConfig {
    /// Load configuration from a TOML file.
    ///
    /// Missing keys fall back to their [`Default`] values.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let toml_config: TomlConfig = toml::from_str(&content)?;
        let renderer = toml_config.renderer;

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

    /// Load from file, falling back to defaults on any error.
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
