use crate::framework::config::Config;
use crate::framework::parser::Parser;
use crate::framework::renderer::Renderer;

pub struct Converter<P, R> {
    parser: P,
    renderer: R,
    css_url: String,
}

impl<P, R> Converter<P, R>
where
    P: Parser,
    R: Renderer,
{
    pub fn new(parser: P, renderer: R, css_url: String) -> Self {
        Self {
            parser,
            renderer,
            css_url,
        }
    }

    pub fn from_config(config: &Config, parser: P, renderer: R) -> Self {
        Self {
            parser,
            renderer,
            css_url: config.css_url.clone(),
        }
    }

    pub fn convert(&self, input: &str, title: &str) -> String {
        let doc = self.parser.parse(input);
        self.renderer.render(&doc, title, &self.css_url)
    }

    pub fn css_url(&self) -> &str {
        &self.css_url
    }

    pub fn parser(&self) -> &P {
        &self.parser
    }

    pub fn renderer(&self) -> &R {
        &self.renderer
    }
}
