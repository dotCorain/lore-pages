//! Crate entry point. Exports the four public sub-modules:
//!
//! - [`framework`] — Traits, configuration, and the conversion pipeline.
//! - [`ir`] — Intermediate representation types ([`Anchor`], [`Category`]).
//! - [`parser`] — Default parser implementation ([`LorePagesParser`]).
//! - [`render`] — Default HTML renderer ([`HtmlRenderer`]).
//!
//! # Quick start
//!
//! ```
//! use lore_pages::framework::category_config::CategoryConfig;
//! use lore_pages::framework::converter::CategoryConverter;
//! use lore_pages::framework::parser_config::ParserConfig;
//! use lore_pages::framework::renderer_config::RenderConfig;
//! use lore_pages::parser::LorePagesParser;
//! use lore_pages::render::HtmlRenderer;
//!
//! let cat_cfg = CategoryConfig::default();
//! let rend_cfg = RenderConfig::default();
//! let pars_cfg = ParserConfig::default();
//!
//! let converter = CategoryConverter::from_config(
//!     LorePagesParser,
//!     HtmlRenderer,
//!     &cat_cfg,
//!     &rend_cfg,
//!     &pars_cfg,
//! );
//!
//! let html = converter.convert_simple("# Hello\n\nWorld!\n");
//! assert!(html.contains("<h1>Hello</h1>"));
//! ```

pub mod framework;
pub mod ir;
pub mod parser;
pub mod render;
