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

        html.push_str(
            "<!DOCTYPE html>\n",
        );
        html.push_str(
            format!(
                "<html lang=\"{}\">\n",
                &renderer_config
                    .main_lang
            )
            .as_str(),
        );
        html.push_str("<head>\n");
        html.push_str("<meta charset=\"utf-8\">\n");
        html.push_str(&format!(
            "<title>{}</title>\n",
            escape_html(
                &renderer_config
                    .main_lang
            )
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
            html.push_str(
                &render_node(
                    node,
                    renderer_config,
                ),
            );
            html.push('\n');
        }

        html.push_str("</body>\n");
        html.push_str("</html>\n");

        html
    }
}

fn render_node(
    node: &Anchor,
    renderer_config: &RenderConfig,
) -> String {
    match node {
        Anchor::Heading {
            level,
            content,
        } => {
            let tag =
                format!("h{}", level);
            format!(
                "  <{}>{}</{}>",
                tag,
                escape_html(content),
                tag
            )
        }
        Anchor::Paragraph {
            content,
        } => {
            if content.trim().is_empty()
            {
                // do not render empty paragraphs as <br>
                String::new()
            } else {
                format!(
                    "  <p>{}</p>",
                    escape_html(
                        content
                    )
                )
            }
        }
        Anchor::BreakLine => {
            "  <br>".to_string()
        }
        Anchor::PlaceHolderLine {
            ..
        } => {
            // 占位符行不输出任何内容
            String::new()
        }
        Anchor::EmptyLine => {
            "".to_string()
        }
        Anchor::UrlLink {
            name,
            url,
        } => {
            format!(
                "  <p style=\"margin-left: 20px\"><a href=\"{}\" class=\"link_url\">{}</a></p>",
                escape_html(url),
                escape_html(name)
            )
        }
        Anchor::LoreLink {
            name,
            path,
        } => {
            let href =
                if renderer_config
                    .link_base
                    .is_empty()
                {
                    path.clone()
                } else {
                    // if path is absolute URL, do not prepend
                    if path.starts_with(
                        "http://",
                    ) || path
                        .starts_with(
                            "https://",
                        )
                    {
                        path.clone()
                    } else {
                        let base = renderer_config
                            .link_base
                            .trim_end_matches('/');
                        let p =
                            path.trim_start_matches('/');
                        format!(
                            "{}/{}",
                            base, p
                        )
                    }
                };

            format!(
                "  <p style=\"margin-left: 20px\"><a href=\"{}\" class=\"link_lore\">{}</a></p>",
                escape_html(&href),
                escape_html(name)
            )
        }
        Anchor::Comment { content } => {
            format!(
                "  <!-- {} -->",
                content
            )
        }
        Anchor::Image { url } => {
            format!(
                "  <img src=\"{}\">",
                url
            )
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
