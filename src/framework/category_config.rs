use serde::Deserialize;

#[derive(Deserialize)]
pub struct CategoryConfig {}

impl CategoryConfig {}

impl Default for CategoryConfig {
    fn default() -> Self {
        Self {}
    }
}
