use crate::framework::category_config::CategoryConfig;
use crate::framework::parser::Parser;
use crate::framework::parser_config::ParserConfig;
use crate::framework::renderer::Renderer;
use crate::framework::renderer_config::RenderConfig;

pub struct CategoryConverter<'a, P, R> {
    parser: P,
    renderer: R,
    category_config: &'a CategoryConfig,
    renderer_config: &'a RenderConfig,
    parser_config: &'a ParserConfig,
}

impl<'a, P, R> CategoryConverter<'a, P, R>
where
    P: Parser<'a>,
    R: Renderer<'a>,
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

    pub fn convert(
        &self,
        raw: &'a str,
        category_config: &'a CategoryConfig,
        renderer_config: &'a RenderConfig,
        parser_config: &'a ParserConfig,
    ) -> String {
        let doc = &self.parser.parse(raw, parser_config);
        self.renderer.render(doc, category_config, renderer_config)
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
