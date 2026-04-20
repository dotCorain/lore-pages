use crate::framework::renderer::Renderer;
use crate::ir::anchor::Anchor;
use crate::ir::category::Category;
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
            if content.is_empty() {
                "  <br>".to_string()
            } else {
                format!("  <p>{}</p>", escape_html(content))
            }
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
