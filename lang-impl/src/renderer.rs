//! HTML 渲染器的具体实现

use lang_core::{Category, Anchor};
use lang_framework::Renderer;

/// `HtmlRenderer` — 将 `lang_core::Category` 渲染为完整的 HTML 页面实现。
///
/// 行为说明：
/// - 生成基本的 HTML 结构（`<!DOCTYPE html>`、`<head>`、`<title>`、`<link>` 等）
/// - 将 `Anchor::Heading` 渲染为对应的 `<hN>` 标签，`Anchor::Paragraph` 渲染为 `<p>`（空段落渲染为 `<br>`）
///
/// 注意：此实现为简化示例，不处理复杂 Markdown 结构（如列表、多行段落、代码块、内联格式等）。
///
/// 示例：
/// ```rust
/// use lang_core::{Category, Anchor};
/// use lang_impl::HtmlRenderer;
/// use lang_framework::Renderer;
///
/// let mut doc = Category::new();
/// doc.push(Anchor::Heading { level: 1, content: "标题".to_string() });
/// let renderer = HtmlRenderer;
/// let html = renderer.render(&doc, "页面", "style.css");
/// ```
pub struct HtmlRenderer;

impl Renderer for HtmlRenderer {
    fn render(&self, doc: &Category, title: &str, css_url: &str) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"zh-CN\">\n");
        html.push_str("<head>\n");
        html.push_str("<meta charset=\"utf-8\">\n");
        html.push_str(&format!("<title>{}</title>\n", escape_html(title)));
        html.push_str(&format!(
            "<link rel=\"stylesheet\" href=\"{}\">\n",
            escape_html(css_url)
        ));
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str(&format!(
            "<h1 class=\"page-title\">{}</h1>\n",
            escape_html(title)
        ));

        for node in &doc.nodes {
            html.push_str(&render_node(node));
            html.push('\n');
        }

        html.push_str("</body>\n");
        html.push_str("</html>\n");

        html
    }
}

/// 渲染单个节点
/// 渲染单个节点为 HTML 片段。
///
/// - `node`: 要渲染的 `Anchor`（例如标题或段落）。
///
/// 返回值为对应的 HTML 字符串片段（不包含文档头尾结构）。
fn render_node(node: &Anchor) -> String {
    match node {
        Anchor::Heading { level, content } => {
            let tag = format!("h{}", level);
            format!("  <{}>{}</{}>", tag, escape_html(content), tag)
        }
        Anchor::Paragraph { content } => {
            if content.is_empty() {
                "  <br>".to_string()
            } else {
                format!("  <p>{}</p>", escape_html(content))
            }
        }
    }
}

/// 转义 HTML 特殊字符
/// 转义 HTML 特殊字符以保证输出安全与正确显示。
///
/// 本函数将 `&`, `<`, `>`, `"`, `'` 等字符替换为对应的 HTML 实体，
/// 避免在插入用户或文档内容时破坏 HTML 结构或造成 XSS 风险（此处为简单替换实现）。
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use lang_core::Category;

    #[test]
    fn test_render_with_title_and_css() {
        let mut doc = Category::new();
        doc.push(Anchor::Heading {
            level: 1,
            content: "Hello".to_string(),
        });

        let renderer = HtmlRenderer;
        let html = renderer.render(&doc, "My Page", "custom.css");

        assert!(html.contains("<title>My Page</title>"));
        assert!(html.contains("custom.css"));
        assert!(html.contains("<h1 class=\"page-title\">My Page</h1>"));
        assert!(html.contains("<h1>Hello</h1>"));
    }
}
