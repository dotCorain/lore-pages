use crate::framework::category_config::CategoryConfig;
use crate::framework::parser::Parser;
use crate::framework::parser_config::ParserConfig;
use crate::framework::renderer::Renderer;
use crate::framework::renderer_config::RenderConfig;

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

    pub fn convert<'b>(
        &self,
        raw: &'b str,
        category_config: &'b CategoryConfig,
        renderer_config: &'b RenderConfig,
        parser_config: &'b ParserConfig,
    ) -> String {
        let doc = self.parser.parse(raw, category_config, parser_config);
        self.renderer.render(&doc, category_config, renderer_config)
    }

    pub fn convert_simple(
        &self,
        raw: &str,
    ) -> String {
        self.convert(
            raw,
            self.category_config,
            self.renderer_config,
            self.parser_config,
        )
    }

    pub fn css_url(&self) -> &str {
        &self.renderer_config.css_url
    }

    pub fn parser(&self) -> &P {
        &self.parser
    }

    pub fn renderer(&self) -> &R {
        &self.renderer
    }
}
