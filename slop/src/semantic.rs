use crate::{
    ast::{self, Position},
    stack_parser,
};

use thiserror::Error;

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
pub struct Ingredient {
    pub position: Position,
    pub derived: bool,
    pub quantities: Option<(String, f64)>,
    pub unit: Option<String>,
    pub text: String,
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
        position: Position,
        text: String,
        operands: Vec<Operand>,
    },
    MissingOperand {
        position: Position,
    },
    UnusedOperands {
        position: Position,
        operands: Vec<Operand>,
    },
}

pub fn convert_source_file(f: &ast::SourceFile) -> (SourceFile, Vec<Error>) {
    let mut errors = Vec::new();
    (
        SourceFile {
            recipes: f
                .recipes
                .iter()
                .map(|r| convert_recipe(&r, &mut errors))
                .collect(),
        },
        errors,
    )
}
fn convert_recipe(r: &ast::Recipe, errors: &mut Vec<Error>) -> Recipe {
    Recipe {
        position: r.position.clone(),
        title: r.title.clone(),
        preamble: r.preamble.clone(),
        comment: r.comment.clone(),
        root: convert_operand(&r.root, errors),
    }
}
pub fn convert_operand(op: &ast::Operand, errors: &mut Vec<Error>) -> Operand {
    match op {
        ast::Operand::Ingredient {
            position,
            derived,
            quantities,
            unit,
            text,
        } => Operand::Ingredient(Ingredient {
            position: position.clone(),
            derived: *derived,
            quantities: quantities
                .iter()
                .map(|q| -> (String, f64) {
                    (
                        q.text().to_owned(),
                        q.try_into()
                            .expect("quantity should always parse into float"),
                    )
                })
                .collect::<Vec<(String, f64)>>()
                .iter()
                .fold(None, |acc, q| {
                    if let Some(acc) = acc {
                        Some(((acc.0 + " " + q.0.as_str()).to_owned(), acc.1 + q.1))
                    } else {
                        Some(q.to_owned())
                    }
                }),
            unit: unit.clone(),
            text: text.clone(),
        }),
        ast::Operand::UnaryOp {
            position,
            operand,
            text,
        } => Operand::Operator {
            position: position.clone(),
            operands: vec![convert_operand(operand, errors)],
            text: text.clone(),
        },
        ast::Operand::BinaryOp {
            position,
            first,
            second,
            text,
        } => {
            let mut ops: Vec<Operand> = Vec::with_capacity(2);
            let f = convert_operand(first, errors);
            let s = convert_operand(second, errors);
            for mut op in [f, s] {
                if let Operand::Operator { text, operands, .. } = &mut op {
                    if text == "+" {
                        ops.append(operands);
                        continue;
                    }
                }
                ops.push(op)
            }

            Operand::Operator {
                position: position.clone(),
                text: text.clone(),
                operands: ops,
            }
        }
        ast::Operand::MissingOperand { position } => {
            errors.push(Error::MissingOperand(position.clone()));
            Operand::MissingOperand {
                position: position.clone(),
            }
        }
        ast::Operand::UnusedOperands { position, operands } => {
            errors.push(Error::UnusedOperands(operands.len(), position.clone()));
            Operand::UnusedOperands {
                position: position.clone(),
                operands: operands
                    .iter()
                    .map(|operand| convert_operand(operand, errors))
                    .collect(),
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("missing operand")]
    MissingOperand(Position),
    #[error("unused operands: {0}")]
    UnusedOperands(usize, Position),
}

#[cfg(test)]
mod test {
    use expect_test::{expect, Expect};

    use crate::parse;

    use super::*;

    fn test_convert_source(src: &str, expectation: Expect, expected_errors: Expect) {
        let (src_ast, errors) = parse(src);
        assert!(errors.0.is_empty());
        let (src_semantic, errors) = convert_source_file(&src_ast);
        expectation.assert_debug_eq(&src_semantic);
        expected_errors.assert_debug_eq(&errors);
    }

    #[test]
    fn one_to_one() {
        test_convert_source(
            r#"<
                *butter = soften
                *salt # mix
            >"#,
            expect![[r#"
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..76,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Operator {
                                position: 57..76,
                                text: "mix",
                                operands: [
                                    Operator {
                                        position: 26..52,
                                        text: "soften",
                                        operands: [
                                            Ingredient(
                                                Ingredient {
                                                    position: 18..27,
                                                    derived: false,
                                                    quantities: None,
                                                    unit: None,
                                                    text: "butter",
                                                },
                                            ),
                                        ],
                                    },
                                    Ingredient(
                                        Ingredient {
                                            position: 51..58,
                                            derived: false,
                                            quantities: None,
                                            unit: None,
                                            text: "salt",
                                        },
                                    ),
                                ],
                            },
                        },
                    ],
                }
            "#]],
            expect![[r#"
                []
            "#]],
        );
    }
    #[test]
    fn merge_binary_plus() {
        test_convert_source(
            r#"<
                *flour
                *baking soda #+
                *salt #mix
            >"#,
            expect![[r#"
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..97,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Operator {
                                position: 79..97,
                                text: "mix",
                                operands: [
                                    Ingredient(
                                        Ingredient {
                                            position: 18..42,
                                            derived: false,
                                            quantities: None,
                                            unit: None,
                                            text: "flour",
                                        },
                                    ),
                                    Ingredient(
                                        Ingredient {
                                            position: 41..55,
                                            derived: false,
                                            quantities: None,
                                            unit: None,
                                            text: "baking soda",
                                        },
                                    ),
                                    Ingredient(
                                        Ingredient {
                                            position: 73..80,
                                            derived: false,
                                            quantities: None,
                                            unit: None,
                                            text: "salt",
                                        },
                                    ),
                                ],
                            },
                        },
                    ],
                }
            "#]],
            expect![[r#"
                []
            "#]],
        );
    }
    #[test]
    fn merge_binary_plus_nested() {
        test_convert_source(
            r#"<
                *flour
                *baking soda #+
                *salt #+
                *oats #mix
            >"#,
            expect![[r#"
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..122,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Operator {
                                position: 104..122,
                                text: "mix",
                                operands: [
                                    Ingredient(
                                        Ingredient {
                                            position: 18..42,
                                            derived: false,
                                            quantities: None,
                                            unit: None,
                                            text: "flour",
                                        },
                                    ),
                                    Ingredient(
                                        Ingredient {
                                            position: 41..55,
                                            derived: false,
                                            quantities: None,
                                            unit: None,
                                            text: "baking soda",
                                        },
                                    ),
                                    Ingredient(
                                        Ingredient {
                                            position: 73..80,
                                            derived: false,
                                            quantities: None,
                                            unit: None,
                                            text: "salt",
                                        },
                                    ),
                                    Ingredient(
                                        Ingredient {
                                            position: 98..105,
                                            derived: false,
                                            quantities: None,
                                            unit: None,
                                            text: "oats",
                                        },
                                    ),
                                ],
                            },
                        },
                    ],
                }
            "#]],
            expect![[r#"
                []
            "#]],
        );
    }
}
