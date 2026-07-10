use std::path::Path;

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
        html.push_str(&format!("<html lang=\"{}\">\n", &renderer_config.main_lang));
        html.push_str("<head>\n");
        html.push_str("<meta charset=\"utf-8\">\n");
        // 取第一个一级标题作为页面标题，没有则用配置中的 site_title
        let page_title = doc.nodes.iter().find_map(|node| {
            if let Anchor::Heading { level: 1, content } = node {
                Some(content.as_str())
            } else {
                None
            }
        }).unwrap_or(&renderer_config.site_title);
        html.push_str(&format!("<title>{}</title>\n", escape_html(page_title)));
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");

        // 加载脚本
        for script in &renderer_config.scripts {
            html.push_str(&format!("<script src=\"{}\"></script>\n", escape_html(script)));
        }

        html.push_str("</head>\n");
        html.push_str("<body>\n");

        for node in &doc.nodes {
            html.push_str(&render_node(node, renderer_config));
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
        Anchor::Heading { level, content } => {
            let tag = format!("h{}", level);
            format!("  <{}>{}</{}>", tag, escape_html(content), tag)
        }
        Anchor::BreakLine => "  <br>".to_string(),
        Anchor::PlaceHolderLine { .. } => String::new(),
        Anchor::EmptyLine => String::new(),
        Anchor::InnerUrlOpen { title, url } => {
            format!(
                "<div class=\"foldable expanded\" data-url=\"{}\" data-title=\"{}\"></div>",
                escape_html(url),
                escape_html(title)
            )
        }
        Anchor::InnerUrlClose { title, url } => {
            if title.is_empty() {
                String::new()
            } else {
                format!(
                    "<div class=\"foldable\" data-url=\"{}\" data-title=\"{}\"></div>",
                    escape_html(url),
                    escape_html(title)
                )
            }
        }
        Anchor::InnerLoreOpen { title, path } => {
            let href = resolve_path(path, renderer_config);
            format!(
                "<div class=\"foldable lore expanded\" data-url=\"{}\" data-title=\"{}\"></div>",
                escape_html(&href),
                escape_html(title)
            )
        }
        Anchor::InnerLoreClose { title, path } => {
            if title.is_empty() {
                String::new()
            } else {
                let href = resolve_path(path, renderer_config);
                format!(
                    "<div class=\"foldable lore\" data-url=\"{}\" data-title=\"{}\"></div>",
                    escape_html(&href),
                    escape_html(title)
                )
            }
        }
        Anchor::UrlLink { name, url } => {
            format!(
                "  <p style=\"margin-left: 20px\"><a href=\"{}\" class=\"link_url\">{}</a></p>",
                escape_html(url),
                escape_html(name)
            )
        }
        Anchor::LoreLink { name, path } => {
            // 校验源文件是否存在（自动补 .lore 扩展名）
            if !renderer_config.from_lore_path.is_empty() {
                let src_file = {
                    let p = Path::new(&renderer_config.from_lore_path).join(path);
                    if p.extension().is_none() {
                        p.with_extension("lore")
                    } else {
                        p
                    }
                };
                if !src_file.exists() {
                    eprintln!("Warning: linked lore file not found: {:?}", src_file);
                }
            }
            // href 指向对应的 .html
            let html_path = Path::new(path).with_extension("html");
            let href = resolve_path(html_path.to_str().unwrap_or(path), renderer_config);
            format!(
                "  <p style=\"margin-left: 20px\"><a href=\"{}\" class=\"link_lore\">{}</a></p>",
                escape_html(&href),
                escape_html(name)
            )
        }
        Anchor::Comment { content } => {
            format!("  <!-- {} -->", content)
        }
        Anchor::Image { url } => {
            format!("  <img src=\"{}\">", url)
        }
        Anchor::Paragraph { content } => {
            if content.trim().is_empty() {
                String::new()
            } else {
                format!("  <p>{}</p>", escape_html(content))
            }
        }
    }
}

// 解析路径：处理相对路径和绝对路径
fn resolve_path(
    path: &str,
    renderer_config: &RenderConfig,
) -> String {
    if renderer_config.link_base.is_empty() || path.starts_with("http://") || path.starts_with("https://") {
        path.to_string()
    } else {
        let base = renderer_config.link_base.trim_end_matches('/');
        let p = path.trim_start_matches('/');
        format!("{}/{}", base, p)
    }
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
