use crate::framework::category_config::CategoryConfig;
use crate::framework::renderer_config::RenderConfig;
use crate::ir::category::Category;

pub trait Renderer {
    fn render(
        &self,
        doc: &Category,
        category_config: &CategoryConfig,
        renderer_config: &RenderConfig,
    ) -> String;
}
