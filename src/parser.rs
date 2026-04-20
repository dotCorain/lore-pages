use crate::framework::parser::Parser;
use crate::ir::anchor::Anchor;
use crate::ir::category::Category;

pub struct MarkdownParser;

impl Parser for MarkdownParser {
    fn parse(&self, input: &str) -> Category {
        let mut doc = Category::new();

        for line in input.lines() {
            if let Some((level, content)) = parse_heading(line) {
                doc.push(Anchor::Heading { level, content });
            } else {
                doc.push(Anchor::Paragraph {
                    content: line.to_string(),
                });
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
