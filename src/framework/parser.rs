use crate::framework::category_config::CategoryConfig;
use crate::framework::parser_config::ParserConfig;
use crate::ir::category::Category;

/// Trait for parsing Lore markup source text into a [`Category`] document.
///
/// Implementations receive the raw input string plus configuration and
/// should return a fully populated `Category`. The default implementation
/// is [`LorePagesParser`](crate::parser::LorePagesParser).
pub trait Parser {
    fn parse(
        &self,
        input: &str,
        category_config: &CategoryConfig,
        parser_config: &ParserConfig,
    ) -> Category;
}
