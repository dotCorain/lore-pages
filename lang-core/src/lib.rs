//! # lang-core
//!
//! 这个 crate 定义了文档的 IR（中间表示）。
//! IR 是解析器和渲染器之间传递的数据结构。

mod ir;

pub use ir::{Document, Node};

/// 这个 crate 的预导入模块。
///
/// 使用 `use lang_core::prelude::*;` 可以一次性导入常用类型：`Document`、`Node`。
///
/// 示例：
/// ```rust
/// use lang_core::prelude::*;
///
/// let mut doc = Document::new();
/// doc.push(Node::Paragraph { content: "示例内容".to_string() });
/// ```
pub mod prelude {
    pub use crate::{Document, Node};
}
