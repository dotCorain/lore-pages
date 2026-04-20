//! # lang-core
//!
//! This crate defines IR for lore-pages.
//! IR is the data structure between parser and impl.

mod ir;

pub use ir::{Anchor, Category};

/// 这个 crate 的预导入模块。
///
/// 使用 `use lang_core::prelude::*;` 可以一次性导入常用类型：`Category`、`Anchor`。
///
/// `Category` is file with domians.
///
/// `Anchor` is line.
///
/// For example:
/// ```rust
/// use lang_core::prelude::*;
///
/// let mut doc = Category::new();
/// doc.push(Anchor::Paragraph { content: "示例内容".to_string() });
/// ```
pub mod prelude {
    pub use crate::{Anchor, Category};
}
