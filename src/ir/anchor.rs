/// The atomic node types that make up a Lore document.
///
/// Each variant represents a distinct syntactic element in the Lore markup
/// language. The parser produces a flat list of these nodes, which the
/// renderer then converts to the target output format (HTML by default).
#[derive(Debug, Clone, PartialEq)]
pub enum Anchor {
    /// A heading at the given level (1–4, corresponding to `#`–`####`).
    Heading {
        level: u8,
        content: String,
        /// Optional auto-link path for H2 headings (set by `Category::auto_link_h2`).
        link: Option<String>,
    },
    /// A paragraph of text.
    Paragraph {
        content: String,
    },
    /// An empty line (whitespace-only lines after unescaping).
    EmptyLine,
    /// A horizontal breakline (default: `---`).
    BreakLine,
    /// A placeholder line (default: `_` prefix). Not rendered.
    PlaceHolderLine {
        content: String,
    },
    /// An external hyperlink: `name | url`.
    UrlLink {
        name: String,
        url: String,
    },
    /// An internal Lore page link: `name = path`.
    LoreLink {
        name: String,
        path: String,
    },
    /// An HTML comment: `% content`.
    Comment {
        content: String,
    },
    /// An image: `| url`.
    Image {
        url: String,
    },
    /// Open a foldable block that loads external content from a URL.
    InnerUrlOpen {
        title: String,
        url: String,
    },
    /// Close a foldable block that loads external content from a URL.
    InnerUrlClose {
        title: String,
        url: String,
    },
    /// Open a foldable block that loads internal Lore content.
    InnerLoreOpen {
        title: String,
        path: String,
    },
    /// Close a foldable block that loads internal Lore content.
    InnerLoreClose {
        title: String,
        path: String,
    },
}
