//! IR 定义模块

/// 文档节点类型
/// 
/// 每个节点代表文档中的一个元素，比如标题或段落。
/// 
/// # 示例
/// 
/// ```
/// use lang_core::Node;
/// 
/// let heading = Node::Heading {
///     level: 2,
///     content: "这是二级标题".to_string(),
/// };
/// 
/// let paragraph = Node::Paragraph {
///     content: "这是普通段落。".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// 标题节点
    /// 
    /// `level` 的取值范围是 1-4，对应 Markdown 中的 #、##、###、####
    Heading {
        /// 标题级别：1 到 4
        level: u8,
        /// 标题的文本内容
        content: String,
    },
    /// 段落节点
    /// 
    /// 普通的文本段落
    Paragraph {
        /// 段落的文本内容
        content: String,
    },
}

/// 整个文档
/// 
/// `Document` 是一个包含多个 `Node` 的容器。
/// 
/// # 示例
/// 
/// ```
/// use lang_core::{Document, Node};
/// 
/// let mut doc = Document::new();
/// doc.nodes.push(Node::Heading {
///     level: 1,
///     content: "我的文档".to_string(),
/// });
/// doc.nodes.push(Node::Paragraph {
///     content: "欢迎阅读我的文档。".to_string(),
/// });
/// 
/// assert_eq!(doc.nodes.len(), 2);
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Document {
    pub nodes: Vec<Node>,
}

impl Document {
    /// 创建一个新的空文档
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    /// 添加一个节点到文档末尾
    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// 检查文档是否为空
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// 获取文档中的节点数量
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_document_is_empty() {
        let doc = Document::new();
        assert!(doc.is_empty());
        assert_eq!(doc.len(), 0);
    }

    #[test]
    fn test_push_node() {
        let mut doc = Document::new();
        doc.push(Node::Heading {
            level: 1,
            content: "Title".to_string(),
        });
        assert_eq!(doc.len(), 1);
        assert!(!doc.is_empty());
    }

    #[test]
    fn test_heading_node() {
        let heading = Node::Heading {
            level: 3,
            content: "Hello".to_string(),
        };
        
        match heading {
            Node::Heading { level, content } => {
                assert_eq!(level, 3);
                assert_eq!(content, "Hello");
            }
            _ => panic!("Expected heading node"),
        }
    }

    #[test]
    fn test_paragraph_node() {
        let paragraph = Node::Paragraph {
            content: "Some text".to_string(),
        };
        
        match paragraph {
            Node::Paragraph { content } => {
                assert_eq!(content, "Some text");
            }
            _ => panic!("Expected paragraph node"),
        }
    }
}