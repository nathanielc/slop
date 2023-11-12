use std::fs;

use expect_test::{expect_file, ExpectFile};
use slop::{compile, format, parse, to_svgs};

macro_rules! define_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            test(
                concat!("./tests/recipes/", stringify!($name), ".slop"),
                expect_file![concat!("recipes/expected/", stringify!($name), ".parse")],
                expect_file![concat!("recipes/expected/", stringify!($name), ".fmt")],
                expect_file![concat!(
                    "recipes/expected/",
                    stringify!($name),
                    ".fmt.errors"
                )],
                expect_file![concat!("recipes/expected/", stringify!($name), ".compile")],
                vec![expect_file![concat!(
                    "recipes/expected/",
                    stringify!($name),
                    ".svg"
                )]],
                expect_file![concat!(
                    "recipes/expected/",
                    stringify!($name),
                    ".svg.errors"
                )],
            )
        }
    };
}

fn test(
    fpath: &str,
    expect_parse: ExpectFile,
    expect_format: ExpectFile,
    expect_format_errors: ExpectFile,
    expect_compile: ExpectFile,
    expect_svgs: Vec<ExpectFile>,
    expect_svg_errors: ExpectFile,
) {
    let src = fs::read_to_string(fpath).unwrap();
    let actual_parse = parse(&src);
    expect_parse.assert_debug_eq(&actual_parse);

    let (actual_format, actual_format_errors) = format(&src);
    expect_format.assert_eq(&actual_format);
    expect_format_errors.assert_debug_eq(&actual_format_errors);

    let actual_compile = compile(&src);
    expect_compile.assert_debug_eq(&actual_compile);

    let (actual_svgs, actual_svg_errors) = to_svgs(&src);
    assert_eq!(actual_svgs.len(), expect_svgs.len());
    actual_svgs
        .iter()
        .zip(expect_svgs.iter())
        .for_each(|(actual, expected)| {
            expected.assert_eq(actual);
        });
    expect_svg_errors.assert_debug_eq(&actual_svg_errors);
}

define_test!(binary);
define_test!(binary_1);
define_test!(binary_long_lines);
define_test!(binary_plus);
define_test!(binary_plus_nested);
define_test!(cookies);
define_test!(hauloumi);
define_test!(ingredient);
define_test!(ingredient_derived);
define_test!(ingredient_derived_no_quantities);
define_test!(ingredient_fractional_quantity);
define_test!(keiserschmarrn);
define_test!(missing_operands);
define_test!(missing_operands_1);
define_test!(missing_operands_2);
define_test!(preamble);
define_test!(salted_butter);
define_test!(simple);
define_test!(souffle);
define_test!(title);
define_test!(title_preamble);
define_test!(unary);
define_test!(unary_1);
define_test!(unary_long_lines);
define_test!(unused_operands);

#[test]
fn carrot_pudding() {
    test(
        "./tests/recipes/carrot_pudding.slop",
        expect_file!["./recipes/expected/carrot_pudding.parse"],
        expect_file!["./recipes/expected/carrot_pudding.fmt"],
        expect_file!["./recipes/expected/carrot_pudding.fmt.errors"],
        expect_file!["./recipes/expected/carrot_pudding.compile"],
        vec![
            expect_file!["./recipes/expected/carrot_pudding.0.svg"],
            expect_file!["./recipes/expected/carrot_pudding.1.svg"],
            expect_file!["./recipes/expected/carrot_pudding.2.svg"],
        ],
        expect_file!["./recipes/expected/carrot_pudding.svg.errors"],
    )
}
