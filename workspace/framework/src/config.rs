//! 配置管理

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// 配置文件结构
///
/// # 示例
///
/// ```toml
/// from_lore_path = "./lore"
/// to_html_path = "./html"
/// css_url = "style.css"
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// 源文件目录（.lore 文件所在目录）
    pub from_lore_path: String,
    /// 输出目录（HTML 文件输出目录）
    pub to_html_path: String,
    /// CSS 样式表 URL
    pub css_url: String,
}

impl Config {
    /// 从文件加载配置
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use lang_framework::Config;
    ///
    /// // 示例：从文件加载配置。此示例使用 `no_run` 标记，
    /// // 不会在 doctest 中执行，避免因缺少本地文件导致失败。
    /// let _ = Config::from_file("Lore.toml");
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// 从文件加载配置，如果失败则使用默认值
    pub fn from_file_or_default(path: &str) -> Self {
        Self::from_file(path).unwrap_or_else(|_| Self::default())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            from_lore_path: "./lore".to_string(),
            to_html_path: "./html".to_string(),
            css_url: "style.css".to_string(),
        }
    }
}
