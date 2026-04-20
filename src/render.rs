use crate::framework::category_config::CategoryConfig;
use crate::framework::renderer::Renderer;
use crate::framework::renderer_config::RenderConfig;
use crate::ir::anchor::Anchor;
use crate::ir::category::Category;

// 渲染器实现：将 IR（Category/Anchor）转换为 HTML 字符串
pub struct HtmlRenderer;

impl Renderer for HtmlRenderer {
    fn render(
        &self,
        doc: &Category,
        _category_config: &CategoryConfig,
        renderer_config: &RenderConfig,
    ) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"zh-CN\">\n");
        html.push_str("<head>\n");
        html.push_str("<meta charset=\"utf-8\">\n");
        html.push_str(&format!(
            "<title>{}</title>\n",
            escape_html(&renderer_config.main_lang)
        ));
        html.push_str(&format!(
            "<link rel=\"stylesheet\" href=\"{}\">\n",
            escape_html(&renderer_config.css_url)
        ));
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");

        // 逐个渲染 Anchor 节点并追加到输出字符串
        for node in &doc.nodes {
            html.push_str(&render_node(node));
            html.push('\n');
        }

        html.push_str("</body>\n");
        html.push_str("</html>\n");

        html
    }
}

fn render_node(node: &Anchor) -> String {
    match node {
        Anchor::Heading { level, content } => {
            let tag = format!("h{}", level);
            format!("  <{}>{}</{}>", tag, escape_html(content), tag)
        }
        Anchor::Paragraph { content } => {
            if content.trim().is_empty() {
                // do not render empty paragraphs as <br>
                String::new()
            } else {
                format!("  <p>{}</p>", escape_html(content))
            }
        }
        Anchor::BreakLine => "  <br>".to_string(),
        Anchor::PlaceHolderLine { .. } => {
            // 占位符行不输出任何内容
            String::new()
        }
        Anchor::UrlLink { name, url } => {
            format!(
                "  <p style=\"margin-left: 20px\"><a href=\"{url}\" class=\"link_url\">{name}</a></p>"
            )
        }
        Anchor::Comment { content } => {
            format!("  <!-- {} -->", content)
        }
    }
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
