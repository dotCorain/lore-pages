use crate::framework::category_config::CategoryConfig;
use crate::framework::parser_config::ParserConfig;
use crate::ir::category::Category;

pub trait Parser {
    fn parse(
        &self,
        input: &str,
        category_config: &CategoryConfig,
        parser_config: &ParserConfig,
    ) -> Category;
}
