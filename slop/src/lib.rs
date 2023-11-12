// Local modules
pub mod ast;
mod format;
pub mod menu;
mod parser;
mod quant;
mod semantic;
mod svg;

pub use parser::Error as ParseError;
pub use semantic::Error as CompilationError;

use std::{fmt::Display, vec::IntoIter};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse error: {0}")]
    ParseError(#[from] ParseError),
    #[error("compilation")]
    CompilationError(#[from] CompilationError),
}

#[derive(Debug)]
pub struct Errors(pub Vec<Error>);

impl From<Vec<Error>> for Errors {
    fn from(value: Vec<Error>) -> Self {
        Self(value)
    }
}

impl IntoIterator for Errors {
    type Item = Error;

    type IntoIter = IntoIter<Error>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::error::Error for Errors {}
impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }
        write!(f, "Found {} errors:", self.0.len())?;
        for error in &self.0 {
            writeln!(f, "{error}")?;
        }
        Ok(())
    }
}

pub fn parse(src: &str) -> (ast::SourceFile, Errors) {
    let (ast_file, errors) = parser::parse(src);
    (
        ast_file,
        errors
            .into_iter()
            .map(Error::ParseError)
            .collect::<Vec<Error>>()
            .into(),
    )
}
pub fn format(src: &str) -> (String, Errors) {
    let (src_ast, errors) = parse(src);
    (format::format(&src_ast), errors)
}
pub fn compile(src: &str) -> (semantic::SourceFile, Errors) {
    let (ast_file, parse_errors) = parser::parse(src);
    let (sem_file, compilation_errors) = semantic::convert_source_file(&ast_file);
    (
        sem_file,
        parse_errors
            .into_iter()
            .map(Error::ParseError)
            .chain(compilation_errors.into_iter().map(Error::CompilationError))
            .collect::<Vec<Error>>()
            .into(),
    )
}

pub fn to_svgs(src: &str) -> (Vec<String>, Errors) {
    let (sem_file, errors) = compile(src);
    (svg::to_svgs(&sem_file), errors)
}
