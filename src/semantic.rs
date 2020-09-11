use crate::ast;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Ingredient {
        text: String,
    },
    Operator {
        text: String,
        operands: Vec<Operand>,
    },
}

pub fn convert_graph(op: ast::Operand) -> Operand {
    match op {
        ast::Operand::Ingredient(text) => Operand::Ingredient { text: text },
        ast::Operand::UnaryOp(operand, text) => Operand::Operator {
            text: text,
            operands: vec![convert_graph(*operand)],
        },
        ast::Operand::BinaryOp(left, right, text) => {
            let mut ops: Vec<Operand> = Vec::with_capacity(2);
            let l = convert_graph(*left);
            let r = convert_graph(*right);
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
                text: text,
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
        let sg = convert_graph(ast::Operand::BinaryOp(
            Box::new(ast::Operand::UnaryOp(
                Box::new(ast::Operand::Ingredient("butter".to_string())),
                "soften".to_string(),
            )),
            Box::new(ast::Operand::Ingredient("salt".to_string())),
            "mix".to_string(),
        ));
        assert_eq!(
            sg,
            Operand::Operator {
                text: "mix".to_string(),
                operands: vec![
                    Operand::Operator {
                        text: "soften".to_string(),
                        operands: vec![Operand::Ingredient {
                            text: "butter".to_string(),
                        }],
                    },
                    Operand::Ingredient {
                        text: "salt".to_string(),
                    },
                ],
            }
        );
    }
    #[test]
    fn merge_binary_plus() {
        let sg = convert_graph(ast::Operand::BinaryOp(
            Box::new(ast::Operand::BinaryOp(
                Box::new(ast::Operand::Ingredient("flour".to_string())),
                Box::new(ast::Operand::Ingredient("baking soda".to_string())),
                "+".to_string(),
            )),
            Box::new(ast::Operand::Ingredient("salt".to_string())),
            "mix".to_string(),
        ));
        assert_eq!(
            sg,
            Operand::Operator {
                text: "mix".to_string(),
                operands: vec![
                    Operand::Ingredient {
                        text: "flour".to_string(),
                    },
                    Operand::Ingredient {
                        text: "baking soda".to_string(),
                    },
                    Operand::Ingredient {
                        text: "salt".to_string(),
                    },
                ],
            }
        );
    }
    #[test]
    fn merge_binary_plus_nested() {
        let sg = convert_graph(ast::Operand::BinaryOp(
            Box::new(ast::Operand::BinaryOp(
                Box::new(ast::Operand::BinaryOp(
                    Box::new(ast::Operand::Ingredient("flour".to_string())),
                    Box::new(ast::Operand::Ingredient("baking soda".to_string())),
                    "+".to_string(),
                )),
                Box::new(ast::Operand::Ingredient("salt".to_string())),
                "+".to_string(),
            )),
            Box::new(ast::Operand::Ingredient("oats".to_string())),
            "mix".to_string(),
        ));
        assert_eq!(
            sg,
            Operand::Operator {
                text: "mix".to_string(),
                operands: vec![
                    Operand::Ingredient {
                        text: "flour".to_string(),
                    },
                    Operand::Ingredient {
                        text: "baking soda".to_string(),
                    },
                    Operand::Ingredient {
                        text: "salt".to_string(),
                    },
                    Operand::Ingredient {
                        text: "oats".to_string(),
                    },
                ],
            }
        );
    }
}
