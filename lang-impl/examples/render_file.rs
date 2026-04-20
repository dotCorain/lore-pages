//! 示例：从文件读取并渲染为 HTML（占位示例）
//!
//! 该示例为占位用途，实际 CLI 工具会递归处理目录并使用 `Converter`。

use std::fs;
use std::path::PathBuf;

use lang_framework::{Parser, Renderer};
use lang_impl::HtmlRenderer;
use lang_parser::MarkdownParser;

fn main() {
    let path = PathBuf::from("example.lore");
    let content = fs::read_to_string(&path).unwrap_or_default();

    let parser = MarkdownParser;
    let doc = parser.parse(&content);

    let renderer = HtmlRenderer;
    let html = renderer.render(&doc, "从文件渲染", "style.css");
    println!("{}", html);
}
