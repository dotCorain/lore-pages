//! Markdown 解析器的具体实现

use lang_core::{Category, Anchor};
use lang_framework::Parser;

/// 解析标题行
///
/// # 示例
///
/// ```
/// use lang_parser::parser::parse_heading;
///
/// assert_eq!(parse_heading("# 标题"), Some((1, "标题".to_string())));
/// assert_eq!(parse_heading("## 二级"), Some((2, "二级".to_string())));
/// assert_eq!(parse_heading("### 三级标题"), Some((3, "三级标题".to_string())));
/// assert_eq!(parse_heading("#### 四级"), Some((4, "四级".to_string())));
///
/// assert_eq!(parse_heading("##### 五级标题"), None);
/// assert_eq!(parse_heading("#没有空格"), None);
/// assert_eq!(parse_heading("普通文本"), None);
/// ```
/// Markdown 解析器 — 一个非常简化的 Markdown 实现。
///
/// 实现了 `lang_framework::Parser` trait，将 Markdown 文本逐行解析为
/// `lang_core::Category`（由 `Anchor` 构成的节点序列）。
///
/// 目前实现仅支持：标题（1-4 级，格式须为 `# ` 后跟空格）和单行段落。
///
/// 示例：
/// ```rust
/// use lang_parser::MarkdownParser;
/// use lang_framework::Parser;
///
/// let parser = MarkdownParser;
/// let doc = parser.parse("# 标题\n第一段");
/// ```
pub struct MarkdownParser;

impl Parser for MarkdownParser {
    /// 解析 Markdown 文本，返回文档
    ///
    /// # 算法说明
    ///
    /// 1. 按行分割输入文本
    /// 2. 对每一行：
    ///    - 如果匹配标题模式（#、##、###、####），生成标题节点
    ///    - 如果行非空，生成段落节点
    ///    - 空行会被忽略
    ///
    /// 注意：这是一个简化实现，真正的 Markdown 需要考虑多行段落、列表等。
    fn parse(&self, input: &str) -> Category {
        let mut doc = Category::new();

        for line in input.lines() {
            // 不再跳过空行，空行也作为段落处理
            if let Some((level, content)) = parse_heading(line) {
                doc.push(Anchor::Heading { level, content });
            } else {
                // 空行会变成内容为空的 Paragraph
                doc.push(Anchor::Paragraph {
                    content: line.to_string(),
                });
            }
        }

        doc
    }
}

/// 解析标题行
///
/// 检查一行文本是否是 Markdown 标题。
///
/// # 参数
///
/// * `line` - 要检查的行
///
/// # 返回值
///
/// 如果是标题，返回 `Some((级别, 标题内容))`；否则返回 `None`
///
/// # 示例
///
/// ```
/// use lang_parser::parse_heading;
///
/// assert_eq!(parse_heading("# 标题"), Some((1, "标题".to_string())));
/// assert_eq!(parse_heading("## 二级"), Some((2, "二级".to_string())));
/// assert_eq!(parse_heading("### 三级标题"), Some((3, "三级标题".to_string())));
/// assert_eq!(parse_heading("#### 四级"), Some((4, "四级".to_string())));
///
/// assert_eq!(parse_heading("##### 五级标题"), None);  // 不支持5级
/// assert_eq!(parse_heading("#没有空格"), None);      // 缺少空格
/// assert_eq!(parse_heading("普通文本"), None);
/// ```
pub fn parse_heading(line: &str) -> Option<(u8, String)> {
    let bytes = line.as_bytes();
    let mut count = 0;

    // 统计开头的 # 个数（最多4个）
    for &b in bytes.iter().take(4) {
        if b == b'#' {
            count += 1;
        } else {
            break;
        }
    }

    // 没有 #
    if count == 0 {
        return None;
    }

    // 检查 # 后面是否有空格
    if bytes.len() > count && bytes[count] == b' ' {
        let content = line[count + 1..].to_string();
        Some((count as u8, content))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lang_framework::Parser;

    // ========== parse_heading 函数测试 ==========

    #[test]
    fn test_parse_heading_level_1() {
        let result = parse_heading("# 一级标题");
        assert_eq!(result, Some((1, "一级标题".to_string())));
    }

    #[test]
    fn test_parse_heading_level_2() {
        let result = parse_heading("## 二级标题");
        assert_eq!(result, Some((2, "二级标题".to_string())));
    }

    #[test]
    fn test_parse_heading_level_3() {
        let result = parse_heading("### 三级标题");
        assert_eq!(result, Some((3, "三级标题".to_string())));
    }

    #[test]
    fn test_parse_heading_level_4() {
        let result = parse_heading("#### 四级标题");
        assert_eq!(result, Some((4, "四级标题".to_string())));
    }

    #[test]
    fn test_parse_heading_level_5_should_fail() {
        let result = parse_heading("##### 五级标题");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_heading_no_space() {
        let result = parse_heading("#没有空格");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_heading_multiple_hashes_without_space() {
        let result = parse_heading("###no space");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_heading_extra_spaces() {
        let result = parse_heading("#   多个空格");
        assert_eq!(result, Some((1, "  多个空格".to_string())));
        // 注意：内容开头的空格会被保留
    }

    #[test]
    fn test_parse_heading_not_heading() {
        let result = parse_heading("普通文本");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_heading_empty_line() {
        let result = parse_heading("");
        assert_eq!(result, None);
    }

    // ========== Parser 整体测试 ==========

    #[test]
    fn test_parse_single_heading() {
        let parser = MarkdownParser;
        let doc = parser.parse("# Hello World");

        assert_eq!(doc.nodes.len(), 1);
        match &doc.nodes[0] {
            Anchor::Heading { level, content } => {
                assert_eq!(*level, 1);
                assert_eq!(content, "Hello World");
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn test_parse_multiple_headings() {
        let parser = MarkdownParser;
        let input = "\
# Title
## Section
### Subsection
#### Detail";

        let doc = parser.parse(input);
        assert_eq!(doc.nodes.len(), 4);

        // 验证级别
        let expected_levels = [1, 2, 3, 4];
        for (i, node) in doc.nodes.iter().enumerate() {
            match node {
                Anchor::Heading { level, .. } => {
                    assert_eq!(*level, expected_levels[i]);
                }
                _ => panic!("Expected heading"),
            }
        }
    }

    #[test]
    fn test_parse_paragraphs() {
        let parser = MarkdownParser;
        let doc = parser.parse("This is a paragraph.\nAnother paragraph.");

        assert_eq!(doc.nodes.len(), 2);
        for node in &doc.nodes {
            match node {
                Anchor::Paragraph { .. } => {}
                _ => panic!("Expected paragraph"),
            }
        }
    }

    #[test]
    fn test_parse_mixed_content() {
        let parser = MarkdownParser;
        let input = "\
# Welcome
This is the first paragraph.
## Introduction
Here is some content.
#### A small note
Final paragraph.";

        let doc = parser.parse(input);
        // 原：期望 5 个节点（没有空行的情况）
        // 现：没有空行，所以还是 6 个节点（H1, P, H2, P, H4, P）
        assert_eq!(doc.nodes.len(), 6);

        // 验证节点类型序列
        let types: Vec<&str> = doc
            .nodes
            .iter()
            .map(|node| match node {
                Anchor::Heading { .. } => "heading",
                Anchor::Paragraph { .. } => "paragraph",
            })
            .collect();

        assert_eq!(
            types,
            vec![
                "heading",
                "paragraph",
                "heading",
                "paragraph",
                "heading",
                "paragraph"
            ]
        );
    }

    #[test]
    fn test_parse_handles_empty_lines() {
        let parser = MarkdownParser;
        let input = "\
# Title

Paragraph after empty line

## Section";

        let doc = parser.parse(input);
        // 原：期望 3 个节点（Title、Paragraph、Section）
        // 现：空行也产生节点，所以是 Title + 空行段落 + Paragraph + 空行段落 + Section = 5 个
        assert_eq!(doc.nodes.len(), 5);
    }

    #[test]
    fn test_parse_empty_input() {
        let parser = MarkdownParser;
        let doc = parser.parse("");
        assert_eq!(doc.nodes.len(), 0);
    }

    #[test]
    fn test_parse_only_empty_lines() {
        let parser = MarkdownParser;
        let doc = parser.parse("\n\n\n");
        // 原：期望 0 个节点
        // 现：3 个空行变成 3 个空段落
        assert_eq!(doc.nodes.len(), 3);
    }

    #[test]
    fn test_parse_heading_with_trailing_spaces() {
        let parser = MarkdownParser;
        let doc = parser.parse("# Title with trailing spaces   ");

        match &doc.nodes[0] {
            Anchor::Heading { content, .. } => {
                // 注意：内容末尾的空格会被保留
                assert_eq!(content, "Title with trailing spaces   ");
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn test_parse_heading_with_inline_hashes() {
        let parser = MarkdownParser;
        let doc = parser.parse("# This is # not a heading");

        match &doc.nodes[0] {
            Anchor::Heading { content, .. } => {
                // 内容中的 # 应该保留
                assert_eq!(content, "This is # not a heading");
            }
            _ => panic!("Expected heading"),
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use lang_framework::Parser;

    /// 测试一个完整的文档示例
    #[test]
    fn test_full_document() {
        let parser = MarkdownParser;
        let input = "\
# My Category

This is the introduction.

## First Chapter

Content of first chapter.

### Subsection

Details here.

#### Important Note

A note to remember.

## Second Chapter

More content.";

        let doc = parser.parse(input);

        // 原：期望 10 个节点（忽略空行）
        // 现：每个空行都是一个段落，所以数量更多
        // 我们来计算：H1, 空行, P, 空行, H2, 空行, P, 空行, H3, 空行, P, 空行, H4, 空行, P, 空行, H2, 空行, P = 19 个
        assert_eq!(doc.nodes.len(), 19);

        // 验证第一个节点是 H1
        match &doc.nodes[0] {
            Anchor::Heading { level, content } => {
                assert_eq!(*level, 1);
                assert_eq!(content, "My Category");
            }
            _ => panic!("First node should be heading"),
        }

        // 第二个节点应该是空段落（因为空行）
        match &doc.nodes[1] {
            Anchor::Paragraph { content } => {
                assert_eq!(content, "");
            }
            _ => panic!("Second node should be empty paragraph"),
        }

        // 统计标题数量（应该不变，还是 5 个）
        let heading_count = doc
            .nodes
            .iter()
            .filter(|node| matches!(node, Anchor::Heading { .. }))
            .count();
        assert_eq!(heading_count, 5);
    }
}
