use crate::framework::category_config::CategoryConfig;
use crate::framework::parser::Parser;
use crate::framework::parser_config::ParserConfig;
use crate::ir::anchor::Anchor;
use crate::ir::category::Category;

/// Default parser that converts Lore markup source text into a [`Category`] document.
///
/// Supports headings (`#`–`####`), paragraphs, links, images, comments,
/// breaklines, placeholders, and foldable blocks (inner open/close with
/// URL or Lore targets).
pub struct LorePagesParser;

impl Parser for LorePagesParser {
    fn parse(
        &self,
        input: &str,
        _category_config: &CategoryConfig,
        parser_config: &ParserConfig,
    ) -> Category {
        let mut doc = Category::new();

        for line in input.lines() {
            // Try each registered syntax pattern in priority order.
            if let Some(anchor) = try_match_line(line, parser_config) {
                doc.push(anchor);
                continue;
            }

            // Fallback: unescape the line and emit a paragraph or empty line.
            let content = unescape_all(line, parser_config);
            if content.trim().is_empty() {
                doc.push(Anchor::EmptyLine);
            } else {
                doc.push(Anchor::Paragraph { content });
            }
        }

        doc
    }
}

// ── Matcher registry ──────────────────────────────────────────────
//
// Adding a new syntax element requires only three steps:
// 1. Add a variant to the `Anchor` enum in `ir/anchor.rs`.
// 2. Write a `match_*` adapter function (see below for examples).
// 3. Register it in the `matchers()` list.
//
// The existing `pub parse_*` functions remain unchanged and can be reused
// directly inside new adapters.

/// Signature of a line matcher: receives the raw line and parser config,
/// returns an `Anchor` node if the line matches the syntax.
type LineMatcher = fn(&str, &ParserConfig) -> Option<Anchor>;

/// Ordered list of all syntax matchers. Earlier entries take priority.
fn matchers() -> &'static [LineMatcher] {
    &[
        match_heading,
        match_comment,
        match_image,
        match_placeholder,
        match_breakline,
        match_inner_url_open,
        match_inner_url_close,
        match_inner_lore_open,
        match_inner_lore_close,
        match_url_link,
        match_lore_link,
    ]
}

/// Try each matcher in order; return the first successful match.
fn try_match_line(line: &str, config: &ParserConfig) -> Option<Anchor> {
    for matcher in matchers() {
        if let Some(anchor) = matcher(line, config) {
            return Some(anchor);
        }
    }
    None
}

// ── Matcher adapters ──────────────────────────────────────────────
// Each wraps a public `parse_*` helper and maps the result to an Anchor.

fn match_heading(line: &str, _: &ParserConfig) -> Option<Anchor> {
    parse_heading(line).map(|(level, content)| Anchor::Heading { level, content, link: None })
}

fn match_comment(line: &str, config: &ParserConfig) -> Option<Anchor> {
    parse_comment(line, config).map(|content| Anchor::Comment { content })
}

fn match_image(line: &str, config: &ParserConfig) -> Option<Anchor> {
    parse_image(line, config).map(|url| Anchor::Image { url })
}

fn match_placeholder(line: &str, config: &ParserConfig) -> Option<Anchor> {
    parse_placeholder(line, config).map(|content| Anchor::PlaceHolderLine { content })
}

fn match_breakline(line: &str, config: &ParserConfig) -> Option<Anchor> {
    parse_breakline(line, config).then_some(Anchor::BreakLine)
}

fn match_url_link(line: &str, config: &ParserConfig) -> Option<Anchor> {
    parse_url_link(line, config).map(|(name, url)| Anchor::UrlLink { name, url })
}

fn match_lore_link(line: &str, config: &ParserConfig) -> Option<Anchor> {
    parse_lore_link(line, config).map(|(name, path)| Anchor::LoreLink { name, path })
}

fn match_inner_url_open(line: &str, config: &ParserConfig) -> Option<Anchor> {
    parse_inner_url_open(line, config).map(|(title, url)| Anchor::InnerUrlOpen { title, url })
}

fn match_inner_url_close(line: &str, config: &ParserConfig) -> Option<Anchor> {
    parse_inner_url_close(line, config).map(|(title, url)| Anchor::InnerUrlClose { title, url })
}

fn match_inner_lore_open(line: &str, config: &ParserConfig) -> Option<Anchor> {
    parse_inner_lore_open(line, config)
        .map(|(title, path)| Anchor::InnerLoreOpen { title, path })
}

fn match_inner_lore_close(line: &str, config: &ParserConfig) -> Option<Anchor> {
    parse_inner_lore_close(line, config)
        .map(|(title, path)| Anchor::InnerLoreClose { title, path })
}

// ── Unescape helpers ─────────────────────────────────────────────

/// Replace all configured `escape → literal` pairs in a single logical pass.
/// This is used for paragraph text that may contain escaped markup characters
/// (e.g. `\|` → `|` so the literal pipe is displayed instead of being parsed
/// as a link delimiter).
fn unescape_all(s: &str, config: &ParserConfig) -> String {
    let pairs: &[(&str, &str)] = &[
        (&config.url_link_key_escape, &config.url_link_key),
        (&config.lore_link_key_escape, &config.lore_link_key),
        (&config.comment_key_escape, &config.comment_key),
        (&config.placeholder_key_escape, &config.placeholder_key),
        (&config.breakline_key_escape, &config.breakline_key),
        (&config.image_key_escape, &config.image_key),
        (&config.inner_close_key_escape, &config.inner_close_key),
        (&config.inner_open_key_escape, &config.inner_open_key),
        (&config.inner_lore_key_escape, &config.inner_lore_key),
        (&config.inner_url_key_escape, &config.inner_url_key),
    ];

    let mut result = s.to_string();
    for (escape, replacement) in pairs {
        // Only perform the replacement when the escape pattern is present,
        // avoiding unnecessary allocations for the common case.
        if !escape.is_empty() && result.contains(escape) {
            result = result.replace(escape, replacement);
        }
    }
    result
}

/// Replace a single `escape` sequence with its `replacement` literal.
/// If `escape` is empty the input is returned as a clone.
fn unescape(s: &str, escape: &str, replacement: &str) -> String {
    if escape.is_empty() {
        s.to_string()
    } else {
        s.replace(escape, replacement)
    }
}

// ── Public parsing primitives ────────────────────────────────────
//
// These functions are the building blocks for parsing individual syntax
// elements. They are kept `pub` for users who want to compose custom
// parsers from the same primitives.

/// Parse repeated single-character marker prefixes (e.g. `#`, `##`, …)
/// up to `max` times.
///
/// If `require_space` is true, a space must follow the repeated markers.
pub fn parse_repeated_prefix(
    line: &str,
    marker: char,
    max: usize,
    require_space: bool,
) -> Option<(u8, String)> {
    let bytes = line.as_bytes();
    let mut count = 0usize;

    for &b in bytes.iter().take(max) {
        if b == marker as u8 {
            count += 1;
        } else {
            break;
        }
    }

    if count == 0 {
        return None;
    }

    match (require_space, bytes.get(count)) {
        (true, Some(&b' ')) => Some((count as u8, line[count + 1..].to_string())),
        (true, _) => None,
        (false, _) => Some((count as u8, line.get(count..).unwrap_or("").to_string())),
    }
}

/// Parse a Markdown-style heading: `#` through `####` followed by a space.
pub fn parse_heading(line: &str) -> Option<(u8, String)> {
    parse_repeated_prefix(line, '#', 4, true)
}

/// Parse a binary infix syntax: `left KEY right` (with spaces around `key`).
///
/// Returns `Some((left, right))` with both sides trimmed and unescaped,
/// or `None` if the key is not found or is empty.
pub fn parse_binary(line: &str, key: &str, key_escape: &str) -> Option<(String, String)> {
    if key.is_empty() {
        return None;
    }

    let sep = format!(" {} ", key);
    if let Some(pos) = line.find(&sep) {
        let left = &line[..pos];
        let right = &line[pos + sep.len()..];

        let name = unescape(left.trim(), key_escape, key);
        let value = unescape(right.trim(), key_escape, key);

        Some((name, value))
    } else {
        None
    }
}

/// Parse an external URL link: `name | url` (with default config).
pub fn parse_url_link(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<(String, String)> {
    parse_binary(line, &parser_config.url_link_key, &parser_config.url_link_key_escape)
}

/// Parse an internal Lore link: `name = path` (with default config).
pub fn parse_lore_link(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<(String, String)> {
    parse_binary(line, &parser_config.lore_link_key, &parser_config.lore_link_key_escape)
}

/// Parse a prefix-based syntax: a line starting with `key`, optionally
/// followed by a space, returning the remainder.
///
/// If the line starts with the escape prefix (`key_escape + key`), it is
/// treated as escaped and `None` is returned.
pub fn parse_prefix(
    line: &str,
    key: &str,
    key_escape: &str,
    require_space: bool,
) -> Option<String> {
    if key.is_empty() {
        return None;
    }

    if !key_escape.is_empty() {
        let esc_prefix = format!("{}{}", key_escape, key);
        if line.starts_with(&esc_prefix) {
            return None;
        }
    }

    match require_space {
        true => {
            let sep = format!("{} ", key);
            if line.starts_with(&sep) {
                Some(line[sep.len()..].to_string())
            } else {
                None
            }
        }
        false => line.strip_prefix(key).map(|stripped| stripped.to_string()),
    }
}

/// Parse a comment line: `% content`.
pub fn parse_comment(line: &str, parser_config: &ParserConfig) -> Option<String> {
    parse_prefix(
        line,
        &parser_config.comment_key,
        &parser_config.comment_key_escape,
        true,
    )
    .map(|content| unescape(&content, &parser_config.comment_key_escape, &parser_config.comment_key))
}

/// Parse an image line: `| url`.
pub fn parse_image(line: &str, parser_config: &ParserConfig) -> Option<String> {
    parse_prefix(
        line,
        &parser_config.image_key,
        &parser_config.image_key_escape,
        true,
    )
    .map(|content| unescape(&content, &parser_config.image_key_escape, &parser_config.image_key))
}

/// Parse a placeholder line: `_content`. No space required after `_`.
pub fn parse_placeholder(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<String> {
    parse_prefix(
        line,
        &parser_config.placeholder_key,
        &parser_config.placeholder_key_escape,
        false,
    )
}

/// Returns `true` if the line is a breakline (default: `---`).
pub fn parse_breakline(line: &str, parser_config: &ParserConfig) -> bool {
    parse_prefix(
        line,
        &parser_config.breakline_key,
        &parser_config.breakline_key_escape,
        false,
    )
    .is_some()
}

/// Parse a "domain" syntax: `FRONT title KEY value`.
///
/// Used for foldable block open/close markers. The line must start with
/// `key_front` followed by a space, then a title, then `key` surrounded
/// by spaces, then a value.
pub fn parse_domain(
    line: &str,
    key_front: &str,
    key_front_escape: &str,
    key: &str,
    key_escape: &str,
) -> Option<(String, String)> {
    if key_front.is_empty() {
        return None;
    }

    if !key_front_escape.is_empty() {
        let esc_prefix = format!("{}{}", key_front_escape, key_front);
        if line.starts_with(&esc_prefix) {
            return None;
        }
    }

    if key.is_empty() {
        return None;
    }

    if !key_escape.is_empty() {
        let esc_prefix = format!("{}{}", key_escape, key);
        if line.starts_with(&esc_prefix) {
            return None;
        }
    }

    let prefix = format!("{} ", key_front);
    if !line.starts_with(&prefix) {
        return None;
    }

    let after_prefix = &line[prefix.len()..];
    let sep = format!(" {} ", key);

    if let Some(pos) = after_prefix.find(&sep) {
        let title = after_prefix[..pos].trim();
        let value = after_prefix[pos + sep.len()..].trim();

        if !title.is_empty() && !value.is_empty() {
            let title = unescape(title, key_escape, key);
            let value = unescape(value, key_escape, key);
            return Some((title, value));
        }
    }
    None
}

/// Parse an inner URL open marker: `- title KEY url`.
pub fn parse_inner_url_open(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<(String, String)> {
    parse_domain(
        line,
        &parser_config.inner_open_key,
        &parser_config.inner_open_key_escape,
        &parser_config.inner_url_key,
        &parser_config.inner_url_key_escape,
    )
}

/// Parse an inner URL close marker: `+ title KEY url`.
pub fn parse_inner_url_close(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<(String, String)> {
    parse_domain(
        line,
        &parser_config.inner_close_key,
        &parser_config.inner_close_key_escape,
        &parser_config.inner_url_key,
        &parser_config.inner_url_key_escape,
    )
}

/// Parse an inner Lore open marker: `- title KEY path`.
pub fn parse_inner_lore_open(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<(String, String)> {
    parse_domain(
        line,
        &parser_config.inner_open_key,
        &parser_config.inner_open_key_escape,
        &parser_config.inner_lore_key,
        &parser_config.inner_lore_key_escape,
    )
}

/// Parse an inner Lore close marker: `+ title KEY path`.
pub fn parse_inner_lore_close(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<(String, String)> {
    parse_domain(
        line,
        &parser_config.inner_close_key,
        &parser_config.inner_close_key_escape,
        &parser_config.inner_lore_key,
        &parser_config.inner_lore_key_escape,
    )
}

// ── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> ParserConfig {
        ParserConfig::default()
    }

    // ── parse_heading ────────────────────────────────────────────

    #[test]
    fn heading_h1() {
        assert_eq!(parse_heading("# Title"), Some((1, "Title".into())));
    }

    #[test]
    fn heading_h2() {
        assert_eq!(parse_heading("## Section"), Some((2, "Section".into())));
    }

    #[test]
    fn heading_h3() {
        assert_eq!(parse_heading("### Sub"), Some((3, "Sub".into())));
    }

    #[test]
    fn heading_h4() {
        assert_eq!(parse_heading("#### Deep"), Some((4, "Deep".into())));
    }

    #[test]
    fn heading_too_many_hashes() {
        assert_eq!(parse_heading("##### Five"), None);
    }

    #[test]
    fn heading_no_space_after_hash() {
        assert_eq!(parse_heading("#NoSpace"), None);
    }

    #[test]
    fn heading_not_a_heading() {
        assert_eq!(parse_heading("Just text"), None);
    }

    #[test]
    fn heading_with_trailing_spaces() {
        assert_eq!(parse_heading("##  Extra space"), Some((2, " Extra space".into())));
    }

    // ── parse_comment ────────────────────────────────────────────

    #[test]
    fn comment_basic() {
        let cfg = default_config();
        assert_eq!(parse_comment("% a note", &cfg), Some("a note".into()));
    }

    #[test]
    fn comment_no_space_after_percent() {
        let cfg = default_config();
        assert_eq!(parse_comment("%note", &cfg), None);
    }

    #[test]
    fn comment_not_a_comment() {
        let cfg = default_config();
        assert_eq!(parse_comment("not a % comment", &cfg), None);
    }

    #[test]
    fn comment_escaped_percent() {
        let mut cfg = default_config();
        cfg.comment_key = "%".into();
        cfg.comment_key_escape = "\\%".into();
        // The escape prefix should prevent matching
        assert_eq!(parse_comment("\\% escaped", &cfg), None);
    }

    // ── parse_image ──────────────────────────────────────────────

    #[test]
    fn image_basic() {
        let cfg = default_config();
        assert_eq!(
            parse_image("| https://example.com/img.png", &cfg),
            Some("https://example.com/img.png".into())
        );
    }

    #[test]
    fn image_no_space() {
        let cfg = default_config();
        assert_eq!(parse_image("|https://x.com/a.png", &cfg), None);
    }

    // ── parse_placeholder ────────────────────────────────────────

    #[test]
    fn placeholder_basic() {
        let cfg = default_config();
        assert_eq!(parse_placeholder("_ content", &cfg), Some(" content".into()));
    }

    #[test]
    fn placeholder_no_space() {
        let cfg = default_config();
        assert_eq!(parse_placeholder("_content", &cfg), Some("content".into()));
    }

    // ── parse_breakline ──────────────────────────────────────────

    #[test]
    fn breakline_basic() {
        let cfg = default_config();
        assert!(parse_breakline("---", &cfg));
    }

    #[test]
    fn breakline_with_extra() {
        let cfg = default_config();
        assert!(parse_breakline("--- more", &cfg));
    }

    #[test]
    fn breakline_not_enough_dashes() {
        let cfg = default_config();
        assert!(!parse_breakline("--", &cfg));
    }

    #[test]
    fn breakline_not_breakline() {
        let cfg = default_config();
        assert!(!parse_breakline("not a breakline", &cfg));
    }

    // ── parse_url_link ───────────────────────────────────────────

    #[test]
    fn url_link_basic() {
        let cfg = default_config();
        assert_eq!(
            parse_url_link("Docs | https://example.com", &cfg),
            Some(("Docs".into(), "https://example.com".into()))
        );
    }

    #[test]
    fn url_link_no_spaces_around_pipe() {
        let cfg = default_config();
        assert_eq!(parse_url_link("Docs|url", &cfg), None);
    }

    // ── parse_lore_link ──────────────────────────────────────────

    #[test]
    fn lore_link_basic() {
        let cfg = default_config();
        assert_eq!(
            parse_lore_link("Home = index", &cfg),
            Some(("Home".into(), "index".into()))
        );
    }

    #[test]
    fn lore_link_no_spaces() {
        let cfg = default_config();
        assert_eq!(parse_lore_link("Home=index", &cfg), None);
    }

    // ── parse_inner_* ────────────────────────────────────────────

    #[test]
    fn inner_url_open_basic() {
        let cfg = default_config();
        assert_eq!(
            parse_inner_url_open("- notes > https://example.com", &cfg),
            Some(("notes".into(), "https://example.com".into()))
        );
    }

    #[test]
    fn inner_url_close_basic() {
        let cfg = default_config();
        assert_eq!(
            parse_inner_url_close("+ notes > https://example.com", &cfg),
            Some(("notes".into(), "https://example.com".into()))
        );
    }

    #[test]
    fn inner_lore_open_basic() {
        let cfg = default_config();
        assert_eq!(
            parse_inner_lore_open("- notes = some/path", &cfg),
            Some(("notes".into(), "some/path".into()))
        );
    }

    #[test]
    fn inner_lore_close_basic() {
        let cfg = default_config();
        assert_eq!(
            parse_inner_lore_close("+ notes = some/path", &cfg),
            Some(("notes".into(), "some/path".into()))
        );
    }

    // ── parse_domain ─────────────────────────────────────────────

    #[test]
    fn domain_empty_key_front() {
        assert_eq!(parse_domain("any", "", "", ">", ""), None);
    }

    #[test]
    fn domain_empty_key() {
        assert_eq!(parse_domain("any", "-", "", "", ""), None);
    }

    #[test]
    fn domain_escaped_front() {
        assert_eq!(
            parse_domain("\\- title > url", "-", "\\-", ">", ""),
            None
        );
    }

    // ── parse_binary ─────────────────────────────────────────────

    #[test]
    fn binary_empty_key() {
        assert_eq!(parse_binary("a = b", "", ""), None);
    }

    #[test]
    fn binary_not_found() {
        assert_eq!(parse_binary("no equals here", "=", "\\="), None);
    }

    #[test]
    fn binary_basic() {
        assert_eq!(
            parse_binary("left = right", "=", "\\="),
            Some(("left".into(), "right".into()))
        );
    }

    // ── parse_prefix ─────────────────────────────────────────────

    #[test]
    fn prefix_empty_key() {
        assert_eq!(parse_prefix("any", "", "", true), None);
    }

    #[test]
    fn prefix_with_space() {
        assert_eq!(
            parse_prefix("% comment", "%", "\\%", true),
            Some("comment".into())
        );
    }

    #[test]
    fn prefix_no_space_required() {
        assert_eq!(parse_prefix("___", "---", "", false), None);
    }

    // ── parse_repeated_prefix ────────────────────────────────────

    #[test]
    fn repeated_prefix_single() {
        assert_eq!(
            parse_repeated_prefix("# h", '#', 4, true),
            Some((1, "h".into()))
        );
    }

    #[test]
    fn repeated_prefix_max() {
        assert_eq!(
            parse_repeated_prefix("#### h", '#', 4, true),
            Some((4, "h".into()))
        );
    }

    #[test]
    fn repeated_prefix_exceeds_max() {
        // max is 4, so only 4 are counted, but the 5th char is '#' not ' ',
        // so require_space fails
        assert_eq!(parse_repeated_prefix("##### h", '#', 4, true), None);
    }

    #[test]
    fn repeated_prefix_none() {
        assert_eq!(parse_repeated_prefix("no hash", '#', 4, true), None);
    }

    // ── Full parse (LorePagesParser) ─────────────────────────────

    fn parse(input: &str) -> Category {
        let parser = LorePagesParser;
        parser.parse(input, &CategoryConfig::default(), &default_config())
    }

    #[test]
    fn full_parse_headings_and_paragraphs() {
        let doc = parse("# Title\n\n## Section\n\nSome text.\n");
        assert_eq!(doc.nodes.len(), 5);
        assert!(matches!(doc.nodes[0], Anchor::Heading { level: 1, .. }));
        assert!(matches!(doc.nodes[1], Anchor::EmptyLine));
        assert!(matches!(doc.nodes[2], Anchor::Heading { level: 2, .. }));
        assert!(matches!(doc.nodes[3], Anchor::EmptyLine));
        assert!(matches!(doc.nodes[4], Anchor::Paragraph { .. }));
    }

    #[test]
    fn full_parse_url_link() {
        let doc = parse("Docs | https://example.com\n");
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::UrlLink { .. }));
    }

    #[test]
    fn full_parse_lore_link() {
        let doc = parse("Home = index\n");
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::LoreLink { .. }));
    }

    #[test]
    fn full_parse_comment_is_not_paragraph() {
        let doc = parse("% this is a comment\n");
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::Comment { .. }));
    }

    #[test]
    fn full_parse_image() {
        let doc = parse("| https://img.example.com/pic.png\n");
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::Image { .. }));
    }

    #[test]
    fn full_parse_breakline() {
        let doc = parse("---\n");
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::BreakLine));
    }

    #[test]
    fn full_parse_placeholder() {
        let doc = parse("_ some placeholder\n");
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::PlaceHolderLine { .. }));
    }

    #[test]
    fn full_parse_empty_line() {
        let doc = parse("\n");
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::EmptyLine));
    }

    #[test]
    fn full_parse_whitespace_only_line() {
        let doc = parse("   \n");
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::EmptyLine));
    }

    #[test]
    fn paragraph_with_escaped_marker() {
        let doc = parse("This is \\| not a link\n");
        assert_eq!(doc.nodes.len(), 1);
        if let Anchor::Paragraph { content } = &doc.nodes[0] {
            assert_eq!(content, "This is | not a link");
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn paragraph_escaped_lore_link() {
        let doc = parse("Not a \\= link\n");
        assert_eq!(doc.nodes.len(), 1);
        if let Anchor::Paragraph { content } = &doc.nodes[0] {
            assert_eq!(content, "Not a = link");
        } else {
            panic!("Expected paragraph");
        }
    }

    #[test]
    fn custom_url_link_key() {
        let cfg = ParserConfig {
            url_link_key: "->".into(),
            url_link_key_escape: "\\->".into(),
            ..ParserConfig::default()
        };

        let parser = LorePagesParser;
        let doc = parser.parse("Docs -> https://example.com\n", &CategoryConfig::default(), &cfg);
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::UrlLink { .. }));
    }

    #[test]
    fn custom_comment_key() {
        let cfg = ParserConfig {
            comment_key: "//".into(),
            comment_key_escape: "\\//".into(),
            ..ParserConfig::default()
        };

        let parser = LorePagesParser;
        let doc = parser.parse("// a comment\n", &CategoryConfig::default(), &cfg);
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::Comment { .. }));
    }

    #[test]
    fn inner_url_open_full_parse() {
        let doc = parse("- notes > https://example.com\n");
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::InnerUrlOpen { .. }));
    }

    #[test]
    fn inner_lore_open_full_parse() {
        let doc = parse("- notes = some/path\n");
        assert_eq!(doc.nodes.len(), 1);
        assert!(matches!(doc.nodes[0], Anchor::InnerLoreOpen { .. }));
    }

    #[test]
    fn multiple_paragraphs() {
        let doc = parse("First paragraph.\n\nSecond paragraph.\n");
        assert_eq!(doc.nodes.len(), 3);
        assert!(matches!(doc.nodes[0], Anchor::Paragraph { .. }));
        assert!(matches!(doc.nodes[1], Anchor::EmptyLine));
        assert!(matches!(doc.nodes[2], Anchor::Paragraph { .. }));
    }

    #[test]
    fn mixed_document() {
        let input = concat!(
            "# My Page\n",
            "\n",
            "## Intro\n",
            "\n",
            "Welcome to my page.\n",
            "\n",
            "GitHub | https://github.com\n",
            "\n",
            "## Links\n",
            "\n",
            "Home = index\n",
            "\n",
            "% footer comment\n",
        );
        let doc = parse(input);

        // Should have: H1, Empty, H2, Empty, P, Empty, UrlLink, Empty, H2, Empty, LoreLink, Empty, Comment
        assert_eq!(doc.nodes.len(), 13);
    }
}
