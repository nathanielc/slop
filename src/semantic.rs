use crate::ast;
use anyhow::Result;

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
pub struct Ingredient {
    pub derived: bool,
    pub quantity: Option<(String, f64)>,
    pub unit: Option<String>,
    pub name: String,
}
#[derive(Debug, PartialEq)]
pub enum Quantity {
    Number(f64),
    Fraction(f64),
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Ingredient(Ingredient),
    Operator {
        text: String,
        operands: Vec<Operand>,
    },
}

pub fn convert_source_file(mut f: ast::SourceFile) -> SourceFile {
    SourceFile {
        recipes: f.recipes.drain(..).map(|r| convert_recipe(r)).collect(),
    }
}
pub fn convert_recipe(r: ast::Recipe) -> Recipe {
    Recipe {
        title: r.title,
        preamble: r.preamble,
        comment: r.comment,
        root: convert_operand(r.root),
    }
}
pub fn convert_operand(op: ast::Operand) -> Operand {
    match op {
        ast::Operand::Ingredient {
            derived,
            quantity,
            unit,
            name,
        } => Operand::Ingredient(Ingredient {
            derived,
            quantity: quantity
                .iter()
                .map(|q| -> Result<(String, f64)> { Ok((q.text().to_owned(), q.try_into()?)) })
                .collect::<Result<Vec<(String, f64)>>>()
                .expect("TODO ast -> semantic quantity")
                .iter()
                .fold(None, |acc, q| {
                    if let Some(acc) = acc {
                        Some(((acc.0 + " " + q.0.as_str()).to_owned(), acc.1 + q.1))
                    } else {
                        Some(q.to_owned())
                    }
                }),
            unit,
            name,
        }),
        ast::Operand::UnaryOp(operand, text) => Operand::Operator {
            text,
            operands: vec![convert_operand(*operand)],
        },
        ast::Operand::BinaryOp(left, right, text) => {
            let mut ops: Vec<Operand> = Vec::with_capacity(2);
            let l = convert_operand(*left);
            let r = convert_operand(*right);
            for mut op in vec![l, r] {
                if let Operand::Operator { text, operands } = &mut op {
                    if text == "+" {
                        ops.extend(operands.drain(..));
                        continue;
                    }
                }
                ops.push(op)
            }

            Operand::Operator {
                text,
                operands: ops,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_to_one() {
        let sg = convert_operand(ast::Operand::BinaryOp(
            Box::new(ast::Operand::UnaryOp(
                Box::new(ast::Operand::Ingredient {
                    derived: false,
                    quantity: vec![],
                    unit: None,
                    name: "butter".to_string(),
                }),
                "soften".to_string(),
            )),
            Box::new(ast::Operand::Ingredient {
                derived: false,
                quantity: vec![],
                unit: None,
                name: "salt".to_string(),
            }),
            "mix".to_string(),
        ));
        assert_eq!(
            sg,
            Operand::Operator {
                text: "mix".to_string(),
                operands: vec![
                    Operand::Operator {
                        text: "soften".to_string(),
                        operands: vec![Operand::Ingredient(Ingredient {
                            derived: false,
                            quantity: None,
                            unit: None,
                            name: "butter".to_string(),
                        })],
                    },
                    Operand::Ingredient(Ingredient {
                        derived: false,
                        quantity: None,
                        unit: None,
                        name: "salt".to_string(),
                    }),
                ],
            }
        );
    }
    #[test]
    fn merge_binary_plus() {
        let sg = convert_operand(ast::Operand::BinaryOp(
            Box::new(ast::Operand::BinaryOp(
                Box::new(ast::Operand::Ingredient {
                    derived: false,
                    quantity: vec![],
                    unit: None,
                    name: "flour".to_string(),
                }),
                Box::new(ast::Operand::Ingredient {
                    derived: false,
                    quantity: vec![],
                    unit: None,
                    name: "baking soda".to_string(),
                }),
                "+".to_string(),
            )),
            Box::new(ast::Operand::Ingredient {
                derived: false,
                quantity: vec![],
                unit: None,
                name: "salt".to_string(),
            }),
            "mix".to_string(),
        ));
        assert_eq!(
            sg,
            Operand::Operator {
                text: "mix".to_string(),
                operands: vec![
                    Operand::Ingredient(Ingredient {
                        derived: false,
                        quantity: None,
                        unit: None,
                        name: "flour".to_string(),
                    }),
                    Operand::Ingredient(Ingredient {
                        derived: false,
                        quantity: None,
                        unit: None,
                        name: "baking soda".to_string(),
                    }),
                    Operand::Ingredient(Ingredient {
                        derived: false,
                        quantity: None,
                        unit: None,
                        name: "salt".to_string(),
                    }),
                ],
            }
        );
    }
    #[test]
    fn merge_binary_plus_nested() {
        let sg = convert_operand(ast::Operand::BinaryOp(
            Box::new(ast::Operand::BinaryOp(
                Box::new(ast::Operand::BinaryOp(
                    Box::new(ast::Operand::Ingredient {
                        derived: false,
                        quantity: vec![],
                        unit: None,
                        name: "flour".to_string(),
                    }),
                    Box::new(ast::Operand::Ingredient {
                        derived: false,
                        quantity: vec![],
                        unit: None,
                        name: "baking soda".to_string(),
                    }),
                    "+".to_string(),
                )),
                Box::new(ast::Operand::Ingredient {
                    derived: false,
                    quantity: vec![],
                    unit: None,
                    name: "salt".to_string(),
                }),
                "+".to_string(),
            )),
            Box::new(ast::Operand::Ingredient {
                derived: false,
                quantity: vec![],
                unit: None,
                name: "oats".to_string(),
            }),
            "mix".to_string(),
        ));
        assert_eq!(
            sg,
            Operand::Operator {
                text: "mix".to_string(),
                operands: vec![
                    Operand::Ingredient(Ingredient {
                        derived: false,
                        quantity: None,
                        unit: None,
                        name: "flour".to_string(),
                    }),
                    Operand::Ingredient(Ingredient {
                        derived: false,
                        quantity: None,
                        unit: None,
                        name: "baking soda".to_string(),
                    }),
                    Operand::Ingredient(Ingredient {
                        derived: false,
                        quantity: None,
                        unit: None,
                        name: "salt".to_string(),
                    }),
                    Operand::Ingredient(Ingredient {
                        derived: false,
                        quantity: None,
                        unit: None,
                        name: "oats".to_string(),
                    }),
                ],
            }
        );
    }
}
