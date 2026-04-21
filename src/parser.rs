use crate::framework::category_config::CategoryConfig;
use crate::framework::parser::Parser;
use crate::framework::parser_config::ParserConfig;
use crate::ir::anchor::Anchor;
use crate::ir::category::Category;

pub struct MarkdownParser;

impl Parser for MarkdownParser {
    fn parse(
        &self,
        input: &str,
        _category_config: &CategoryConfig,
        parser_config: &ParserConfig,
    ) -> Category {
        let mut doc = Category::new();

        let lines = input.lines();
        for line in lines {
            if let Some((level, content)) = parse_heading(line) {
                doc.push(Anchor::Heading { level, content });
                continue;
            }

            if let Some(comment_content) = parse_comment(line, parser_config) {
                doc.push(Anchor::Comment { content: comment_content });
                continue;
            }

            if let Some(url) = parse_image(line, parser_config) {
                doc.push(Anchor::Image { url });
                continue;
            }

            if let Some(content) = parse_placeholder(line, parser_config) {
                doc.push(Anchor::PlaceHolderLine { content });
                continue;
            }

            if parse_breakline(line, parser_config) {
                doc.push(Anchor::BreakLine);
                continue;
            }

            if let Some((title, url)) = parse_inner_url_open(line, parser_config) {
                doc.push(Anchor::InnerUrlOpen { title, url });
                continue;
            }

            if let Some((title, url)) = parse_inner_url_close(line, parser_config) {
                doc.push(Anchor::InnerUrlClose { title, url });
                continue;
            }

            if let Some((title, path)) = parse_inner_lore_open(line, parser_config) {
                doc.push(Anchor::InnerLoreOpen { title, path });
                continue;
            }

            if let Some((title, path)) = parse_inner_lore_close(line, parser_config) {
                doc.push(Anchor::InnerLoreClose { title, path });
                continue;
            }

            if let Some((name, url)) = parse_url_link(line, parser_config) {
                doc.push(Anchor::UrlLink { name, url });
                continue;
            }

            if let Some((name, path)) = parse_lore_link(line, parser_config) {
                doc.push(Anchor::LoreLink { name, path });
                continue;
            }

            let mut content = line.to_string();
            // Unescape any escaped markers so paragraphs
            // display literal markers
            content = unescape(&content, &parser_config.url_link_key_escape, &parser_config.url_link_key);
            content = unescape(&content, &parser_config.lore_link_key_escape, &parser_config.lore_link_key);
            content = unescape(&content, &parser_config.comment_key_escape, &parser_config.comment_key);
            content = unescape(&content, &parser_config.placeholder_key_escape, &parser_config.placeholder_key);
            content = unescape(&content, &parser_config.breakline_key_escape, &parser_config.breakline_key);
            content = unescape(&content, &parser_config.image_key_escape, &parser_config.image_key);
            content = unescape(&content, &parser_config.inner_close_key_escape, &parser_config.inner_close_key);
            content = unescape(&content, &parser_config.inner_open_key_escape, &parser_config.inner_open_key);
            content = unescape(&content, &parser_config.inner_lore_key_escape, &parser_config.inner_lore_key);
            content = unescape(&content, &parser_config.inner_url_key_escape, &parser_config.inner_url_key);

            // skip truly empty lines (or lines with only whitespace)
            if content.trim().is_empty() {
                doc.push(Anchor::EmptyLine);
            } else {
                doc.push(Anchor::Paragraph { content });
            }
        }

        doc
    }
}

/// Parse repeated single-character marker prefixes (e.g. `#`,
///  up to `max` times).
/// If `require_space` is true, require a space after the
/// repeated markers.
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

pub fn parse_heading(line: &str) -> Option<(u8, String)> {
    parse_repeated_prefix(line, '#', 4, true)
}

/// Replace escaped marker sequences with the actual marker.
/// If `escape` is empty this returns `s` as an
/// owned `String` unchanged.
fn unescape(
    s: &str,
    escape: &str,
    replacement: &str,
) -> String {
    if escape.is_empty() {
        s.to_string()
    } else {
        s.replace(escape, replacement)
    }
}

pub fn parse_binary(
    line: &str,
    key: &str,
    key_escape: &str,
) -> Option<(String, String)> {
    if key.is_empty() {
        return None;
    }

    // require spaces around the key: 'name <space>
    // key <space> value'
    let sep = format!(" {} ", key);
    if let Some(pos) = line.find(&sep) {
        let left = &line[..pos];
        let right = &line[pos + sep.len()..];

        // Trim and unescape occurrences of the escaped key (if configured)
        let name = unescape(left.trim(), key_escape, key);
        let value = unescape(right.trim(), key_escape, key);

        Some((name, value))
    } else {
        None
    }
}

pub fn parse_url_link(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<(String, String)> {
    // kept for compatibility but replaced by `parse_binary`
    parse_binary(line, &parser_config.url_link_key, &parser_config.url_link_key_escape)
}

pub fn parse_lore_link(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<(String, String)> {
    // symmetric helper for lore links
    parse_binary(line, &parser_config.lore_link_key, &parser_config.lore_link_key_escape)
}

pub fn parse_prefix(
    line: &str,
    key: &str,
    key_escape: &str,
    require_space: bool,
) -> Option<String> {
    if key.is_empty() {
        return None;
    }

    // If an escape prefix is configured and the line
    // starts with it followed by the key,
    // treat it as escaped and do not match.
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

pub fn parse_comment(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<String> {
    parse_prefix(line, &parser_config.comment_key, &parser_config.comment_key_escape, true)
        .map(|content| unescape(&content, &parser_config.comment_key_escape, &parser_config.comment_key))
}

pub fn parse_image(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<String> {
    parse_prefix(line, &parser_config.image_key, &parser_config.image_key_escape, true)
        .map(|content| unescape(&content, &parser_config.image_key_escape, &parser_config.image_key))
}

pub fn parse_placeholder(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<String> {
    parse_prefix(line, &parser_config.placeholder_key, &parser_config.placeholder_key_escape, false)
}

pub fn parse_breakline(
    line: &str,
    parser_config: &ParserConfig,
) -> bool {
    parse_prefix(line, &parser_config.breakline_key, &parser_config.breakline_key_escape, false).is_some()
}

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

    let sep = format!("{} ", key_front);
    if line.starts_with(&sep) {
        let sep = format!(" {} ", &key[2..]); // TODO: can be better
        if let Some(pos) = line.find(&sep) {
            let left = &line[..pos];
            let right = &line[pos + sep.len()..];

            let name = unescape(left.trim(), key_escape, key);
            let value = unescape(right.trim(), key_escape, key);

            Some((name, value))
        } else {
            None
        }
    } else {
        None
    }
}

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
