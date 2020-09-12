#[derive(Debug, PartialEq)]
pub struct Recipe {
    pub title: Option<String>,
    pub preamble: Option<String>,
    pub root: Operand,
}
#[derive(Debug, PartialEq)]
pub enum Operand {
    Ingredient(String),
    UnaryOp(Box<Operand>, String),
    BinaryOp(Box<Operand>, Box<Operand>, String),
}
