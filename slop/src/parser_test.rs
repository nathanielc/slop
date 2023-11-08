use crate::ast::Quantity;

use super::*;
use ast::{Operand, Recipe, SourceFile};
use expect_test::{expect, Expect};
use lalrpop_util::ParseError;

fn test_parse(src: &str, expected: Expect) {
    let src_ast = parse(src);
    expected.assert_debug_eq(&src_ast);
}
#[test]
fn ingredient() {
    test_parse(
        "<*1 cup: brown sugar>",
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Ingredient {
                                derived: false,
                                quantity: [
                                    Number(
                                        "1",
                                    ),
                                ],
                                unit: Some(
                                    "cup",
                                ),
                                name: "brown sugar",
                            },
                        },
                    ],
                },
            )
        "#]],
    );
}
#[test]
fn ingredient_derived() {
    test_parse(
        "<*^1 cup: brown sugar>",
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Ingredient {
                                derived: true,
                                quantity: [
                                    Number(
                                        "1",
                                    ),
                                ],
                                unit: Some(
                                    "cup",
                                ),
                                name: "brown sugar",
                            },
                        },
                    ],
                },
            )
        "#]],
    );
}
#[test]
fn ingredient_derived_no_measure() {
    test_parse(
        "<*^egg yolk>",
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Ingredient {
                                derived: true,
                                quantity: [],
                                unit: None,
                                name: "egg yolk",
                            },
                        },
                    ],
                },
            )
        "#]],
    );
}
#[test]
fn ingredient_fractional_measure() {
    test_parse(
        "<*1 1/3 cups: milk>",
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Ingredient {
                                derived: false,
                                quantity: [
                                    Number(
                                        "1",
                                    ),
                                    Fraction(
                                        "1/3",
                                    ),
                                ],
                                unit: Some(
                                    "cups",
                                ),
                                name: "milk",
                            },
                        },
                    ],
                },
            )
        "#]],
    );
}
#[test]
fn unary() {
    test_parse(
        "<*1 cup: sugar =pulverize>",
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: None,
                            preamble: None,
                            comment: None,
                            root: UnaryOp(
                                Ingredient {
                                    derived: false,
                                    quantity: [
                                        Number(
                                            "1",
                                        ),
                                    ],
                                    unit: Some(
                                        "cup",
                                    ),
                                    name: "sugar",
                                },
                                "pulverize",
                            ),
                        },
                    ],
                },
            )
        "#]],
    );
}
#[test]
fn binary() {
    test_parse(
        "<*1 cup: sugar *3 cups: milk #boil and stir>",
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: None,
                            preamble: None,
                            comment: None,
                            root: BinaryOp(
                                Ingredient {
                                    derived: false,
                                    quantity: [
                                        Number(
                                            "1",
                                        ),
                                    ],
                                    unit: Some(
                                        "cup",
                                    ),
                                    name: "sugar",
                                },
                                Ingredient {
                                    derived: false,
                                    quantity: [
                                        Number(
                                            "3",
                                        ),
                                    ],
                                    unit: Some(
                                        "cups",
                                    ),
                                    name: "milk",
                                },
                                "boil and stir",
                            ),
                        },
                    ],
                },
            )
        "#]],
    );
}
#[test]
fn title() {
    test_parse(
        "<**Sugar *1 cup: sugar>",
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: Some(
                                "Sugar",
                            ),
                            preamble: None,
                            comment: None,
                            root: Ingredient {
                                derived: false,
                                quantity: [
                                    Number(
                                        "1",
                                    ),
                                ],
                                unit: Some(
                                    "cup",
                                ),
                                name: "sugar",
                            },
                        },
                    ],
                },
            )
        "#]],
    );
}
#[test]
fn preamble() {
    test_parse(
        "< ##preheat oven *1 cup: sugar>",
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: None,
                            preamble: Some(
                                "preheat oven",
                            ),
                            comment: None,
                            root: Ingredient {
                                derived: false,
                                quantity: [
                                    Number(
                                        "1",
                                    ),
                                ],
                                unit: Some(
                                    "cup",
                                ),
                                name: "sugar",
                            },
                        },
                    ],
                },
            )
        "#]],
    );
}
#[test]
fn title_preamble() {
    test_parse(
        "<**Sugar ##preheat oven *1 cup: sugar>",
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: Some(
                                "Sugar",
                            ),
                            preamble: Some(
                                "preheat oven",
                            ),
                            comment: None,
                            root: Ingredient {
                                derived: false,
                                quantity: [
                                    Number(
                                        "1",
                                    ),
                                ],
                                unit: Some(
                                    "cup",
                                ),
                                name: "sugar",
                            },
                        },
                    ],
                },
            )
        "#]],
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
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: None,
                            preamble: None,
                            comment: None,
                            root: BinaryOp(
                                BinaryOp(
                                    BinaryOp(
                                        UnaryOp(
                                            BinaryOp(
                                                UnaryOp(
                                                    Ingredient {
                                                        derived: false,
                                                        quantity: [
                                                            Number(
                                                                "6",
                                                            ),
                                                        ],
                                                        unit: Some(
                                                            "cups",
                                                        ),
                                                        name: "water",
                                                    },
                                                    "boil",
                                                ),
                                                Ingredient {
                                                    derived: false,
                                                    quantity: [
                                                        Number(
                                                            "2",
                                                        ),
                                                    ],
                                                    unit: Some(
                                                        "cups",
                                                    ),
                                                    name: "macarroni noodles",
                                                },
                                                "boil till soft",
                                            ),
                                            "drain",
                                        ),
                                        Ingredient {
                                            derived: false,
                                            quantity: [
                                                Fraction(
                                                    "1/4",
                                                ),
                                            ],
                                            unit: Some(
                                                "cup",
                                            ),
                                            name: "butter",
                                        },
                                        "stir until melted",
                                    ),
                                    Ingredient {
                                        derived: false,
                                        quantity: [
                                            Fraction(
                                                "1/3",
                                            ),
                                        ],
                                        unit: Some(
                                            "cup",
                                        ),
                                        name: "milk",
                                    },
                                    "stir",
                                ),
                                Ingredient {
                                    derived: false,
                                    quantity: [
                                        Number(
                                            "1",
                                        ),
                                    ],
                                    unit: Some(
                                        "pouch",
                                    ),
                                    name: "dried cheese",
                                },
                                "stir until well mixed",
                            ),
                        },
                    ],
                },
            )
        "#]],
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
        expect![[r#"
            Ok(
                SourceFile {
                    recipes: [
                        Recipe {
                            title: None,
                            preamble: None,
                            comment: Some(
                                "Yield 1 dozen cookies",
                            ),
                            root: UnaryOp(
                                UnaryOp(
                                    BinaryOp(
                                        BinaryOp(
                                            BinaryOp(
                                                BinaryOp(
                                                    UnaryOp(
                                                        Ingredient {
                                                            derived: false,
                                                            quantity: [],
                                                            unit: None,
                                                            name: "butter",
                                                        },
                                                        "soften",
                                                    ),
                                                    BinaryOp(
                                                        BinaryOp(
                                                            Ingredient {
                                                                derived: false,
                                                                quantity: [],
                                                                unit: None,
                                                                name: "sugar",
                                                            },
                                                            Ingredient {
                                                                derived: false,
                                                                quantity: [],
                                                                unit: None,
                                                                name: "brown sugar",
                                                            },
                                                            "+",
                                                        ),
                                                        Ingredient {
                                                            derived: false,
                                                            quantity: [],
                                                            unit: None,
                                                            name: "vanilla",
                                                        },
                                                        "+",
                                                    ),
                                                    "beat",
                                                ),
                                                Ingredient {
                                                    derived: false,
                                                    quantity: [],
                                                    unit: None,
                                                    name: "eggs",
                                                },
                                                "beat one at a time",
                                            ),
                                            BinaryOp(
                                                BinaryOp(
                                                    Ingredient {
                                                        derived: false,
                                                        quantity: [],
                                                        unit: None,
                                                        name: "flour",
                                                    },
                                                    Ingredient {
                                                        derived: false,
                                                        quantity: [],
                                                        unit: None,
                                                        name: "soda",
                                                    },
                                                    "+",
                                                ),
                                                Ingredient {
                                                    derived: false,
                                                    quantity: [],
                                                    unit: None,
                                                    name: "salt",
                                                },
                                                "mix",
                                            ),
                                            "beat slowly",
                                        ),
                                        BinaryOp(
                                            Ingredient {
                                                derived: false,
                                                quantity: [],
                                                unit: None,
                                                name: "chocolate chips",
                                            },
                                            Ingredient {
                                                derived: false,
                                                quantity: [],
                                                unit: None,
                                                name: "chopped nuts",
                                            },
                                            "+",
                                        ),
                                        "stir",
                                    ),
                                    "form into balls",
                                ),
                                "bake 375F 10m",
                            ),
                        },
                    ],
                },
            )
        "#]],
    );
}

#[test]
fn invalid_token_error() {
    test_parse(
        "<#>",
        expect![[r###"
            Err(
                UnrecognizedToken {
                    token: (
                        1,
                        "#",
                        2,
                    ),
                    expected: [
                        "\"##\"",
                        "\"*\"",
                        "\"**\"",
                    ],
                },
            )
        "###]],
    );
}
