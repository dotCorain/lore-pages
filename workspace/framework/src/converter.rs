//! 转换器：组合解析器和渲染器

use crate::{Config, Parser, Renderer};

/// 转换器
///
/// 组合一个解析器和一个渲染器，提供统一的转换接口。
///
/// # 示例
///
/// ```
/// use lang_framework::{Converter, Parser, Renderer};
/// use lang_core::{Category, Anchor};
///
/// struct MyParser;
/// impl Parser for MyParser {
///     fn parse(&self, input: &str) -> Category {
///         let mut doc = Category::new();
///         doc.push(Anchor::Paragraph { content: input.to_string() });
///         doc
///     }
/// }
///
/// struct MyRenderer;
/// impl Renderer for MyRenderer {
///     fn render(&self, doc: &Category, title: &str, css_url: &str) -> String {
///         format!("<html><title>{}</title><body>{:?}</body></html>", title, doc)
///     }
/// }
///
/// let converter = Converter::new(MyParser, MyRenderer, "style.css".to_string());
/// let html = converter.convert("Hello world", "My Page");
/// ```
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
    /// 创建新的转换器实例
    pub fn new(parser: P, renderer: R, css_url: String) -> Self {
        Self {
            parser,
            renderer,
            css_url,
        }
    }

    /// 从配置创建转换器
    pub fn from_config(config: &Config, parser: P, renderer: R) -> Self {
        Self {
            parser,
            renderer,
            css_url: config.css_url.clone(),
        }
    }

    /// 转换单个文件内容
    pub fn convert(&self, input: &str, title: &str) -> String {
        let doc = self.parser.parse(input);
        self.renderer.render(&doc, title, &self.css_url)
    }

    /// 获取 CSS URL
    pub fn css_url(&self) -> &str {
        &self.css_url
    }

    /// 获取解析器的引用
    pub fn parser(&self) -> &P {
        &self.parser
    }

    /// 获取渲染器的引用
    pub fn renderer(&self) -> &R {
        &self.renderer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lang_core::{Anchor, Category};

    struct TestParser;
    impl Parser for TestParser {
        fn parse(&self, input: &str) -> Category {
            let mut doc = Category::new();
            doc.push(Anchor::Paragraph {
                content: input.to_string(),
            });
            doc
        }
    }

    struct TestRenderer;
    impl Renderer for TestRenderer {
        fn render(&self, doc: &Category, title: &str, css_url: &str) -> String {
            format!(
                "<!DOCTYPE html><html><head><title>{}</title><link href='{}'></head><body>{:?}</body></html>",
                title, css_url, doc
            )
        }
    }

    #[test]
    fn test_converter_new() {
        let converter = Converter::new(TestParser, TestRenderer, "custom.css".to_string());
        assert_eq!(converter.css_url(), "custom.css");
    }

    #[test]
    fn test_converter_convert() {
        let converter = Converter::new(TestParser, TestRenderer, "style.css".to_string());
        let html = converter.convert("Hello", "My Title");
        assert!(html.contains("<title>My Title</title>"));
        assert!(html.contains("Hello"));
        assert!(html.contains("style.css"));
    }
}
