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

        for line in input.lines() {
            if let Some(content) = parse_placeholder(line, parser_config) {
                doc.push(Anchor::PlaceHolderLine { content });
            } else if let Some((level, content)) = parse_heading(line) {
                doc.push(Anchor::Heading { level, content });
            } else if parse_breakline(line, parser_config) {
                doc.push(Anchor::BreakLine);
            } else if let Some(comment_content) = parse_comment(line, parser_config) {
                doc.push(Anchor::Comment {
                    content: comment_content,
                });
            } else if let Some((name, url)) = parse_url_link(line, parser_config) {
                doc.push(Anchor::UrlLink { name, url });
            } else {
                let mut content = line.to_string();
                if !parser_config.url_link_key_escape.is_empty() {
                    content = content.replace(
                        &parser_config.url_link_key_escape,
                        &parser_config.url_link_key,
                    );
                }
                if !parser_config.comment_key_escape.is_empty() {
                    content = content.replace(
                        &parser_config.comment_key_escape,
                        &parser_config.comment_key,
                    );
                }
                if !parser_config.placeholder_key_escape.is_empty() {
                    content = content.replace(&parser_config.placeholder_key_escape, "_");
                }

                // skip truly empty lines (or lines with only whitespace)
                if content.trim().is_empty() {
                    // do not push an empty paragraph (no automatic <br>)
                } else {
                    doc.push(Anchor::Paragraph { content });
                }
            }
        }

        doc
    }
}

pub fn parse_heading(line: &str) -> Option<(u8, String)> {
    let bytes = line.as_bytes();
    let mut count = 0;

    for &b in bytes.iter().take(4) {
        if b == b'#' {
            count += 1;
        } else {
            break;
        }
    }

    if count == 0 {
        return None;
    }

    if bytes.len() > count && bytes[count] == b' ' {
        let content = line[count + 1..].to_string();
        Some((count as u8, content))
    } else {
        None
    }
}

pub fn parse_url_link(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<(String, String)> {
    let key = &parser_config.url_link_key;
    if key.is_empty() {
        return None;
    }

    // require spaces around the key: 'name <space> key <space> url'
    let sep = format!(" {} ", key);
    if let Some(pos) = line.find(&sep) {
        let left = &line[..pos];
        let right = &line[pos + sep.len()..];

        let mut name = left.trim().to_string();
        let mut url = right.trim().to_string();

        // unescape any escaped key occurrences (e.g. "\|" -> "|")
        if !parser_config.url_link_key_escape.is_empty() {
            name = name.replace(
                &parser_config.url_link_key_escape,
                &parser_config.url_link_key,
            );
            url = url.replace(
                &parser_config.url_link_key_escape,
                &parser_config.url_link_key,
            );
        }

        Some((name, url))
    } else {
        None
    }
}

pub fn parse_comment(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<String> {
    let key = &parser_config.comment_key;
    if key.is_empty() {
        return None;
    }

    // require marker at start of line followed by a space: '% ' for example
    let sep = format!("{} ", key);
    if line.starts_with(&sep) {
        let mut content = line[sep.len()..].to_string();

        // unescape any escaped comment key occurrences (e.g. "\\%" -> "%")
        if !parser_config.comment_key_escape.is_empty() {
            content = content.replace(
                &parser_config.comment_key_escape,
                &parser_config.comment_key,
            );
        }

        Some(content)
    } else {
        None
    }
}

pub fn parse_placeholder(
    line: &str,
    parser_config: &ParserConfig,
) -> Option<String> {
    let marker = "_";

    if parser_config.placeholder_key_escape.is_empty() {
        if line.starts_with(marker) {
            let content = if line.len() > marker.len() {
                line[marker.len()..].to_string()
            } else {
                String::new()
            };
            Some(content)
        } else {
            None
        }
    } else {
        let sep = format!("{}{}", parser_config.placeholder_key_escape, marker);
        if line.starts_with(&sep) {
            // escaped marker at start -> not a placeholder
            None
        } else if line.starts_with(marker) {
            let content = if line.len() > marker.len() {
                line[marker.len()..].to_string()
            } else {
                String::new()
            };
            Some(content)
        } else {
            None
        }
    }
}

pub fn parse_breakline(
    line: &str,
    parser_config: &ParserConfig,
) -> bool {
    let key = &parser_config.breakline_key;
    if key.is_empty() {
        return false;
    }

    if parser_config.breakline_key_escape.is_empty() {
        line.starts_with(key)
    } else {
        let sep = format!("{}{}", parser_config.breakline_key_escape, key);
        if line.starts_with(&sep) {
            // escaped marker at start -> not a breakline
            false
        } else {
            line.starts_with(key)
        }
    }
}
