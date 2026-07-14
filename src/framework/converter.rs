use crate::framework::category_config::CategoryConfig;
use crate::framework::parser::Parser;
use crate::framework::parser_config::ParserConfig;
use crate::framework::renderer::Renderer;
use crate::framework::renderer_config::RenderConfig;

/// Combines a [`Parser`] and [`Renderer`] into a single conversion pipeline.
///
/// This is the primary entry point for turning Lore source text into
/// rendered output. It holds references to configuration and delegates
/// to the parser and renderer implementations.
pub struct CategoryConverter<'a, P, R> {
    parser: P,
    renderer: R,
    #[allow(dead_code)]
    category_config: &'a CategoryConfig,
    renderer_config: &'a RenderConfig,
    #[allow(dead_code)]
    parser_config: &'a ParserConfig,
}

impl<'a, P, R> CategoryConverter<'a, P, R>
where
    P: Parser,
    R: Renderer,
{
    /// Build a converter from a parser, renderer, and configuration references.
    pub fn from_config(
        parser: P,
        renderer: R,
        category_config: &'a CategoryConfig,
        renderer_config: &'a RenderConfig,
        parser_config: &'a ParserConfig,
    ) -> Self {
        Self {
            parser,
            renderer,
            category_config,
            renderer_config,
            parser_config,
        }
    }

    /// Parse and render with explicit configuration overrides.
    pub fn convert<'b>(
        &self,
        raw: &'b str,
        category_config: &'b CategoryConfig,
        renderer_config: &'b RenderConfig,
        parser_config: &'b ParserConfig,
        source_path: Option<&str>,
    ) -> String {
        let mut doc = self.parser.parse(raw, category_config, parser_config);
        doc.auto_link_h2();
        self.renderer
            .render(&doc, category_config, renderer_config, source_path)
    }

    /// Convenience method: convert using the converter's stored configuration.
    pub fn convert_simple(&self, raw: &str) -> String {
        self.convert(
            raw,
            self.category_config,
            self.renderer_config,
            self.parser_config,
            None,
        )
    }

    /// Convenience method: like `convert_simple`, but passes a source file
    /// path for use in warning messages and relative link resolution.
    pub fn convert_with_source(&self, raw: &str, source_path: &str) -> String {
        self.convert(
            raw,
            self.category_config,
            self.renderer_config,
            self.parser_config,
            Some(source_path),
        )
    }

    /// Return the configured CSS URL.
    pub fn css_url(&self) -> &str {
        &self.renderer_config.css_url
    }

    /// Access the parser.
    pub fn parser(&self) -> &P {
        &self.parser
    }

    /// Access the renderer.
    pub fn renderer(&self) -> &R {
        &self.renderer
    }
}
