//! # lang-parser
//!
//! 这个 crate 实现了 Markdown 解析器。
//!
//! 支持的语法：
//! - 标题：#、##、###、####（井号后必须有空格）
//! - 段落：普通文本
//!
//! # 示例
//!
//! ```
//! use lang_parser::MarkdownParser;
//! use lang_framework::Parser;
//!
//! let parser = MarkdownParser;
//! let doc = parser.parse("# Hello\n\nWorld");
//!
//! // 注意：空行会被解析为空段落，所以有 3 个节点
//! assert_eq!(doc.nodes.len(), 3);
//! ```

pub mod parser;

pub use parser::MarkdownParser;
pub use parser::parse_heading;

pub mod prelude {
    /// 预导入模块：通过 `use lang_parser::prelude::*;` 导出 `MarkdownParser`。
    pub use crate::MarkdownParser;
}
