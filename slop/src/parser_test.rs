use expect_test::{expect, Expect};

fn test_parse(src: &str, expected: Expect) {
    let src_ast = crate::stack_parser::Parser::parse(src);
    expected.assert_debug_eq(&src_ast);
}
#[test]
fn ingredient() {
    test_parse(
        "<*1 cup: brown sugar>",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..21,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Ingredient {
                                position: 1..21,
                                derived: false,
                                quantities: [
                                    Number(
                                        "1",
                                    ),
                                ],
                                unit: Some(
                                    "cup",
                                ),
                                text: "brown sugar",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}
#[test]
fn ingredient_derived() {
    test_parse(
        "<*^1 cup: brown sugar>",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..22,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Ingredient {
                                position: 1..22,
                                derived: true,
                                quantities: [
                                    Number(
                                        "1",
                                    ),
                                ],
                                unit: Some(
                                    "cup",
                                ),
                                text: "brown sugar",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}
#[test]
fn ingredient_derived_no_measure() {
    test_parse(
        "<*^egg yolk>",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..12,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Ingredient {
                                position: 1..12,
                                derived: true,
                                quantities: [],
                                unit: None,
                                text: "egg yolk",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}
#[test]
fn ingredient_fractional_measure() {
    test_parse(
        "<*1 1/3 cups: milk>",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..19,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: Ingredient {
                                position: 1..19,
                                derived: false,
                                quantities: [
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
                                text: "milk",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}
#[test]
fn unary() {
    test_parse(
        "<*1 cup: sugar =pulverize>",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..26,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: UnaryOp {
                                position: 15..26,
                                operand: Ingredient {
                                    position: 1..16,
                                    derived: false,
                                    quantities: [
                                        Number(
                                            "1",
                                        ),
                                    ],
                                    unit: Some(
                                        "cup",
                                    ),
                                    text: "sugar",
                                },
                                text: "pulverize",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}
#[test]
fn binary() {
    test_parse(
        "<*1 cup: sugar *3 cups: milk #boil and stir>",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..44,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: BinaryOp {
                                position: 29..44,
                                first: Ingredient {
                                    position: 1..16,
                                    derived: false,
                                    quantities: [
                                        Number(
                                            "1",
                                        ),
                                    ],
                                    unit: Some(
                                        "cup",
                                    ),
                                    text: "sugar",
                                },
                                second: Ingredient {
                                    position: 15..30,
                                    derived: false,
                                    quantities: [
                                        Number(
                                            "3",
                                        ),
                                    ],
                                    unit: Some(
                                        "cups",
                                    ),
                                    text: "milk",
                                },
                                text: "boil and stir",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}
#[test]
fn title() {
    test_parse(
        "<**Sugar *1 cup: sugar>",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..23,
                            title: Some(
                                "Sugar",
                            ),
                            preamble: None,
                            comment: None,
                            root: Ingredient {
                                position: 9..23,
                                derived: false,
                                quantities: [
                                    Number(
                                        "1",
                                    ),
                                ],
                                unit: Some(
                                    "cup",
                                ),
                                text: "sugar",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}
#[test]
fn preamble() {
    test_parse(
        "< ##preheat oven *1 cup: sugar>",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..31,
                            title: None,
                            preamble: Some(
                                "preheat oven",
                            ),
                            comment: None,
                            root: Ingredient {
                                position: 17..31,
                                derived: false,
                                quantities: [
                                    Number(
                                        "1",
                                    ),
                                ],
                                unit: Some(
                                    "cup",
                                ),
                                text: "sugar",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}
#[test]
fn title_preamble() {
    test_parse(
        "<**Sugar ##preheat oven *1 cup: sugar>",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..38,
                            title: Some(
                                "Sugar",
                            ),
                            preamble: Some(
                                "preheat oven",
                            ),
                            comment: None,
                            root: Ingredient {
                                position: 24..38,
                                derived: false,
                                quantities: [
                                    Number(
                                        "1",
                                    ),
                                ],
                                unit: Some(
                                    "cup",
                                ),
                                text: "sugar",
                            },
                        },
                    ],
                },
                [],
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
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..177,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: BinaryOp {
                                position: 153..177,
                                first: BinaryOp {
                                    position: 124..131,
                                    first: BinaryOp {
                                        position: 90..110,
                                        first: UnaryOp {
                                            position: 66..74,
                                            operand: BinaryOp {
                                                position: 50..67,
                                                first: UnaryOp {
                                                    position: 17..24,
                                                    operand: Ingredient {
                                                        position: 2..18,
                                                        derived: false,
                                                        quantities: [
                                                            Number(
                                                                "6",
                                                            ),
                                                        ],
                                                        unit: Some(
                                                            "cups",
                                                        ),
                                                        text: "water",
                                                    },
                                                    text: "boil",
                                                },
                                                second: Ingredient {
                                                    position: 23..51,
                                                    derived: false,
                                                    quantities: [
                                                        Number(
                                                            "2",
                                                        ),
                                                    ],
                                                    unit: Some(
                                                        "cups",
                                                    ),
                                                    text: "macarroni noodles",
                                                },
                                                text: "boil till soft",
                                            },
                                            text: "drain",
                                        },
                                        second: Ingredient {
                                            position: 73..91,
                                            derived: false,
                                            quantities: [
                                                Fraction(
                                                    "1/4",
                                                ),
                                            ],
                                            unit: Some(
                                                "cup",
                                            ),
                                            text: "butter",
                                        },
                                        text: "stir until melted",
                                    },
                                    second: Ingredient {
                                        position: 109..125,
                                        derived: false,
                                        quantities: [
                                            Fraction(
                                                "1/3",
                                            ),
                                        ],
                                        unit: Some(
                                            "cup",
                                        ),
                                        text: "milk",
                                    },
                                    text: "stir",
                                },
                                second: Ingredient {
                                    position: 130..154,
                                    derived: false,
                                    quantities: [
                                        Number(
                                            "1",
                                        ),
                                    ],
                                    unit: Some(
                                        "pouch",
                                    ),
                                    text: "dried cheese",
                                },
                                text: "stir until well mixed",
                            },
                        },
                    ],
                },
                [],
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
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..224,
                            title: None,
                            preamble: None,
                            comment: Some(
                                "Yield 1 dozen cookies",
                            ),
                            root: UnaryOp {
                                position: 183..199,
                                operand: UnaryOp {
                                    position: 166..184,
                                    operand: BinaryOp {
                                        position: 160..167,
                                        first: BinaryOp {
                                            position: 113..127,
                                            first: BinaryOp {
                                                position: 65..87,
                                                first: BinaryOp {
                                                    position: 53..60,
                                                    first: UnaryOp {
                                                        position: 10..19,
                                                        operand: Ingredient {
                                                            position: 2..11,
                                                            derived: false,
                                                            quantities: [],
                                                            unit: None,
                                                            text: "butter",
                                                        },
                                                        text: "soften",
                                                    },
                                                    second: BinaryOp {
                                                        position: 50..54,
                                                        first: BinaryOp {
                                                            position: 38..42,
                                                            first: Ingredient {
                                                                position: 18..26,
                                                                derived: false,
                                                                quantities: [],
                                                                unit: None,
                                                                text: "sugar",
                                                            },
                                                            second: Ingredient {
                                                                position: 25..39,
                                                                derived: false,
                                                                quantities: [],
                                                                unit: None,
                                                                text: "brown sugar",
                                                            },
                                                            text: "+",
                                                        },
                                                        second: Ingredient {
                                                            position: 41..51,
                                                            derived: false,
                                                            quantities: [],
                                                            unit: None,
                                                            text: "vanilla",
                                                        },
                                                        text: "+",
                                                    },
                                                    text: "beat",
                                                },
                                                second: Ingredient {
                                                    position: 59..66,
                                                    derived: false,
                                                    quantities: [],
                                                    unit: None,
                                                    text: "eggs",
                                                },
                                                text: "beat one at a time",
                                            },
                                            second: BinaryOp {
                                                position: 108..114,
                                                first: BinaryOp {
                                                    position: 99..103,
                                                    first: Ingredient {
                                                        position: 86..94,
                                                        derived: false,
                                                        quantities: [],
                                                        unit: None,
                                                        text: "flour",
                                                    },
                                                    second: Ingredient {
                                                        position: 93..100,
                                                        derived: false,
                                                        quantities: [],
                                                        unit: None,
                                                        text: "soda",
                                                    },
                                                    text: "+",
                                                },
                                                second: Ingredient {
                                                    position: 102..109,
                                                    derived: false,
                                                    quantities: [],
                                                    unit: None,
                                                    text: "salt",
                                                },
                                                text: "mix",
                                            },
                                            text: "beat slowly",
                                        },
                                        second: BinaryOp {
                                            position: 157..161,
                                            first: Ingredient {
                                                position: 126..144,
                                                derived: false,
                                                quantities: [],
                                                unit: None,
                                                text: "chocolate chips",
                                            },
                                            second: Ingredient {
                                                position: 143..158,
                                                derived: false,
                                                quantities: [],
                                                unit: None,
                                                text: "chopped nuts",
                                            },
                                            text: "+",
                                        },
                                        text: "stir",
                                    },
                                    text: "form into balls",
                                },
                                text: "bake 375F 10m",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}

#[test]
fn souffle() {
    test_parse(
        r#"<** Souffle pancake with one egg
*3 or 4 drops: lemon juice
*1: egg =separate keep white #stir in =beat at medium speed, until foamy
*1 1/2 tbsp: sugar #sprinkle in =beat at medium speed 3m until firm peaks form
*^egg yolk
*2 tbsp: flour #+
*1 tbsp: milk #mix to combine
*1/2 tsp: vanilla #stir in
*^1/3 of: egg white mixture #mix with circular motion #fold in with flat spatula
*1 tsp: oil =heat in pan 1m
*^2/3 of: pancake mixture #scoop into pan as two pancakes
*2 tsp: water #add to sides of pan =cover cook 2m on medium heat
*1 tsp: water #add to sides of pan #place on top
    =cover cook 5m on medium low heat =flip =cover cook 5m
    =serve with fruit and syrup/powdered sugar
#*Makes 2 pancakes
>"#,
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..705,
                            title: Some(
                                "Souffle pancake with one egg",
                            ),
                            preamble: None,
                            comment: Some(
                                "Makes 2 pancakes",
                            ),
                            root: UnaryOp {
                                position: 642..686,
                                operand: UnaryOp {
                                    position: 623..643,
                                    operand: UnaryOp {
                                        position: 617..624,
                                        operand: UnaryOp {
                                            position: 583..618,
                                            operand: BinaryOp {
                                                position: 565..584,
                                                first: BinaryOp {
                                                    position: 352..380,
                                                    first: UnaryOp {
                                                        position: 165..213,
                                                        operand: BinaryOp {
                                                            position: 152..166,
                                                            first: UnaryOp {
                                                                position: 98..134,
                                                                operand: BinaryOp {
                                                                    position: 89..99,
                                                                    first: Ingredient {
                                                                        position: 33..61,
                                                                        derived: false,
                                                                        quantities: [
                                                                            Number(
                                                                                "3",
                                                                            ),
                                                                        ],
                                                                        unit: Some(
                                                                            "or 4 drops",
                                                                        ),
                                                                        text: "lemon juice",
                                                                    },
                                                                    second: UnaryOp {
                                                                        position: 68..90,
                                                                        operand: Ingredient {
                                                                            position: 60..69,
                                                                            derived: false,
                                                                            quantities: [
                                                                                Number(
                                                                                    "1",
                                                                                ),
                                                                            ],
                                                                            unit: None,
                                                                            text: "egg",
                                                                        },
                                                                        text: "separate keep white",
                                                                    },
                                                                    text: "stir in",
                                                                },
                                                                text: "beat at medium speed, until foamy",
                                                            },
                                                            second: Ingredient {
                                                                position: 133..153,
                                                                derived: false,
                                                                quantities: [
                                                                    Number(
                                                                        "1",
                                                                    ),
                                                                    Fraction(
                                                                        "1/2",
                                                                    ),
                                                                ],
                                                                unit: Some(
                                                                    "tbsp",
                                                                ),
                                                                text: "sugar",
                                                            },
                                                            text: "sprinkle in",
                                                        },
                                                        text: "beat at medium speed 3m until firm peaks form",
                                                    },
                                                    second: BinaryOp {
                                                        position: 326..353,
                                                        first: BinaryOp {
                                                            position: 289..299,
                                                            first: BinaryOp {
                                                                position: 255..272,
                                                                first: BinaryOp {
                                                                    position: 238..242,
                                                                    first: Ingredient {
                                                                        position: 212..224,
                                                                        derived: true,
                                                                        quantities: [],
                                                                        unit: None,
                                                                        text: "egg yolk",
                                                                    },
                                                                    second: Ingredient {
                                                                        position: 223..239,
                                                                        derived: false,
                                                                        quantities: [
                                                                            Number(
                                                                                "2",
                                                                            ),
                                                                        ],
                                                                        unit: Some(
                                                                            "tbsp",
                                                                        ),
                                                                        text: "flour",
                                                                    },
                                                                    text: "+",
                                                                },
                                                                second: Ingredient {
                                                                    position: 241..256,
                                                                    derived: false,
                                                                    quantities: [
                                                                        Number(
                                                                            "1",
                                                                        ),
                                                                    ],
                                                                    unit: Some(
                                                                        "tbsp",
                                                                    ),
                                                                    text: "milk",
                                                                },
                                                                text: "mix to combine",
                                                            },
                                                            second: Ingredient {
                                                                position: 271..290,
                                                                derived: false,
                                                                quantities: [
                                                                    Fraction(
                                                                        "1/2",
                                                                    ),
                                                                ],
                                                                unit: Some(
                                                                    "tsp",
                                                                ),
                                                                text: "vanilla",
                                                            },
                                                            text: "stir in",
                                                        },
                                                        second: Ingredient {
                                                            position: 298..327,
                                                            derived: true,
                                                            quantities: [
                                                                Fraction(
                                                                    "1/3",
                                                                ),
                                                            ],
                                                            unit: Some(
                                                                "of",
                                                            ),
                                                            text: "egg white mixture",
                                                        },
                                                        text: "mix with circular motion",
                                                    },
                                                    text: "fold in with flat spatula",
                                                },
                                                second: BinaryOp {
                                                    position: 544..566,
                                                    first: UnaryOp {
                                                        position: 500..531,
                                                        operand: BinaryOp {
                                                            position: 479..501,
                                                            first: BinaryOp {
                                                                position: 433..466,
                                                                first: UnaryOp {
                                                                    position: 391..408,
                                                                    operand: Ingredient {
                                                                        position: 379..392,
                                                                        derived: false,
                                                                        quantities: [
                                                                            Number(
                                                                                "1",
                                                                            ),
                                                                        ],
                                                                        unit: Some(
                                                                            "tsp",
                                                                        ),
                                                                        text: "oil",
                                                                    },
                                                                    text: "heat in pan 1m",
                                                                },
                                                                second: Ingredient {
                                                                    position: 407..434,
                                                                    derived: true,
                                                                    quantities: [
                                                                        Fraction(
                                                                            "2/3",
                                                                        ),
                                                                    ],
                                                                    unit: Some(
                                                                        "of",
                                                                    ),
                                                                    text: "pancake mixture",
                                                                },
                                                                text: "scoop into pan as two pancakes",
                                                            },
                                                            second: Ingredient {
                                                                position: 465..480,
                                                                derived: false,
                                                                quantities: [
                                                                    Number(
                                                                        "2",
                                                                    ),
                                                                ],
                                                                unit: Some(
                                                                    "tsp",
                                                                ),
                                                                text: "water",
                                                            },
                                                            text: "add to sides of pan",
                                                        },
                                                        text: "cover cook 2m on medium heat",
                                                    },
                                                    second: Ingredient {
                                                        position: 530..545,
                                                        derived: false,
                                                        quantities: [
                                                            Number(
                                                                "1",
                                                            ),
                                                        ],
                                                        unit: Some(
                                                            "tsp",
                                                        ),
                                                        text: "water",
                                                    },
                                                    text: "add to sides of pan",
                                                },
                                                text: "place on top",
                                            },
                                            text: "cover cook 5m on medium low heat",
                                        },
                                        text: "flip",
                                    },
                                    text: "cover cook 5m",
                                },
                                text: "serve with fruit and syrup/powdered sugar",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    )
}

#[test]
fn keiserschmarrn() {
    test_parse(
        r#"<**Keiserschmarrn
            ## Good Saturday breakfast
*6 : eggs =separate
*1 1/2 cups: milk #+
*1 tsp: vanilla #whisk
*salt #+
*1 cup: flour #whisk
*3 tbsp: sugar
*^egg whites #whip to form soft peaks #gently fold until white lumps are gone
*2 tbsp: butter =melt in large pan #in two batches cook until bottom side is firm flip and break into bit size pieces
*powdered sugar #+
*syrup #top and serve
#* Serves 4
>"#,
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..416,
                            title: Some(
                                "Keiserschmarrn",
                            ),
                            preamble: Some(
                                "Good Saturday breakfast",
                            ),
                            comment: Some(
                                "Serves 4",
                            ),
                            root: BinaryOp {
                                position: 388..404,
                                first: BinaryOp {
                                    position: 378..382,
                                    first: BinaryOp {
                                        position: 279..363,
                                        first: BinaryOp {
                                            position: 204..245,
                                            first: BinaryOp {
                                                position: 144..152,
                                                first: BinaryOp {
                                                    position: 127..131,
                                                    first: BinaryOp {
                                                        position: 114..122,
                                                        first: BinaryOp {
                                                            position: 95..99,
                                                            first: UnaryOp {
                                                                position: 67..78,
                                                                operand: Ingredient {
                                                                    position: 57..68,
                                                                    derived: false,
                                                                    quantities: [
                                                                        Number(
                                                                            "6",
                                                                        ),
                                                                    ],
                                                                    unit: None,
                                                                    text: "eggs",
                                                                },
                                                                text: "separate",
                                                            },
                                                            second: Ingredient {
                                                                position: 77..96,
                                                                derived: false,
                                                                quantities: [
                                                                    Number(
                                                                        "1",
                                                                    ),
                                                                    Fraction(
                                                                        "1/2",
                                                                    ),
                                                                ],
                                                                unit: Some(
                                                                    "cups",
                                                                ),
                                                                text: "milk",
                                                            },
                                                            text: "+",
                                                        },
                                                        second: Ingredient {
                                                            position: 98..115,
                                                            derived: false,
                                                            quantities: [
                                                                Number(
                                                                    "1",
                                                                ),
                                                            ],
                                                            unit: Some(
                                                                "tsp",
                                                            ),
                                                            text: "vanilla",
                                                        },
                                                        text: "whisk",
                                                    },
                                                    second: Ingredient {
                                                        position: 121..128,
                                                        derived: false,
                                                        quantities: [],
                                                        unit: None,
                                                        text: "salt",
                                                    },
                                                    text: "+",
                                                },
                                                second: Ingredient {
                                                    position: 130..145,
                                                    derived: false,
                                                    quantities: [
                                                        Number(
                                                            "1",
                                                        ),
                                                    ],
                                                    unit: Some(
                                                        "cup",
                                                    ),
                                                    text: "flour",
                                                },
                                                text: "whisk",
                                            },
                                            second: BinaryOp {
                                                position: 179..205,
                                                first: Ingredient {
                                                    position: 151..167,
                                                    derived: false,
                                                    quantities: [
                                                        Number(
                                                            "3",
                                                        ),
                                                    ],
                                                    unit: Some(
                                                        "tbsp",
                                                    ),
                                                    text: "sugar",
                                                },
                                                second: Ingredient {
                                                    position: 166..180,
                                                    derived: true,
                                                    quantities: [],
                                                    unit: None,
                                                    text: "egg whites",
                                                },
                                                text: "whip to form soft peaks",
                                            },
                                            text: "gently fold until white lumps are gone",
                                        },
                                        second: UnaryOp {
                                            position: 260..280,
                                            operand: Ingredient {
                                                position: 244..261,
                                                derived: false,
                                                quantities: [
                                                    Number(
                                                        "2",
                                                    ),
                                                ],
                                                unit: Some(
                                                    "tbsp",
                                                ),
                                                text: "butter",
                                            },
                                            text: "melt in large pan",
                                        },
                                        text: "in two batches cook until bottom side is firm flip and break into bit size pieces",
                                    },
                                    second: Ingredient {
                                        position: 362..379,
                                        derived: false,
                                        quantities: [],
                                        unit: None,
                                        text: "powdered sugar",
                                    },
                                    text: "+",
                                },
                                second: Ingredient {
                                    position: 381..389,
                                    derived: false,
                                    quantities: [],
                                    unit: None,
                                    text: "syrup",
                                },
                                text: "top and serve",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    )
}

#[test]
fn invalid_token_error() {
    test_parse(
        "<#>",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..0,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: BinaryOp {
                                position: 1..3,
                                first: MissingOperand {
                                    position: 1..2,
                                },
                                second: MissingOperand {
                                    position: 1..2,
                                },
                                text: "",
                            },
                        },
                    ],
                },
                [
                    UnexpectedToken(
                        ">",
                        2..3,
                    ),
                    UnexpectedEOF,
                ],
            )
        "#]],
    );
}

#[test]
fn ghost_operands() {
    test_parse(
        "<
        *a #zero
        *b #one
        *c #two
        *d #three
        >",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..78,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: BinaryOp {
                                position: 62..78,
                                first: BinaryOp {
                                    position: 46..60,
                                    first: BinaryOp {
                                        position: 30..44,
                                        first: BinaryOp {
                                            position: 13..28,
                                            first: MissingOperand {
                                                position: 13..14,
                                            },
                                            second: Ingredient {
                                                position: 10..14,
                                                derived: false,
                                                quantities: [],
                                                unit: None,
                                                text: "a",
                                            },
                                            text: "zero",
                                        },
                                        second: Ingredient {
                                            position: 27..31,
                                            derived: false,
                                            quantities: [],
                                            unit: None,
                                            text: "b",
                                        },
                                        text: "one",
                                    },
                                    second: Ingredient {
                                        position: 43..47,
                                        derived: false,
                                        quantities: [],
                                        unit: None,
                                        text: "c",
                                    },
                                    text: "two",
                                },
                                second: Ingredient {
                                    position: 59..63,
                                    derived: false,
                                    quantities: [],
                                    unit: None,
                                    text: "d",
                                },
                                text: "three",
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}

#[test]
fn missing_operands() {
    test_parse(
        "<
        *a
        *b #one
        *c =two
        *d #three
        >",
        expect![[r#"
            (
                SourceFile {
                    recipes: [
                        Recipe {
                            position: 0..72,
                            title: None,
                            preamble: None,
                            comment: None,
                            root: UnusedOperands {
                                position: 56..72,
                                operands: [
                                    BinaryOp {
                                        position: 24..38,
                                        first: Ingredient {
                                            position: 10..22,
                                            derived: false,
                                            quantities: [],
                                            unit: None,
                                            text: "a",
                                        },
                                        second: Ingredient {
                                            position: 21..25,
                                            derived: false,
                                            quantities: [],
                                            unit: None,
                                            text: "b",
                                        },
                                        text: "one",
                                    },
                                    BinaryOp {
                                        position: 56..72,
                                        first: UnaryOp {
                                            position: 40..54,
                                            operand: Ingredient {
                                                position: 37..41,
                                                derived: false,
                                                quantities: [],
                                                unit: None,
                                                text: "c",
                                            },
                                            text: "two",
                                        },
                                        second: Ingredient {
                                            position: 53..57,
                                            derived: false,
                                            quantities: [],
                                            unit: None,
                                            text: "d",
                                        },
                                        text: "three",
                                    },
                                ],
                            },
                        },
                    ],
                },
                [],
            )
        "#]],
    );
}
