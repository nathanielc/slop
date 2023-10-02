use anyhow::anyhow;
use std::convert::TryFrom;

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
        derived: bool,
        quantity: Vec<Quantity>,
        unit: Option<String>,
        name: String,
    },
    UnaryOp(Box<Operand>, String),
    BinaryOp(Box<Operand>, Box<Operand>, String),
}

#[derive(Debug, PartialEq)]
pub enum Quantity {
    Number(String),
    Fraction(String),
}

impl TryFrom<&Quantity> for f64 {
    type Error = anyhow::Error;

    fn try_from(value: &Quantity) -> Result<Self, Self::Error> {
        match value {
            Quantity::Number(n) => Ok(n.parse::<f64>()?),
            Quantity::Fraction(f) => {
                let parts: Vec<&str> = f.split('/').collect();
                if parts.len() == 2 {
                    let numerator = parts[0].parse::<f64>()?;
                    let denominator = parts[1].parse::<f64>()?;
                    Ok(numerator / denominator)
                } else {
                    Err(anyhow!("invalid fraction literal"))
                }
            }
        }
    }
}

impl Quantity {
    pub(crate) fn text(&self) -> &str {
        match self {
            Quantity::Number(text) => text.as_str(),
            Quantity::Fraction(text) => text.as_str(),
        }
    }
}
