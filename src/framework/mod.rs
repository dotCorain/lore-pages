//! Configurable framework for parsing and rendering Lore documents.
//!
//! This module provides the traits ([`Parser`], [`Renderer`]) and
//! configuration types that make Lore Pages extensible. The default
//! implementations live in [`crate::parser`] and [`crate::render`].

pub mod category_config;
pub mod converter;
pub mod parser;
pub mod parser_config;
pub mod renderer;
pub mod renderer_config;
