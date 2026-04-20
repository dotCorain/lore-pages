//! 示例：使用 `HtmlRenderer` 渲染一个简单文档（占位示例）

use lang_core::{Category, Anchor};
use lang_framework::Renderer;
use lang_impl::HtmlRenderer;

fn main() {
    let mut doc = Category::new();
    doc.push(Anchor::Heading {
        level: 1,
        content: "示例标题".to_string(),
    });

    doc.push(Anchor::Paragraph {
        content: "这是一个简单示例段落。".to_string(),
    });

    let renderer = HtmlRenderer;
    let html = renderer.render(&doc, "示例页面", "style.css");
    println!("{}", html);
}
