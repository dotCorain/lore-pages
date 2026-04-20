use crate::ir::category::Category;

pub trait Parser {
    fn parse(&self, input: &str) -> Category;
}
