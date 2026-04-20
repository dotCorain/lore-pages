//! 示例：使用 Converter

use lang_core::{Document, Node};
use lang_framework::{Converter, Parser, Renderer};

// 一个简单的解析器（只识别标题）
struct SimpleParser;

impl Parser for SimpleParser {
    fn parse(&self, input: &str) -> Document {
        let mut doc = Document::new();
        for line in input.lines() {
            if line.starts_with("# ") {
                if let Some(content) = line.strip_prefix("# ") {
                    doc.push(Node::Heading {
                        level: 1,
                        content: content.to_string(),
                    });
                }
            } else if !line.is_empty() {
                doc.push(Node::Paragraph {
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
    fn render(&self, doc: &Document, title: &str, _css_url: &str) -> String {
        let mut output = format!("=== {} ===\n\n", title);
        for node in &doc.nodes {
            match node {
                Node::Heading { level, content } => {
                    output.push_str(&format!("{}. {}\n", level, content));
                }
                Node::Paragraph { content } => {
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
