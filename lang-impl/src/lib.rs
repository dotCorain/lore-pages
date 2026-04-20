//! # lang-impl
//!
//! 这个 crate 实现了 HTML 渲染器。
//!
//! 提供 `HtmlRenderer`，用于将 `lang_core::Category` 渲染为完整的 HTML 页面。
//!
//! 示例：
//!
//! ```rust
//! use lang_impl::HtmlRenderer;
//! use lang_core::{Category, Anchor};
//! use lang_framework::Renderer;
//!
//! let mut doc = Category::new();
//! doc.push(Anchor::Paragraph { content: "示例内容".to_string() });
//! let renderer = HtmlRenderer;
//! let html = renderer.render(&doc, "页面标题", "style.css");
//! println!("{}", html);
//! ```

mod renderer;

pub use renderer::HtmlRenderer;

pub mod prelude {
    pub use crate::HtmlRenderer;
}
