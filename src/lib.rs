// Local modules
pub mod ast;
#[cfg(test)]
mod slop_test;
pub mod semantic;
pub mod svg;

// Bring in generated parser for slop
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(parser);

pub fn parse(src :&str) -> ast::Recipe{
    parser::RecipeParser::new().parse(&src).unwrap()
}

