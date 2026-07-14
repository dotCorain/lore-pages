use crate::framework::category_config::CategoryConfig;
use crate::framework::renderer_config::RenderConfig;
use crate::ir::category::Category;

/// Trait for rendering a [`Category`] document into an output string.
///
/// The default implementation is [`HtmlRenderer`](crate::render::HtmlRenderer),
/// which produces a complete HTML5 page. Custom renderers can be plugged in
/// via the same trait to target other formats.
pub trait Renderer {
    fn render(
        &self,
        doc: &Category,
        category_config: &CategoryConfig,
        renderer_config: &RenderConfig,
        source_path: Option<&str>,
    ) -> String;
}
