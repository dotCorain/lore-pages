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
        source_path: Option<&str>,
    ) -> String {
        let mut doc = self.parser.parse(raw, category_config, parser_config);
        doc.auto_link_h2();
        self.renderer.render(&doc, category_config, renderer_config, source_path)
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
            None,
        )
    }

    /// 同 convert_simple，但传入源文件路径用于 warning 信息。
    pub fn convert_with_source(
        &self,
        raw: &str,
        source_path: &str,
    ) -> String {
        self.convert(
            raw,
            self.category_config,
            self.renderer_config,
            self.parser_config,
            Some(source_path),
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
