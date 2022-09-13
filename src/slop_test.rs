use super::*;
use ast::{Operand, Recipe, SourceFile};
use lalrpop_util::ParseError;

fn test_parse(src: &str, f: Result<ast::SourceFile, Error>) {
    assert_eq!(f, parse(src))
}
#[test]
fn ingredient() {
    test_parse(
        "<*1 cup: brown sugar>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: None,
                preamble: None,
                comment: None,
                root: Operand::Ingredient {
                    quantity: Some("1".to_string()),
                    unit: Some("cup".to_string()),
                    name: "brown sugar".to_string(),
                },
            }],
        }),
    );
}
#[test]
fn unary() {
    test_parse(
        "<*1 cup: sugar =pulverize>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: None,
                preamble: None,
                comment: None,
                root: Operand::UnaryOp(
                    Box::new(Operand::Ingredient {
                        quantity: Some("1".to_string()),
                        unit: Some("cup".to_string()),
                        name: "sugar".to_string(),
                    }),
                    "pulverize".to_string(),
                ),
            }],
        }),
    );
}
#[test]
fn binary() {
    test_parse(
        "<*1 cup: sugar *3 cups: milk #boil and stir>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: None,
                preamble: None,
                comment: None,
                root: Operand::BinaryOp(
                    Box::new(Operand::Ingredient {
                        quantity: Some("1".to_string()),
                        unit: Some("cup".to_string()),
                        name: "sugar".to_string(),
                    }),
                    Box::new(Operand::Ingredient {
                        quantity: Some("3".to_string()),
                        unit: Some("cups".to_string()),
                        name: "milk".to_string(),
                    }),
                    "boil and stir".to_string(),
                ),
            }],
        }),
    );
}
#[test]
fn title() {
    test_parse(
        "<**Sugar *1 cup: sugar>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: Some("Sugar".to_string()),
                preamble: None,
                comment: None,
                root: Operand::Ingredient {
                    quantity: Some("1".to_string()),
                    unit: Some("cup".to_string()),
                    name: "sugar".to_string(),
                },
            }],
        }),
    );
}
#[test]
fn preamble() {
    test_parse(
        "< ##preheat oven *1 cup: sugar>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: None,
                preamble: Some("preheat oven".to_string()),
                comment: None,
                root: Operand::Ingredient {
                    quantity: Some("1".to_string()),
                    unit: Some("cup".to_string()),
                    name: "sugar".to_string(),
                },
            }],
        }),
    );
}
#[test]
fn title_preamble() {
    test_parse(
        "<**Sugar ##preheat oven *1 cup: sugar>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: Some("Sugar".to_string()),
                preamble: Some("preheat oven".to_string()),
                comment: None,
                root: Operand::Ingredient {
                    quantity: Some("1".to_string()),
                    unit: Some("cup".to_string()),
                    name: "sugar".to_string(),
                },
            }],
        }),
    );
}
#[test]
fn simple_recipe() {
    test_parse(
        "<
*6 cups: water =boil
*2 cups: macarroni noodles #boil till soft =drain
*1/4 cup: butter #stir until melted
*1/3 cup: milk #stir
*1 pouch: dried cheese #stir until well mixed
>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: None,
                preamble: None,
                comment: None,
                root: Operand::BinaryOp(
                    Box::new(Operand::BinaryOp(
                        Box::new(Operand::BinaryOp(
                            Box::new(Operand::UnaryOp(
                                Box::new(Operand::BinaryOp(
                                    Box::new(Operand::UnaryOp(
                                        Box::new(Operand::Ingredient {
                                            quantity: Some("6".to_string()),
                                            unit: Some("cups".to_string()),
                                            name: "water".to_string(),
                                        }),
                                        "boil".to_string(),
                                    )),
                                    Box::new(Operand::Ingredient {
                                        quantity: Some("2".to_string()),
                                        unit: Some("cups".to_string()),
                                        name: "macarroni noodles".to_string(),
                                    }),
                                    "boil till soft".to_string(),
                                )),
                                "drain".to_string(),
                            )),
                            Box::new(Operand::Ingredient {
                                quantity: Some("1/4".to_string()),
                                unit: Some("cup".to_string()),
                                name: "butter".to_string(),
                            }),
                            "stir until melted".to_string(),
                        )),
                        Box::new(Operand::Ingredient {
                            quantity: Some("1/3".to_string()),
                            unit: Some("cup".to_string()),
                            name: "milk".to_string(),
                        }),
                        "stir".to_string(),
                    )),
                    Box::new(Operand::Ingredient {
                        quantity: Some("1".to_string()),
                        unit: Some("pouch".to_string()),
                        name: "dried cheese".to_string(),
                    }),
                    "stir until well mixed".to_string(),
                ),
            }],
        }),
    );
}
#[test]
fn cookies() {
    test_parse(
        "<
*butter =soften
*sugar
*brown sugar #+
*vanilla #+ #beat
*eggs # beat one at a time
*flour
*soda #+
*salt #mix #beat slowly
*chocolate chips
*chopped nuts #+ #stir =form into balls =bake 375F 10m
#* Yield 1 dozen cookies
>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: None,
                preamble: None,
                comment: Some("Yield 1 dozen cookies".to_string()),
                root: Operand::UnaryOp(
                    Box::new(Operand::UnaryOp(
                        Box::new(Operand::BinaryOp(
                            Box::new(Operand::BinaryOp(
                                Box::new(Operand::BinaryOp(
                                    Box::new(Operand::BinaryOp(
                                        Box::new(Operand::UnaryOp(
                                            Box::new(Operand::Ingredient {
                                                quantity: None,
                                                unit: None,
                                                name: "butter".to_string(),
                                            }),
                                            "soften".to_string(),
                                        )),
                                        Box::new(Operand::BinaryOp(
                                            Box::new(Operand::BinaryOp(
                                                Box::new(Operand::Ingredient {
                                                    quantity: None,
                                                    unit: None,
                                                    name: "sugar".to_string(),
                                                }),
                                                Box::new(Operand::Ingredient {
                                                    quantity: None,
                                                    unit: None,
                                                    name: "brown sugar".to_string(),
                                                }),
                                                "+".to_string(),
                                            )),
                                            Box::new(Operand::Ingredient {
                                                quantity: None,
                                                unit: None,
                                                name: "vanilla".to_string(),
                                            }),
                                            "+".to_string(),
                                        )),
                                        "beat".to_string(),
                                    )),
                                    Box::new(Operand::Ingredient {
                                        quantity: None,
                                        unit: None,
                                        name: "eggs".to_string(),
                                    }),
                                    "beat one at a time".to_string(),
                                )),
                                Box::new(Operand::BinaryOp(
                                    Box::new(Operand::BinaryOp(
                                        Box::new(Operand::Ingredient {
                                            quantity: None,
                                            unit: None,
                                            name: "flour".to_string(),
                                        }),
                                        Box::new(Operand::Ingredient {
                                            quantity: None,
                                            unit: None,
                                            name: "soda".to_string(),
                                        }),
                                        "+".to_string(),
                                    )),
                                    Box::new(Operand::Ingredient {
                                        quantity: None,
                                        unit: None,
                                        name: "salt".to_string(),
                                    }),
                                    "mix".to_string(),
                                )),
                                "beat slowly".to_string(),
                            )),
                            Box::new(Operand::BinaryOp(
                                Box::new(Operand::Ingredient {
                                    quantity: None,
                                    unit: None,
                                    name: "chocolate chips".to_string(),
                                }),
                                Box::new(Operand::Ingredient {
                                    quantity: None,
                                    unit: None,
                                    name: "chopped nuts".to_string(),
                                }),
                                "+".to_string(),
                            )),
                            "stir".to_string(),
                        )),
                        "form into balls".to_string(),
                    )),
                    "bake 375F 10m".to_string(),
                ),
            }],
        }),
    );
}

#[test]
fn invalid_token_error() {
    test_parse(
        "<#>",
        Err(ParseError::UnrecognizedToken {
            token: (1, Token(2, "#"), 2),
            expected: vec![
                "\"##\"".to_string(),
                "\"*\"".to_string(),
                "\"**\"".to_string(),
            ],
        }),
    );
}
