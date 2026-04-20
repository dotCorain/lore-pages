//! 示例：使用 Converter

use lang_core::{Category, Anchor};
use lang_framework::{Converter, Parser, Renderer};

// 一个简单的解析器（只识别标题）
struct SimpleParser;

impl Parser for SimpleParser {
    fn parse(&self, input: &str) -> Category {
        let mut doc = Category::new();
        for line in input.lines() {
            if line.starts_with("# ") {
                if let Some(content) = line.strip_prefix("# ") {
                    doc.push(Anchor::Heading {
                        level: 1,
                        content: content.to_string(),
                    });
                }
            } else if !line.is_empty() {
                doc.push(Anchor::Paragraph {
                    content: line.to_string(),
                });
            }
        }
        doc
    }
}

// 一个简单的渲染器（输出纯文本格式）
struct SimpleRenderer;

impl Renderer for SimpleRenderer {
    fn render(&self, doc: &Category, title: &str, _css_url: &str) -> String {
        let mut output = format!("=== {} ===\n\n", title);
        for node in &doc.nodes {
            match node {
                Anchor::Heading { level, content } => {
                    output.push_str(&format!("{}. {}\n", level, content));
                }
                Anchor::Paragraph { content } => {
                    output.push_str(&format!("[P] {}\n", content));
                }
            }
        }
        output
    }
}

fn main() {
    let converter = Converter::new(SimpleParser, SimpleRenderer, "".to_string());

    let markdown = "\
# 我的文档
这是第一段。
这是第二段。";

    let result = converter.convert(markdown, "示例文档");
    println!("{}", result);
}
