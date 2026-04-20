//! 示例：如何遍历文档中的节点

use lang_core::{Category, Anchor};

fn main() {
    let mut doc = Category::new();

    doc.push(Anchor::Heading {
        level: 1,
        content: "文档".to_string(),
    });
    doc.push(Anchor::Paragraph {
        content: "第一段".to_string(),
    });
    doc.push(Anchor::Heading {
        level: 2,
        content: "章节".to_string(),
    });
    doc.push(Anchor::Paragraph {
        content: "第二段".to_string(),
    });

    // 方法1：使用 for 循环
    println!("方法1 - for 循环:");
    for (i, node) in doc.nodes.iter().enumerate() {
        println!("  节点 {}: {:?}", i, node);
    }

    // 方法2：使用函数式编程
    println!("\n方法2 - 函数式风格:");
    let heading_count = doc
        .nodes
        .iter()
        .filter(|node| matches!(node, Anchor::Heading { .. }))
        .count();
    println!("  标题数量: {}", heading_count);

    // 方法3：提取所有标题内容
    println!("\n方法3 - 提取标题:");
    let titles: Vec<&str> = doc
        .nodes
        .iter()
        .filter_map(|node| {
            if let Anchor::Heading { content, .. } = node {
                Some(content.as_str())
            } else {
                None
            }
        })
        .collect();
    for title in titles {
        println!("  标题: {}", title);
    }
}
