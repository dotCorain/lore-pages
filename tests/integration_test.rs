//! Integration tests for the full parse → render pipeline.

use lore_pages::framework::category_config::CategoryConfig;
use lore_pages::framework::converter::CategoryConverter;
use lore_pages::framework::parser_config::ParserConfig;
use lore_pages::framework::renderer_config::RenderConfig;
use lore_pages::parser::LorePagesParser;
use lore_pages::render::HtmlRenderer;

fn make_converter() -> CategoryConverter<'static, LorePagesParser, HtmlRenderer> {
    // Leak the configs so they have a 'static lifetime.
    // This is safe because the tests never run long enough for the
    // leaked memory to matter.
    let cat_cfg: &'static CategoryConfig = Box::leak(Box::new(CategoryConfig::default()));
    let rend_cfg: &'static RenderConfig = Box::leak(Box::new(RenderConfig::default()));
    let pars_cfg: &'static ParserConfig = Box::leak(Box::new(ParserConfig::default()));

    CategoryConverter::from_config(LorePagesParser, HtmlRenderer, cat_cfg, rend_cfg, pars_cfg)
}

#[test]
fn convert_simple_page() {
    let converter = make_converter();
    let input = "# Hello\n\nWorld\n\nDocs | https://example.com\n";
    let html = converter.convert_simple(input);

    assert!(html.contains("<h1>Hello</h1>"));
    assert!(html.contains("<p>World</p>"));
    assert!(html.contains("https://example.com"));
}

#[test]
fn convert_headings_all_levels() {
    let converter = make_converter();
    let input = "# H1\n## H2\n### H3\n#### H4\n";
    let html = converter.convert_simple(input);

    assert!(html.contains("<h1>H1</h1>"));
    // H2 gets auto-linked, so it's wrapped in <a> inside <h2>
    assert!(html.contains("<h2><a"));
    assert!(html.contains(">H2</a></h2>"));
    assert!(html.contains("<h3>H3</h3>"));
    assert!(html.contains("<h4>H4</h4>"));
}

#[test]
fn convert_lore_link() {
    let converter = make_converter();
    let input = "Home = index\n";
    let html = converter.convert_simple(input);

    assert!(html.contains(">Home<"));
    assert!(html.contains("href="));
    assert!(html.contains("index"));
}

#[test]
fn convert_comment_becomes_html_comment() {
    let converter = make_converter();
    let input = "% secret note\nvisible\n";
    let html = converter.convert_simple(input);

    assert!(html.contains("<!-- secret note -->"));
    assert!(html.contains("<p>visible</p>"));
}

#[test]
fn convert_image() {
    let converter = make_converter();
    let input = "| https://img.example.com/pic.png\n";
    let html = converter.convert_simple(input);

    assert!(html.contains("<img src=\"https://img.example.com/pic.png\">"));
}

#[test]
fn convert_breakline() {
    let converter = make_converter();
    let input = "---\n";
    let html = converter.convert_simple(input);

    assert!(html.contains("<br>"));
}

#[test]
fn convert_empty_and_placeholder_not_rendered() {
    let converter = make_converter();
    let input = "\n_ placeholder\n";
    let html = converter.convert_simple(input);

    // Should not contain visible output for empty lines or placeholders
    let body_start = html.find("<body>").unwrap();
    let body_end = html.find("</body>").unwrap();
    let body = &html[body_start..body_end];

    // After <body>\n, we should only have newlines (from EmptyLine and PlaceHolderLine)
    let after_body_tag = &body["<body>\n".len()..];
    // There should be no <p> or other visible content
    assert!(!after_body_tag.contains("<p>"));
    assert!(!after_body_tag.contains("<br>"));
}

#[test]
fn convert_inner_url_open() {
    let converter = make_converter();
    let input = "- notes > https://example.com\n";
    let html = converter.convert_simple(input);

    assert!(html.contains("foldable expanded"));
    assert!(html.contains("data-url=\"https://example.com\""));
}

#[test]
fn convert_inner_lore_open() {
    let converter = make_converter();
    let input = "- notes = some/path\n";
    let html = converter.convert_simple(input);

    assert!(html.contains("foldable lore expanded"));
    assert!(html.contains("some/path"));
}

#[test]
fn convert_with_source_path() {
    let cat_cfg = CategoryConfig::default();
    let rend_cfg = RenderConfig::default();
    let pars_cfg = ParserConfig::default();
    let converter =
        CategoryConverter::from_config(LorePagesParser, HtmlRenderer, &cat_cfg, &rend_cfg, &pars_cfg);

    let input = "# Page\n";
    let html = converter.convert_with_source(input, "dir/subdir/index.lore");

    assert!(html.contains("<h1>Page</h1>"));
}

#[test]
fn convert_page_with_title() {
    let converter = make_converter();
    let input = "# My Page Title\n\nContent here.\n";
    let html = converter.convert_simple(input);

    assert!(html.contains("<title>My Page Title</title>"));
}
