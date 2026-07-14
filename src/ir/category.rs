use crate::ir::anchor::Anchor;

/// A complete Lore document: an ordered list of [`Anchor`] nodes.
///
/// This is the central data structure produced by the parser and consumed
/// by the renderer. The list preserves the line-by-line order of the
/// source document.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Category {
    pub nodes: Vec<Anchor>,
}

impl Category {
    /// Create an empty document.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a node to the end of the document.
    pub fn push(&mut self, anchor: Anchor) {
        self.nodes.push(anchor);
    }

    /// Automatically set links for every H2 heading.
    ///
    /// Each H2 heading gets a link pointing to `{content}/index`. The
    /// renderer uses this to make H2 headings clickable, linking to the
    /// corresponding sub-page.
    pub fn auto_link_h2(&mut self) {
        for node in &mut self.nodes {
            if let Anchor::Heading {
                level: 2, content, link
            } = node
            {
                *link = Some(format!("{}/index", content));
            }
        }
    }
}
