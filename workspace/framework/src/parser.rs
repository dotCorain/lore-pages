//! 解析器 trait

use lang_core::Category;

/// 解析器接口
///
/// 任何解析器都需要实现这个 trait。
pub trait Parser {
    /// 解析输入的字符串，返回一个 `Category`（中间表示）。
    ///
    /// # 参数
    ///
    /// - `input`：要解析的文本内容，支持多行输入。
    ///
    /// # 返回值
    ///
    /// 返回解析后的 `Category`，表示结构化的文档节点序列。
    fn parse(&self, input: &str) -> Category;
}
