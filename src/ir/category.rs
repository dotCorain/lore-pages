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

    /// 在每个 H2 标题后面自动插入一个 LoreLink，指向 `标题名/index`。
    pub fn auto_link_h2(&mut self) {
        let mut new_nodes = Vec::with_capacity(self.nodes.len() * 2);
        for node in self.nodes.drain(..) {
            match node {
                Anchor::Heading { level: 2, content } => {
                    let path = format!("{}/index", content);
                    new_nodes.push(Anchor::Heading { level: 2, content: content.clone() });
                    new_nodes.push(Anchor::LoreLink { name: content, path });
                }
                other => new_nodes.push(other),
            }
        }
        self.nodes = new_nodes;
    }
}
