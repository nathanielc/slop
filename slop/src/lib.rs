use lalrpop_util::{lalrpop_mod, ParseError};

// Local modules
pub mod ast;
pub mod format;
pub mod menu;
mod quant;
pub mod semantic;
pub mod svg;

#[cfg(test)]
mod parser_test;

lalrpop_mod!(
    #[allow(clippy::all, missing_debug_implementations)]
    pub parser
);

pub type Error<'a> = ParseError<usize, String, &'static str>;

pub fn parse(src: &str) -> Result<ast::SourceFile, Error> {
    let sf = parser::SourceFileParser::new()
        .parse(src)
        // Map the err tokens to an owned value since otherwise the
        // input would have to live as long as the error which has a static lifetime.
        .map_err(|err| err.map_token(|tok| tok.to_string()))?;
    Ok(sf)
}

pub fn format(src: &str) -> Result<String, Error> {
    let src_ast = parse(src)?;
    Ok(format::format(&src_ast))
}
