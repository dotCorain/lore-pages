//! 示例：解析文件中的 Markdown

// 需要导入 Node 类型
use lang_core::Node;

use std::fs;
use std::path::PathBuf;

use lang_framework::Parser;
use lang_parser::MarkdownParser;

fn main() {
    // 创建一个示例 Markdown 文件
    let example_content = "\
# 示例文档

这是用 parse_file 示例解析的内容。

## 如何工作

1. 读取文件内容
2. 使用 MarkdownParser 解析
3. 输出结构化的节点信息

### 支持的语法

#### 标题级别 1-4

普通文本会被解析为段落。
";

    // 保存到临时文件
    let temp_file = PathBuf::from("example.md");
    fs::write(&temp_file, example_content).expect("Failed to write example file");

    println!("已创建文件: {:?}", temp_file);
    println!("{}\n", "-".repeat(50));

    // 读取并解析
    let content = fs::read_to_string(&temp_file).expect("Failed to read file");
    let parser = MarkdownParser;
    let doc = parser.parse(&content);

    println!("文件大小: {} 字节", content.len());
    println!("节点数量: {}\n", doc.nodes.len());

    // 输出统计信息
    let mut headings = 0;
    let mut paragraphs = 0;

    for node in &doc.nodes {
        match node {
            Node::Heading { level, content } => {
                headings += 1;
                println!("  H{}: {}", level, content);
            }
            Node::Paragraph { content } => {
                paragraphs += 1;
                let preview = if content.len() > 50 {
                    format!("{}...", &content[..47])
                } else {
                    content.clone()
                };
                println!("  P:  {}", preview);
            }
        }
    }

    println!("\n统计: {} 个标题, {} 个段落", headings, paragraphs);

    // 清理临时文件
    fs::remove_file(temp_file).expect("Failed to remove example file");
    println!("\n已清理临时文件");
}
