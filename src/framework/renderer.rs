use crate::framework::category_config::CategoryConfig;
use crate::framework::renderer_config::RenderConfig;
use crate::ir::category::Category;

pub trait Renderer<'a> {
    fn render(
        &self,
        doc: &'a Category,
        category_config: &'a CategoryConfig,
        renderer_config: &'a RenderConfig,
    ) -> String;
}
