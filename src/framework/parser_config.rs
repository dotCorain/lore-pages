use serde::Deserialize;
use std::fs;
use std::path::Path;

// 配置解析：使用 Serde 从 TOML 文件读取可选的 parser 表
// `fs` 提供文件 I/O，`Path` 是路径类型，用于在文件系统中定位文件

#[derive(Deserialize)]
struct TomlConfig {
    // `Option<ParserTable>` 表示 TOML 文件中可能存在 `parser` 部分，或者不存在（None）
    pub parser: Option<ParserTable>,
}

#[derive(Deserialize)]
struct ParserTable {
    // 这里所有字段都是 `Option<String>`：配置可以选择性提供这些键
    // 如果某个键缺失，代码会在后面使用默认值代替
    pub url_link_key: Option<String>,
    pub url_link_key_escape: Option<String>,
    pub comment_key: Option<String>,
    pub comment_key_escape: Option<String>,
    pub placeholder_key_escape: Option<String>,
    pub breakline_key: Option<String>,
    pub breakline_key_escape: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParserConfig {
    // 运行时使用的实际设置，使用 `String`（拥有所有权）保存值
    pub url_link_key: String,
    pub url_link_key_escape: String,
    pub comment_key: String,
    pub comment_key_escape: String,
    pub placeholder_key_escape: String,
    pub breakline_key: String,
    pub breakline_key_escape: String,
}

impl ParserConfig {
    // 从文件加载配置：使用泛型 `P: AsRef<Path>` 以接受 `&str` / `String` / `PathBuf` 等多种路径类型
    // 返回 `Result<Self, Box<dyn std::error::Error>>`：成功返回 ParserConfig，失败返回任意错误的“装箱” trait 对象
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        // 读取整个文件为 `String`：`fs::read_to_string` 返回 `Result<String, std::io::Error>`。
        // `?` 会在出错时提前返回，将错误传播给调用方
        let content = fs::read_to_string(path)?;

        // 使用 `toml::from_str` 将 TOML 文本反序列化为 `TomlConfig`，解析错误同样会被 `?` 传播
        let toml_config: TomlConfig = toml::from_str(&content)?;

        // 如果 TOML 中没有 `parser` 表，使用一个空的 `ParserTable` 作为占位（所有字段为 None）
        let parser = toml_config.parser.unwrap_or(ParserTable {
            url_link_key: None,
            url_link_key_escape: None,
            comment_key: None,
            comment_key_escape: None,
            placeholder_key_escape: None,
            breakline_key: None,
            breakline_key_escape: None,
        });

        // 构造最终的 ParserConfig：对每个可能为 None 的字段使用默认值
        // `unwrap_or_else(|| "...".to_string())` 只有在 Option 为 None 时才执行闭包并返回默认字符串
        Ok(Self {
            url_link_key: parser.url_link_key.unwrap_or_else(|| "|".to_string()),
            url_link_key_escape: parser
                .url_link_key_escape
                .unwrap_or_else(|| "\\|".to_string()),
            comment_key: parser.comment_key.unwrap_or_else(|| "%".to_string()),
            comment_key_escape: parser
                .comment_key_escape
                .unwrap_or_else(|| "\\%".to_string()),
            placeholder_key_escape: parser
                .placeholder_key_escape
                .unwrap_or_else(|| "\\_".to_string()),
            breakline_key: parser.breakline_key.unwrap_or_else(|| "---".to_string()),
            breakline_key_escape: parser
                .breakline_key_escape
                .unwrap_or_else(|| "\\---".to_string()),
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
            comment_key: "%".to_string(),
            comment_key_escape: "\\%".to_string(),
            placeholder_key_escape: "\\_".to_string(),
            breakline_key: "---".to_string(),
            breakline_key_escape: "\\---".to_string(),
        }
    }
}
