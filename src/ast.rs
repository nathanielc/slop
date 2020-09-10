#[derive(Debug, PartialEq)]
pub enum Operand {
    Ingredient(String),
    UnaryOp(Box<Operand>, String),
    BinaryOp(Box<Operand>, Box<Operand>, String),
}
