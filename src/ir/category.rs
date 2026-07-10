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

    /// 给每个 H2 标题设置自动链接，指向 `标题名/index`。
    /// 渲染时 H2 文本本身就是链接，不需要额外插入 LoreLink 节点。
    pub fn auto_link_h2(&mut self) {
        for node in &mut self.nodes {
            if let Anchor::Heading { level: 2, content, link } = node {
                *link = Some(format!("{}/index", content));
            }
        }
    }
}
