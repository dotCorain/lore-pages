use crate::ir::anchor::Anchor;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Category {
    pub nodes: Vec<Anchor>,
}

impl Category {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, anchor: Anchor) {
        self.nodes.push(anchor);
    }
}
