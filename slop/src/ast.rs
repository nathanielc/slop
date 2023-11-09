use std::{convert::TryFrom, ops::Range};

use anyhow::anyhow;

pub type Position = Range<usize>;

pub trait Positioned {
    fn position(&self) -> Position;
}

#[derive(Debug, PartialEq)]
pub struct SourceFile {
    pub recipes: Vec<Recipe>,
}

#[derive(Debug, PartialEq)]
pub struct Recipe {
    pub position: Position,
    pub title: Option<String>,
    pub preamble: Option<String>,
    pub comment: Option<String>,
    pub root: Operand,
}
#[derive(Debug, PartialEq)]
pub enum Operand {
    Ingredient {
        position: Position,
        derived: bool,
        quantities: Vec<Quantity>,
        unit: Option<String>,
        text: String,
    },
    UnaryOp {
        position: Position,
        operand: Box<Operand>,
        text: String,
    },
    BinaryOp {
        position: Position,
        first: Box<Operand>,
        second: Box<Operand>,
        text: String,
    },
    MissingOperand {
        position: Position,
    },
    UnusedOperands {
        position: Position,
        operands: Vec<Operand>,
    },
}

impl Positioned for Operand {
    fn position(&self) -> Position {
        match self {
            Operand::Ingredient { position, .. } => position.clone(),
            Operand::UnaryOp { position, .. } => position.clone(),
            Operand::BinaryOp { position, .. } => position.clone(),
            Operand::MissingOperand { position } => position.clone(),
            Operand::UnusedOperands { position, .. } => position.clone(),
        }
    }
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
