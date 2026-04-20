//! 示例：如何创建一个文档

use lang_core::{Document, Node};

fn main() {
    // 创建一个新文档
    let mut doc = Document::new();
    
    // 添加一级标题
    doc.push(Node::Heading {
        level: 1,
        content: "我的第一个文档".to_string(),
    });
    
    // 添加一个段落
    doc.push(Node::Paragraph {
        content: "这是一个用 lang-core 创建的文档。".to_string(),
    });
    
    // 添加二级标题
    doc.push(Node::Heading {
        level: 2,
        content: "详细介绍".to_string(),
    });
    
    // 添加另一个段落
    doc.push(Node::Paragraph {
        content: "这里是详细内容……".to_string(),
    });
    
    // 打印文档信息
    println!("文档包含 {} 个节点：", doc.len());
    for node in &doc.nodes {
        match node {
            Node::Heading { level, content } => {
                println!("  标题{}: {}", level, content);
            }
            Node::Paragraph { content } => {
                println!("  段落: {}", content);
            }
        }
    }
}