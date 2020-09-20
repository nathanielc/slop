use super::*;
use ast::{Operand, Recipe, SourceFile};
use lalrpop_util::ParseError;

fn test_parse(src: &str, f: Result<ast::SourceFile, Error>) {
    assert_eq!(f, parse(src))
}
#[test]
fn ingredient() {
    test_parse(
        "<*sugar 1 cup>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: None,
                preamble: None,
                comment: None,
                root: Operand::Ingredient("sugar 1 cup".to_string()),
            }],
        }),
    );
}
#[test]
fn unary() {
    test_parse(
        "<*sugar 1 cup =pulverize>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: None,
                preamble: None,
                comment: None,
                root: Operand::UnaryOp(
                    Box::new(Operand::Ingredient("sugar 1 cup".to_string())),
                    "pulverize".to_string(),
                ),
            }],
        }),
    );
}
#[test]
fn binary() {
    test_parse(
        "<*sugar 1 cup *milk 3 cups #boil and stir>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: None,
                preamble: None,
                comment: None,
                root: Operand::BinaryOp(
                    Box::new(Operand::Ingredient("sugar 1 cup".to_string())),
                    Box::new(Operand::Ingredient("milk 3 cups".to_string())),
                    "boil and stir".to_string(),
                ),
            }],
        }),
    );
}
#[test]
fn title() {
    test_parse(
        "<**Sugar *sugar 1 cup>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: Some("Sugar".to_string()),
                preamble: None,
                comment: None,
                root: Operand::Ingredient("sugar 1 cup".to_string()),
            }],
        }),
    );
}
#[test]
fn preamble() {
    test_parse(
        "< ##preheat oven *sugar 1 cup>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: None,
                preamble: Some("preheat oven".to_string()),
                comment: None,
                root: Operand::Ingredient("sugar 1 cup".to_string()),
            }],
        }),
    );
}
#[test]
fn title_preamble() {
    test_parse(
        "<**Sugar ##preheat oven *sugar 1 cup>",
        Ok(SourceFile {
            recipes: vec![Recipe {
                title: Some("Sugar".to_string()),
                preamble: Some("preheat oven".to_string()),
                comment: None,
                root: Operand::Ingredient("sugar 1 cup".to_string()),
            }],
        }),
    );
}
#[test]
fn simple_recipe() {
    test_parse(
        "<
*water 6 cups =boil
*macarroni noodles 2 cups #boil till soft =drain
*butter 1/4 cup #stir until melted
*milk 1/3 cup #stir
*dried cheese one pouch #stir until well mixed
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
                                        Box::new(Operand::Ingredient("water 6 cups".to_string())),
                                        "boil".to_string(),
                                    )),
                                    Box::new(Operand::Ingredient(
                                        "macarroni noodles 2 cups".to_string(),
                                    )),
                                    "boil till soft".to_string(),
                                )),
                                "drain".to_string(),
                            )),
                            Box::new(Operand::Ingredient("butter 1/4 cup".to_string())),
                            "stir until melted".to_string(),
                        )),
                        Box::new(Operand::Ingredient("milk 1/3 cup".to_string())),
                        "stir".to_string(),
                    )),
                    Box::new(Operand::Ingredient("dried cheese one pouch".to_string())),
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
                                            Box::new(Operand::Ingredient("butter".to_string())),
                                            "soften".to_string(),
                                        )),
                                        Box::new(Operand::BinaryOp(
                                            Box::new(Operand::BinaryOp(
                                                Box::new(Operand::Ingredient("sugar".to_string())),
                                                Box::new(Operand::Ingredient(
                                                    "brown sugar".to_string(),
                                                )),
                                                "+".to_string(),
                                            )),
                                            Box::new(Operand::Ingredient("vanilla".to_string())),
                                            "+".to_string(),
                                        )),
                                        "beat".to_string(),
                                    )),
                                    Box::new(Operand::Ingredient("eggs".to_string())),
                                    "beat one at a time".to_string(),
                                )),
                                Box::new(Operand::BinaryOp(
                                    Box::new(Operand::BinaryOp(
                                        Box::new(Operand::Ingredient("flour".to_string())),
                                        Box::new(Operand::Ingredient("soda".to_string())),
                                        "+".to_string(),
                                    )),
                                    Box::new(Operand::Ingredient("salt".to_string())),
                                    "mix".to_string(),
                                )),
                                "beat slowly".to_string(),
                            )),
                            Box::new(Operand::BinaryOp(
                                Box::new(Operand::Ingredient("chocolate chips".to_string())),
                                Box::new(Operand::Ingredient("chopped nuts".to_string())),
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
            token: (1, Token(1, "#"), 2),
            expected: vec![
                "\"##\"".to_string(),
                "\"*\"".to_string(),
                "\"**\"".to_string(),
            ],
        }),
    );
}
