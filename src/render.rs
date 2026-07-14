use std::path::Path;

use crate::framework::category_config::CategoryConfig;
use crate::framework::renderer::Renderer;
use crate::framework::renderer_config::RenderConfig;
use crate::ir::anchor::Anchor;
use crate::ir::category::Category;

/// Default HTML renderer that converts a [`Category`] document into a
/// complete HTML5 page.
///
/// The output includes `<!DOCTYPE html>`, `<head>` with charset and
/// viewport meta tags, configurable `<title>`, optional `<script>`
/// includes, and a `<body>` containing the rendered document nodes.
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

        // Page title: use the first H1 heading, or fall back to site_title.
        let page_title = doc
            .nodes
            .iter()
            .find_map(|node| {
                if let Anchor::Heading {
                    level: 1, content, ..
                } = node
                {
                    Some(content.as_str())
                } else {
                    None
                }
            })
            .unwrap_or(&renderer_config.site_title);
        html.push_str(&format!("<title>{}</title>\n", escape_html(page_title)));
        html.push_str(
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n",
        );

        for script in &renderer_config.scripts {
            html.push_str(&format!(
                "<script src=\"{}\"></script>\n",
                escape_html(script)
            ));
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

/// Render a single [`Anchor`] node to its HTML representation.
fn render_node(
    node: &Anchor,
    renderer_config: &RenderConfig,
    source_path: Option<&str>,
) -> String {
    match node {
        Anchor::Heading {
            level,
            content,
            link,
        } => {
            let tag = format!("h{}", level);
            let body = escape_html(content);
            match link {
                Some(path) if *level == 2 => {
                    let resolved = resolve_link_from_source(path, source_path, renderer_config);
                    check_lore_link(content, &resolved, renderer_config, source_path);
                    let html_path = Path::new(&resolved).with_extension("html");
                    let href =
                        resolve_path(html_path.to_str().unwrap_or(&resolved), renderer_config);
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
            let resolved = resolve_link_from_source(path, source_path, renderer_config);
            check_lore_link(name, &resolved, renderer_config, source_path);
            let html_path = Path::new(&resolved).with_extension("html");
            let href = resolve_path(html_path.to_str().unwrap_or(&resolved), renderer_config);
            format!(
                "  <p style=\"margin-left: 20px\"><a href=\"{}\" class=\"link_lore\">{}</a></p>",
                escape_html(&href),
                escape_html(name)
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

/// Resolve a link path relative to the source file's directory.
///
/// For example, if the source file is `foo/bar/index.lore` and the link
/// target is `buz/index`, the resolved path becomes `foo/bar/buz/index`.
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

/// Validate that a Lore link target file exists on disk.
/// Emits a warning to stderr if the target is missing.
fn check_lore_link(
    name: &str,
    path: &str,
    renderer_config: &RenderConfig,
    source_path: Option<&str>,
) {
    if renderer_config.from_lore_path.is_empty() {
        return;
    }
    let base = Path::new(&renderer_config.from_lore_path);
    let src_file = {
        let p = base.join(path);
        if p.extension().is_none() {
            p.with_extension("lore")
        } else {
            p
        }
    };
    if src_file.exists() {
        return;
    }

    let parent = src_file.parent().unwrap_or(Path::new("."));
    if parent != base && !parent.exists() {
        eprintln!(
            "Warning: lore link \"{name}\" -> \"{path}\"\n  in source file: {}\n  folder not found: {:?}",
            source_path.unwrap_or("<unknown>"),
            parent,
        );
    } else {
        let filename = src_file.file_name().unwrap_or_default();
        eprintln!(
            "Warning: lore link \"{name}\" -> \"{path}\"\n  in source file: {}\n  folder exists, but {:?} not found in {:?}",
            source_path.unwrap_or("<unknown>"),
            filename,
            parent,
        );
    }
}

/// Apply the configured `link_base` prefix to a path.
///
/// Absolute URLs (starting with `http://` or `https://`) are returned
/// unchanged.
fn resolve_path(path: &str, renderer_config: &RenderConfig) -> String {
    if renderer_config.link_base.is_empty()
        || path.starts_with("http://")
        || path.starts_with("https://")
    {
        path.to_string()
    } else {
        let base = renderer_config.link_base.trim_end_matches('/');
        let p = path.trim_start_matches('/');
        format!("{}/{}", base, p)
    }
}

/// Escape special HTML characters in text content.
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// ── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> RenderConfig {
        RenderConfig::default()
    }

    fn render(doc: &Category) -> String {
        let renderer = HtmlRenderer;
        renderer.render(doc, &CategoryConfig::default(), &default_config(), None)
    }

    // ── escape_html ──────────────────────────────────────────────

    #[test]
    fn escape_html_ampersand() {
        assert_eq!(escape_html("a & b"), "a &amp; b");
    }

    #[test]
    fn escape_html_lt_gt() {
        assert_eq!(escape_html("<p>"), "&lt;p&gt;");
    }

    #[test]
    fn escape_html_quotes() {
        assert_eq!(escape_html("\"hello\""), "&quot;hello&quot;");
    }

    #[test]
    fn escape_html_plain_text() {
        assert_eq!(escape_html("hello"), "hello");
    }

    // ── render_node ──────────────────────────────────────────────

    #[test]
    fn render_heading_h1() {
        let node = Anchor::Heading {
            level: 1,
            content: "Title".into(),
            link: None,
        };
        let html = render_node(&node, &default_config(), None);
        assert!(html.contains("<h1>Title</h1>"));
    }

    #[test]
    fn render_heading_h2() {
        let node = Anchor::Heading {
            level: 2,
            content: "Section".into(),
            link: None,
        };
        let html = render_node(&node, &default_config(), None);
        assert!(html.contains("<h2>Section</h2>"));
    }

    #[test]
    fn render_heading_h2_with_link() {
        let node = Anchor::Heading {
            level: 2,
            content: "Section".into(),
            link: Some("Section/index".into()),
        };
        let cfg = default_config();
        let html = render_node(&node, &cfg, Some("Section/index.lore"));
        assert!(html.contains("href="));
        assert!(html.contains("Section"));
    }

    #[test]
    fn render_paragraph() {
        let node = Anchor::Paragraph {
            content: "Hello world".into(),
        };
        let html = render_node(&node, &default_config(), None);
        assert_eq!(html, "  <p>Hello world</p>");
    }

    #[test]
    fn render_empty_paragraph() {
        let node = Anchor::Paragraph {
            content: "   ".into(),
        };
        let html = render_node(&node, &default_config(), None);
        assert_eq!(html, "");
    }

    #[test]
    fn render_breakline() {
        let node = Anchor::BreakLine;
        let html = render_node(&node, &default_config(), None);
        assert_eq!(html, "  <br>");
    }

    #[test]
    fn render_empty_line() {
        let node = Anchor::EmptyLine;
        let html = render_node(&node, &default_config(), None);
        assert_eq!(html, "");
    }

    #[test]
    fn render_placeholder_line() {
        let node = Anchor::PlaceHolderLine {
            content: "any".into(),
        };
        let html = render_node(&node, &default_config(), None);
        assert_eq!(html, "");
    }

    #[test]
    fn render_comment() {
        let node = Anchor::Comment {
            content: "note".into(),
        };
        let html = render_node(&node, &default_config(), None);
        assert_eq!(html, "  <!-- note -->");
    }

    #[test]
    fn render_image() {
        let node = Anchor::Image {
            url: "https://example.com/a.png".into(),
        };
        let html = render_node(&node, &default_config(), None);
        assert!(html.contains("<img src=\"https://example.com/a.png\">"));
    }

    #[test]
    fn render_url_link() {
        let node = Anchor::UrlLink {
            name: "Docs".into(),
            url: "https://example.com".into(),
        };
        let html = render_node(&node, &default_config(), None);
        assert!(html.contains("href=\"https://example.com\""));
        assert!(html.contains(">Docs<"));
    }

    #[test]
    fn render_lore_link() {
        let node = Anchor::LoreLink {
            name: "Home".into(),
            path: "index".into(),
        };
        let html = render_node(&node, &default_config(), None);
        assert!(html.contains("href="));
        assert!(html.contains(">Home<"));
    }

    #[test]
    fn render_inner_url_open() {
        let node = Anchor::InnerUrlOpen {
            title: "notes".into(),
            url: "https://example.com".into(),
        };
        let html = render_node(&node, &default_config(), None);
        assert!(html.contains("foldable expanded"));
        assert!(html.contains("data-url=\"https://example.com\""));
        assert!(html.contains("data-title=\"notes\""));
    }

    #[test]
    fn render_inner_url_close_empty_title() {
        let node = Anchor::InnerUrlClose {
            title: "".into(),
            url: "https://example.com".into(),
        };
        let html = render_node(&node, &default_config(), None);
        assert_eq!(html, "");
    }

    #[test]
    fn render_inner_lore_open() {
        let node = Anchor::InnerLoreOpen {
            title: "notes".into(),
            path: "some/path".into(),
        };
        let html = render_node(&node, &default_config(), None);
        assert!(html.contains("foldable lore expanded"));
        assert!(html.contains("data-title=\"notes\""));
    }

    // ── resolve_path ─────────────────────────────────────────────

    #[test]
    fn resolve_path_no_base() {
        let cfg = default_config();
        assert_eq!(resolve_path("foo/bar.html", &cfg), "foo/bar.html");
    }

    #[test]
    fn resolve_path_with_base() {
        let mut cfg = default_config();
        cfg.link_base = "https://example.com".into();
        assert_eq!(
            resolve_path("foo/bar.html", &cfg),
            "https://example.com/foo/bar.html"
        );
    }

    #[test]
    fn resolve_path_absolute_url_unchanged() {
        let mut cfg = default_config();
        cfg.link_base = "https://example.com".into();
        assert_eq!(
            resolve_path("https://other.com/page", &cfg),
            "https://other.com/page"
        );
    }

    // ── resolve_link_from_source ─────────────────────────────────

    #[test]
    fn resolve_link_no_source() {
        let cfg = default_config();
        assert_eq!(resolve_link_from_source("target", None, &cfg), "target");
    }

    #[test]
    fn resolve_link_with_source_at_root() {
        let cfg = default_config();
        let result = resolve_link_from_source("target", Some("index.lore"), &cfg);
        assert_eq!(result, "target");
    }

    // ── Full HTML document structure ─────────────────────────────

    #[test]
    fn render_html_structure() {
        let doc = Category::new();
        let html = render(&doc);
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<html lang=\"en-US\">"));
        assert!(html.contains("<meta charset=\"utf-8\">"));
        assert!(html.contains("<meta name=\"viewport\""));
        assert!(html.contains("<title>LorePages</title>"));
        assert!(html.contains("</html>"));
    }

    #[test]
    fn render_html_title_from_h1() {
        let doc = Category {
            nodes: vec![Anchor::Heading {
                level: 1,
                content: "My Custom Title".into(),
                link: None,
            }],
        };
        let html = render(&doc);
        assert!(html.contains("<title>My Custom Title</title>"));
    }

    #[test]
    fn render_with_scripts() {
        let mut cfg = default_config();
        cfg.scripts = vec!["/js/app.js".into()];
        let renderer = HtmlRenderer;
        let doc = Category::new();
        let html = renderer.render(&doc, &CategoryConfig::default(), &cfg, None);
        assert!(html.contains("<script src=\"/js/app.js\"></script>"));
    }
}
