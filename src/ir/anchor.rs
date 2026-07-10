#[derive(Debug, Clone, PartialEq)]
pub enum Anchor {
    Heading {
        level: u8, // from 1 to 4
        content: String,
        link: Option<String>, // auto-link path (H2 headings)
    },
    Paragraph {
        content: String,
    },
    EmptyLine,
    BreakLine,
    PlaceHolderLine {
        content: String,
    },
    UrlLink {
        name: String,
        url: String,
    },
    LoreLink {
        name: String,
        path: String,
    },
    Comment {
        content: String,
    },
    Image {
        url: String,
    },
    InnerUrlOpen {
        title: String,
        url: String,
    },
    InnerUrlClose {
        title: String,
        url: String,
    },
    InnerLoreOpen {
        title: String,
        path: String,
    },
    InnerLoreClose {
        title: String,
        path: String,
    },
}
