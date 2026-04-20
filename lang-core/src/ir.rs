//! IR 定义模块

/// Anchor Types
///
/// Each Anchor means a line, such as a title or a paragraph.
/// 
/// For example:
///
/// ```
/// use lang_core::Anchor;
///
/// let heading = Anchor::Heading {
///     level: 2,
///     content: "Some 2 Level Title".to_string(),
/// };
///
/// let paragraph = Anchor::Paragraph {
///     content: "Some commen peregraph.".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Anchor {
    /// Title Anchor
    ///
    /// `level` should less than 4 and more than 1.
    Heading {
        /// title level: from 1 to 4
        level: u8,
        /// content in title
        content: String,
    },
    /// Paragraph Anchor
    ///
    /// common text
    Paragraph {
        /// content in the text
        content: String,
    },
}

/// Category Types
///
/// `Category` 是一个包含多个 `Anchor` 的容器。
///
/// For Example
///
/// ```
/// use lang_core::{Category, Anchor};
///
/// let mut doc = Category::new();
/// doc.nodes.push(Anchor::Heading {
///     level: 1,
///     content: "Some File".to_string(),
/// });
/// doc.nodes.push(Anchor::Paragraph {
///     content: "Welcome!".to_string(),
/// });
///
/// assert_eq!(doc.nodes.len(), 2);
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Category {
    pub nodes: Vec<Anchor>,
}

impl Category {
    /// 创建一个新的空文档
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// 添加一个节点到文档末尾
    pub fn push(&mut self, node: Anchor) {
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
        let doc = Category::new();
        assert!(doc.is_empty());
        assert_eq!(doc.len(), 0);
    }

    #[test]
    fn test_push_node() {
        let mut doc = Category::new();
        doc.push(Anchor::Heading {
            level: 1,
            content: "Title".to_string(),
        });
        assert_eq!(doc.len(), 1);
        assert!(!doc.is_empty());
    }

    #[test]
    fn test_heading_node() {
        let heading = Anchor::Heading {
            level: 3,
            content: "Hello".to_string(),
        };

        match heading {
            Anchor::Heading { level, content } => {
                assert_eq!(level, 3);
                assert_eq!(content, "Hello");
            }
            _ => panic!("Expected heading node"),
        }
    }

    #[test]
    fn test_paragraph_node() {
        let paragraph = Anchor::Paragraph {
            content: "Some text".to_string(),
        };

        match paragraph {
            Anchor::Paragraph { content } => {
                assert_eq!(content, "Some text");
            }
            _ => panic!("Expected paragraph node"),
        }
    }
}
