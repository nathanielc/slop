use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;

// Local modules
pub mod ast;
pub mod format;
pub mod semantic;
#[cfg(test)]
mod slop_test;
pub mod svg;

// Bring in generated parser for slop
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(parser);

pub type Error<'a> = ParseError<usize, Token<'a>, &'static str>;

pub fn parse(src: &str) -> Result<ast::SourceFile, Error> {
    parser::SourceFileParser::new().parse(&src)
}

pub fn format(src: &str) -> Result<String, Error> {
    let src_ast = parse(src)?;
    Ok(format::format(&src_ast))
}
