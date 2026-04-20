//! 渲染器 trait

use lang_core::Category;

/// 渲染器接口
///
/// 任何渲染器都需要实现这个 trait。
pub trait Renderer {
    /// 渲染文档并返回 HTML 字符串。
    ///
    /// # 参数
    ///
    /// - `doc`：待渲染的 `Category`（由解析器产生的 IR）。
    /// - `title`：页面标题，会用于 `<title>` 及页面头部展示。
    /// - `css_url`：样式表的 URL，用于在生成的 HTML 中插入 `<link>`。
    ///
    /// # 返回值
    ///
    /// 返回渲染后的 HTML 字符串（可以是完整页面或片段，取决于实现）。
    fn render(&self, doc: &Category, title: &str, css_url: &str) -> String;
}
