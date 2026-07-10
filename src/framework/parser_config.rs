use serde::Deserialize;
use std::fs;
use std::path::Path;

// 配置解析：使用 Serde 从 TOML 文件读取可选的 parser 表
// `fs` 提供文件 I/O，`Path` 是路径类型，用于在文件系统中定位文件

#[derive(Deserialize)]
struct TomlConfig {
    // `Option<ParserTable>` 表示 TOML 文件中可能存在 `parser` 部分，
    // 或者不存在（None）
    pub parser: Option<ParserTable>,
}

#[derive(Deserialize)]
struct ParserTable {
    // 这里所有字段都是 `Option<String>`：配置可以选择性提供这些键
    // 如果某个键缺失，代码会在后面使用默认值代替
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

#[derive(Debug, Clone)]
pub struct ParserConfig {
    // 运行时使用的实际设置，使用 `String`（拥有所有权）保存值
    pub url_link_key: String,
    pub url_link_key_escape: String,
    pub lore_link_key: String,
    pub lore_link_key_escape: String,
    pub comment_key: String,
    pub comment_key_escape: String,
    pub placeholder_key: String,
    pub placeholder_key_escape: String,
    pub breakline_key: String,
    pub breakline_key_escape: String,
    pub image_key: String,
    pub image_key_escape: String,
    pub inner_url_key: String,
    pub inner_url_key_escape: String,
    pub inner_lore_key: String,
    pub inner_lore_key_escape: String,
    pub inner_open_key: String,
    pub inner_open_key_escape: String,
    pub inner_close_key: String,
    pub inner_close_key_escape: String,
}

impl ParserConfig {
    // 从文件加载配置：使用泛型 `P: AsRef<Path>` 以接受
    //  `&str` / `String` / `PathBuf` 等多种路径类型
    // 返回 `Result<Self, Box<dyn std::error::Error>>`：成功返
    // 回 ParserConfig，失败返回任意错误的"装箱" trait 对象
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        // 读取整个文件为 `String`：`fs::read_to_string` 返回
        //  `Result<String, std::io::Error>`。
        // `?` 会在出错时提前返回，将错误传播给调用方
        let content = fs::read_to_string(path)?;

        // 使用 `toml::from_str` 将 TOML 文本反序列化为 `TomlConfig`，解析
        // 错误同样会被 `?` 传播
        let toml_config: TomlConfig = toml::from_str(&content)?;

        // 如果 TOML 中没有 `parser` 表，使用一个空的 `ParserTable` 作
        // 为占位（所有字段为 None）
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

        // 构造最终的 ParserConfig：None 的字段使用 Default 实现中的值
        // 注意：不能用 unwrap_or_default() —— Option<String>::default() 是 ""
        // 会错误地覆盖掉真正的默认值（如 lore_link_key 的 "="）
        let defaults = ParserConfig::default();
        Ok(Self {
            url_link_key: parser.url_link_key.unwrap_or(defaults.url_link_key),
            url_link_key_escape: parser.url_link_key_escape.unwrap_or(defaults.url_link_key_escape),
            lore_link_key: parser.lore_link_key.unwrap_or(defaults.lore_link_key),
            lore_link_key_escape: parser.lore_link_key_escape.unwrap_or(defaults.lore_link_key_escape),
            comment_key: parser.comment_key.unwrap_or(defaults.comment_key),
            comment_key_escape: parser.comment_key_escape.unwrap_or(defaults.comment_key_escape),
            placeholder_key: parser.placeholder_key.unwrap_or(defaults.placeholder_key),
            placeholder_key_escape: parser.placeholder_key_escape.unwrap_or(defaults.placeholder_key_escape),
            breakline_key: parser.breakline_key.unwrap_or(defaults.breakline_key),
            breakline_key_escape: parser.breakline_key_escape.unwrap_or(defaults.breakline_key_escape),
            image_key: parser.image_key.unwrap_or(defaults.image_key),
            image_key_escape: parser.image_key_escape.unwrap_or(defaults.image_key_escape),
            inner_url_key: parser.inner_url_key.unwrap_or(defaults.inner_url_key),
            inner_url_key_escape: parser.inner_url_key_escape.unwrap_or(defaults.inner_url_key_escape),
            inner_lore_key: parser.inner_lore_key.unwrap_or(defaults.inner_lore_key),
            inner_lore_key_escape: parser.inner_lore_key_escape.unwrap_or(defaults.inner_lore_key_escape),
            inner_open_key: parser.inner_open_key.unwrap_or(defaults.inner_open_key),
            inner_open_key_escape: parser.inner_open_key_escape.unwrap_or(defaults.inner_open_key_escape),
            inner_close_key: parser.inner_close_key.unwrap_or(defaults.inner_close_key),
            inner_close_key_escape: parser.inner_close_key_escape.unwrap_or(defaults.inner_close_key_escape),
        })
    }

    // 如果从文件加载失败（无论是 I/O 错误还是解析错误），回退到默认配置
    // `unwrap_or_else(|_| Self::default())` 捕获任何错误并返回 `Default` 实例
    pub fn from_file_or_default(path: &str) -> Self {
        Self::from_file(path).unwrap_or_else(|_| Self::default())
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        // `to_string()` 将字符串字面量转换为 `String`（堆分配）
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
