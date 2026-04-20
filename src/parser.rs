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
            if let Some((level, content)) = parse_heading(line) {
                doc.push(Anchor::Heading { level, content });
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

                doc.push(Anchor::Paragraph { content });
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
