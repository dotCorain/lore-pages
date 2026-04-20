//! # lang-framework
//!
//! 这个 crate 定义了解析器和渲染器的接口（trait），
//! 以及配置加载和转换器。
//!
//! 简要说明：
//! - `Parser`：将输入文本解析成 `lang_core::Document`。
//! - `Renderer`：将 `Document` 渲染为输出（例如 HTML）。
//! - `Config`：负责加载工具所需的配置（从 TOML 文件）。
//! - `Converter`：组合 `Parser` 与 `Renderer`，提供一键转换接口。
//!
//! 示例用法（仅示例说明，不会在 doctest 中编译运行）：
//! ```ignore
//! use lang_framework::{Converter, Config};
//! use lang_parser::MarkdownParser;
//! use lang_impl::HtmlRenderer;
//!
//! let config = Config::from_file_or_default("Lore.toml");
//! let converter = Converter::from_config(&config, MarkdownParser, HtmlRenderer);
//! let html = converter.convert("# 示例", "示例文档");
//! println!("{}", html);
//! ```

mod config;
mod converter;
mod parser;
mod renderer;

pub use config::Config;
pub use converter::Converter;
pub use parser::Parser;
pub use renderer::Renderer;

/// 预导入模块：通过 `use lang_framework::prelude::*;` 快速导入常用 trait/类型。
///
/// 包含 `Config`, `Converter`, `Parser`, `Renderer`。
pub mod prelude {
    pub use crate::{Config, Converter, Parser, Renderer};
}
