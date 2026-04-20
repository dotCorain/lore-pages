//! 示例：基本的解析功能

use lang_core::Node;
use lang_parser::MarkdownParser;
use lang_framework::Parser;

fn main() {
    let parser = MarkdownParser;
    
    let markdown = "\
# 我的第一个文档

欢迎使用 lang-parser！

## 特性

- 支持标题（1-4级）
- 支持段落
- 空行会被忽略

### 代码示例

```rust
let parser = MarkdownParser;
let doc = parser.parse(\"# Hello\");
注意事项

目前只实现了标题和段落。";

    println!("原始 Markdown:\n{}\n", markdown);
    println!("{}", "-".repeat(50));

    let doc = parser.parse(markdown);

    println!("\n解析结果（共 {} 个节点）：\n", doc.nodes.len());

    for (i, node) in doc.nodes.iter().enumerate() {
        print!("{:2}. ", i + 1);
        match node {
            Node::Heading { level, content } => {
                println!("[H{}] {}", level, content);
            }
            Node::Paragraph { content } => {
                // 截断过长的内容
                let display = if content.len() > 60 {
                    format!("{}...", &content[..57])
                } else {
                    content.clone()
                };
                println!("[P] {}", display);
            }
        }
    }
}
