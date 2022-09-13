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

pub type Error<'a> = ParseError<usize, String, &'static str>;

pub fn parse(src: &str) -> Result<ast::SourceFile, Error> {
    let sf = parser::SourceFileParser::new()
        .parse(&src)
        // Map the err tokens to an owned value since otherwise the
        // input would have to live as long as the error which has a static lifetime.
        .map_err(|err| err.map_token(|tok| tok.to_string()))?;
    Ok(sf)
}

pub fn format(src: &str) -> Result<String, Error> {
    let src_ast = parse(src)?;
    Ok(format::format(&src_ast))
}
