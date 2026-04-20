#[derive(Debug, Clone, PartialEq)]
pub enum Anchor {
    Heading {
        level: u8, // from 1 to 4
        content: String,
    },
    Paragraph {
        content: String,
    },
    UrlLink {
        name: String,
        url: String,
    },
}
