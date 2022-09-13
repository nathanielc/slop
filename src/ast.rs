#[derive(Debug, PartialEq)]
pub struct SourceFile {
    pub recipes: Vec<Recipe>,
}

#[derive(Debug, PartialEq)]
pub struct Recipe {
    pub title: Option<String>,
    pub preamble: Option<String>,
    pub comment: Option<String>,
    pub root: Operand,
}
#[derive(Debug, PartialEq)]
pub enum Operand {
    Ingredient {
        quantity: Option<String>,
        unit: Option<String>,
        name: String,
    },
    UnaryOp(Box<Operand>, String),
    BinaryOp(Box<Operand>, Box<Operand>, String),
}
