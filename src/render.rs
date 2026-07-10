use std::path::Path;

use crate::framework::category_config::CategoryConfig;
use crate::framework::renderer::Renderer;
use crate::framework::renderer_config::RenderConfig;
use crate::ir::anchor::Anchor;
use crate::ir::category::Category;

pub struct HtmlRenderer;

impl Renderer for HtmlRenderer {
    fn render(
        &self,
        doc: &Category,
        _category_config: &CategoryConfig,
        renderer_config: &RenderConfig,
        source_path: Option<&str>,
    ) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str(&format!("<html lang=\"{}\">\n", &renderer_config.main_lang));
        html.push_str("<head>\n");
        html.push_str("<meta charset=\"utf-8\">\n");
        let page_title = doc.nodes.iter().find_map(|node| {
            if let Anchor::Heading { level: 1, content, .. } = node {
                Some(content.as_str())
            } else {
                None
            }
        }).unwrap_or(&renderer_config.site_title);
        html.push_str(&format!("<title>{}</title>\n", escape_html(page_title)));
        html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");

        for script in &renderer_config.scripts {
            html.push_str(&format!("<script src=\"{}\"></script>\n", escape_html(script)));
        }

        html.push_str("</head>\n");
        html.push_str("<body>\n");

        for node in &doc.nodes {
            html.push_str(&render_node(node, renderer_config, source_path));
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
    source_path: Option<&str>,
) -> String {
    match node {
        Anchor::Heading { level, content, link } => {
            let tag = format!("h{}", level);
            let body = escape_html(content);
            match link {
                Some(path) if *level == 2 => {
                    let resolved = resolve_link_from_source(path, source_path, renderer_config);
                    check_lore_link(content, &resolved, renderer_config, source_path);
                    let html_path = Path::new(&resolved).with_extension("html");
                    let href = resolve_path(html_path.to_str().unwrap_or(&resolved), renderer_config);
                    format!("  <{tag}><a href=\"{href}\" class=\"link_lore\">{body}</a></{tag}>")
                }
                _ => format!("  <{tag}>{body}</{tag}>"),
            }
        }
        Anchor::BreakLine => "  <br>".to_string(),
        Anchor::PlaceHolderLine { .. } => String::new(),
        Anchor::EmptyLine => String::new(),
        Anchor::InnerUrlOpen { title, url } => {
            format!(
                "<div class=\"foldable expanded\" data-url=\"{}\" data-title=\"{}\"></div>",
                escape_html(url), escape_html(title)
            )
        }
        Anchor::InnerUrlClose { title, url } => {
            if title.is_empty() {
                String::new()
            } else {
                format!(
                    "<div class=\"foldable\" data-url=\"{}\" data-title=\"{}\"></div>",
                    escape_html(url), escape_html(title)
                )
            }
        }
        Anchor::InnerLoreOpen { title, path } => {
            let href = resolve_path(path, renderer_config);
            format!(
                "<div class=\"foldable lore expanded\" data-url=\"{}\" data-title=\"{}\"></div>",
                escape_html(&href), escape_html(title)
            )
        }
        Anchor::InnerLoreClose { title, path } => {
            if title.is_empty() {
                String::new()
            } else {
                let href = resolve_path(path, renderer_config);
                format!(
                    "<div class=\"foldable lore\" data-url=\"{}\" data-title=\"{}\"></div>",
                    escape_html(&href), escape_html(title)
                )
            }
        }
        Anchor::UrlLink { name, url } => {
            format!(
                "  <p style=\"margin-left: 20px\"><a href=\"{}\" class=\"link_url\">{}</a></p>",
                escape_html(url), escape_html(name)
            )
        }
        Anchor::LoreLink { name, path } => {
            let resolved = resolve_link_from_source(path, source_path, renderer_config);
            check_lore_link(name, &resolved, renderer_config, source_path);
            let html_path = Path::new(&resolved).with_extension("html");
            let href = resolve_path(html_path.to_str().unwrap_or(&resolved), renderer_config);
            format!(
                "  <p style=\"margin-left: 20px\"><a href=\"{}\" class=\"link_lore\">{}</a></p>",
                escape_html(&href), escape_html(name)
            )
        }
        Anchor::Comment { content } => format!("  <!-- {} -->", content),
        Anchor::Image { url } => format!("  <img src=\"{}\">", url),
        Anchor::Paragraph { content } => {
            if content.trim().is_empty() {
                String::new()
            } else {
                format!("  <p>{}</p>", escape_html(content))
            }
        }
    }
}

/// 基于源文件所在目录修正链接路径。
/// 例如：源文件 foo/bar/index.lore，链接目标 buz/index
/// → foo/bar/buz/index
fn resolve_link_from_source(
    link_path: &str,
    source_path: Option<&str>,
    renderer_config: &RenderConfig,
) -> String {
    let src = match source_path {
        Some(s) => s,
        None => return link_path.to_string(),
    };
    let src_path = Path::new(src);
    let parent = match src_path.parent() {
        Some(p) => p,
        None => return link_path.to_string(),
    };
    let base = Path::new(&renderer_config.from_lore_path);
    let rel = match parent.strip_prefix(base) {
        Ok(r) => r,
        Err(_) => parent,
    };
    let rel_str = rel.to_string_lossy();
    if rel_str.is_empty() || rel_str == "." {
        link_path.to_string()
    } else {
        format!("{}/{}", rel_str, link_path)
    }
}

/// 校验 lore 链接目标文件是否存在。
fn check_lore_link(
    name: &str,
    path: &str,
    renderer_config: &RenderConfig,
    source_path: Option<&str>,
) {
    if renderer_config.from_lore_path.is_empty() { return; }
    let base = Path::new(&renderer_config.from_lore_path);
    let src_file = {
        let p = base.join(path);
        if p.extension().is_none() { p.with_extension("lore") } else { p }
    };
    if src_file.exists() { return; }

    let parent = src_file.parent().unwrap_or(Path::new("."));
    if parent != base && !parent.exists() {
        eprintln!(
            "Warning: lore link \"{name}\" -> \"{path}\"\n  in source file: {}\n  folder not found: {:?}",
            source_path.unwrap_or("<unknown>"), parent,
        );
    } else {
        let filename = src_file.file_name().unwrap_or_default();
        eprintln!(
            "Warning: lore link \"{name}\" -> \"{path}\"\n  in source file: {}\n  folder exists, but {:?} not found in {:?}",
            source_path.unwrap_or("<unknown>"), filename, parent,
        );
    }
}

fn resolve_path(path: &str, renderer_config: &RenderConfig) -> String {
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
