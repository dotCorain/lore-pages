use serde::Deserialize;
use std::fs;
use std::path::Path;

// ── TOML deserialization helpers ──────────────────────────────────

#[derive(Deserialize)]
struct TomlConfig {
    pub parser: Option<ParserTable>,
}

#[derive(Deserialize)]
struct ParserTable {
    pub url_link_key: Option<String>,
    pub url_link_key_escape: Option<String>,
    pub lore_link_key: Option<String>,
    pub lore_link_key_escape: Option<String>,
    pub comment_key: Option<String>,
    pub comment_key_escape: Option<String>,
    pub placeholder_key: Option<String>,
    pub placeholder_key_escape: Option<String>,
    pub breakline_key: Option<String>,
    pub breakline_key_escape: Option<String>,
    pub image_key: Option<String>,
    pub image_key_escape: Option<String>,
    pub inner_url_key: Option<String>,
    pub inner_url_key_escape: Option<String>,
    pub inner_lore_key: Option<String>,
    pub inner_lore_key_escape: Option<String>,
    pub inner_open_key: Option<String>,
    pub inner_open_key_escape: Option<String>,
    pub inner_close_key: Option<String>,
    pub inner_close_key_escape: Option<String>,
}

// ── Public configuration ─────────────────────────────────────────

/// Parser configuration: controls the marker characters and their escape
/// sequences used in Lore markup syntax.
///
/// Loaded from the `[parser]` section of `Lore.toml`. Every field has a
/// sensible default (see [`ParserConfig::default`]).
///
/// # Customization example
///
/// ```toml
/// [parser]
/// url_link_key = "->"
/// url_link_key_escape = "\\->"
/// ```
///
/// This would let you write `Docs -> https://example.com` instead of the
/// default `Docs | https://example.com`.
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Key for external URL links (default: `|`).
    pub url_link_key: String,
    /// Escape sequence for `url_link_key` (default: `\|`).
    pub url_link_key_escape: String,
    /// Key for internal Lore links (default: `=`).
    pub lore_link_key: String,
    /// Escape sequence for `lore_link_key` (default: `\=`).
    pub lore_link_key_escape: String,
    /// Key for comments (default: `%`).
    pub comment_key: String,
    /// Escape sequence for `comment_key` (default: `\%`).
    pub comment_key_escape: String,
    /// Key for placeholders (default: `_`).
    pub placeholder_key: String,
    /// Escape sequence for `placeholder_key` (default: `\_`).
    pub placeholder_key_escape: String,
    /// Key for breaklines (default: `---`).
    pub breakline_key: String,
    /// Escape sequence for `breakline_key` (default: `\---`).
    pub breakline_key_escape: String,
    /// Key for images (default: `|`).
    pub image_key: String,
    /// Escape sequence for `image_key` (default: `\|`).
    pub image_key_escape: String,
    /// Separator key inside foldable URL blocks (default: `>`).
    pub inner_url_key: String,
    /// Escape sequence for `inner_url_key` (default: `\>`).
    pub inner_url_key_escape: String,
    /// Separator key inside foldable Lore blocks (default: `=`).
    pub inner_lore_key: String,
    /// Escape sequence for `inner_lore_key` (default: `\=`).
    pub inner_lore_key_escape: String,
    /// Key for opening a foldable block (default: `-`).
    pub inner_open_key: String,
    /// Escape sequence for `inner_open_key` (default: `\-`).
    pub inner_open_key_escape: String,
    /// Key for closing a foldable block (default: `+`).
    pub inner_close_key: String,
    /// Escape sequence for `inner_close_key` (default: `\+`).
    pub inner_close_key_escape: String,
}

impl ParserConfig {
    /// Load configuration from a TOML file.
    ///
    /// Missing keys fall back to their [`Default`] values.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let toml_config: TomlConfig = toml::from_str(&content)?;

        let parser = toml_config.parser.unwrap_or(ParserTable {
            url_link_key: None,
            url_link_key_escape: None,
            lore_link_key: None,
            lore_link_key_escape: None,
            comment_key: None,
            comment_key_escape: None,
            placeholder_key: None,
            placeholder_key_escape: None,
            breakline_key: None,
            breakline_key_escape: None,
            image_key: None,
            image_key_escape: None,
            inner_url_key: None,
            inner_url_key_escape: None,
            inner_lore_key: None,
            inner_lore_key_escape: None,
            inner_open_key: None,
            inner_open_key_escape: None,
            inner_close_key: None,
            inner_close_key_escape: None,
        });

        let defaults = ParserConfig::default();
        Ok(Self {
            url_link_key: parser.url_link_key.unwrap_or(defaults.url_link_key),
            url_link_key_escape: parser
                .url_link_key_escape
                .unwrap_or(defaults.url_link_key_escape),
            lore_link_key: parser.lore_link_key.unwrap_or(defaults.lore_link_key),
            lore_link_key_escape: parser
                .lore_link_key_escape
                .unwrap_or(defaults.lore_link_key_escape),
            comment_key: parser.comment_key.unwrap_or(defaults.comment_key),
            comment_key_escape: parser
                .comment_key_escape
                .unwrap_or(defaults.comment_key_escape),
            placeholder_key: parser.placeholder_key.unwrap_or(defaults.placeholder_key),
            placeholder_key_escape: parser
                .placeholder_key_escape
                .unwrap_or(defaults.placeholder_key_escape),
            breakline_key: parser.breakline_key.unwrap_or(defaults.breakline_key),
            breakline_key_escape: parser
                .breakline_key_escape
                .unwrap_or(defaults.breakline_key_escape),
            image_key: parser.image_key.unwrap_or(defaults.image_key),
            image_key_escape: parser.image_key_escape.unwrap_or(defaults.image_key_escape),
            inner_url_key: parser.inner_url_key.unwrap_or(defaults.inner_url_key),
            inner_url_key_escape: parser
                .inner_url_key_escape
                .unwrap_or(defaults.inner_url_key_escape),
            inner_lore_key: parser.inner_lore_key.unwrap_or(defaults.inner_lore_key),
            inner_lore_key_escape: parser
                .inner_lore_key_escape
                .unwrap_or(defaults.inner_lore_key_escape),
            inner_open_key: parser.inner_open_key.unwrap_or(defaults.inner_open_key),
            inner_open_key_escape: parser
                .inner_open_key_escape
                .unwrap_or(defaults.inner_open_key_escape),
            inner_close_key: parser.inner_close_key.unwrap_or(defaults.inner_close_key),
            inner_close_key_escape: parser
                .inner_close_key_escape
                .unwrap_or(defaults.inner_close_key_escape),
        })
    }

    /// Load from file, falling back to defaults on any error.
    pub fn from_file_or_default(path: &str) -> Self {
        Self::from_file(path).unwrap_or_else(|_| Self::default())
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            url_link_key: "|".to_string(),
            url_link_key_escape: "\\|".to_string(),
            lore_link_key: "=".to_string(),
            lore_link_key_escape: "\\=".to_string(),
            comment_key: "%".to_string(),
            comment_key_escape: "\\%".to_string(),
            placeholder_key: "_".to_string(),
            placeholder_key_escape: "\\_".to_string(),
            breakline_key: "---".to_string(),
            breakline_key_escape: "\\---".to_string(),
            image_key: "|".to_string(),
            image_key_escape: "\\|".to_string(),
            inner_url_key: ">".to_string(),
            inner_url_key_escape: "\\>".to_string(),
            inner_lore_key: "=".to_string(),
            inner_lore_key_escape: "\\=".to_string(),
            inner_open_key: "-".to_string(),
            inner_open_key_escape: "\\-".to_string(),
            inner_close_key: "+".to_string(),
            inner_close_key_escape: "\\+".to_string(),
        }
    }
}
