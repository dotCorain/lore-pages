use crate::framework::parser_config::ParserConfig;
use crate::ir::category::Category;

pub trait Parser<'a> {
    fn parse(
        &self,
        input: &'a str,
        config: &'a ParserConfig,
    ) -> Category;
}
