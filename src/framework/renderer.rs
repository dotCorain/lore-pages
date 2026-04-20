use crate::ir::category::Category;

pub trait Renderer {
    fn render(&self, doc: &Category, title: &str, css_url: &str) -> String;
}
