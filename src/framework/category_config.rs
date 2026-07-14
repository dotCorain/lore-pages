use serde::Deserialize;

/// Placeholder configuration for future category-level settings.
///
/// Currently empty, but kept in the API so that future extensions
/// (e.g. tag lists, domain filtering) don't require breaking changes.
#[derive(Deserialize, Default)]
pub struct CategoryConfig {}

impl CategoryConfig {}
