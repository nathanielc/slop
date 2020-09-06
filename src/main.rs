
mod ast;
use ast::Operand;

fn main() {
    println!("Hello, world!");
}

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub rcp); // synthesized by LALRPOP

#[cfg(test)]
mod test{
    use super::*;
    fn test_parse(src:&str, ast:ast::Operand) {
        assert_eq!(
            ast,
            rcp::RecipeParser::new().parse(src).unwrap(),
        )
    }
    #[test]
    fn ingredient() {
        test_parse(
            "<*sugar 1 cup>",
            Operand::Ingredient("sugar 1 cup".to_owned()),
        );
    }
    #[test]
    fn unary() {
        test_parse(
            "<*sugar 1 cup =pulverize>",
            Operand::UnaryOp(
                Box::new(Operand::Ingredient("sugar 1 cup".to_owned())),
                "pulverize".to_string(),
            ),
        );
    }
    #[test]
    fn binary() {
        test_parse(
            "<*sugar 1 cup *milk 3 cups #boil and stir>",
            Operand::BinaryOp(
                Box::new(Operand::Ingredient("sugar 1 cup".to_owned())),
                Box::new(Operand::Ingredient("milk 3 cups".to_owned())),
                "boil and stir".to_string(),
            ),
        );
    }
    #[test]
    fn simple() {
        test_parse(
            "<
*water 6 cups =boil
*macarroni noodles 2 cups #add =drain
*butter 1/4 cup #stir until melted
*milk 1/3 cup #stir
*dried cheese one pouch #stir until well mixed
>",

            Operand::BinaryOp(
                Box::new(Operand::BinaryOp(
                        Box::new(Operand::BinaryOp(
                               Box::new( Operand::UnaryOp(
                                    Box::new( Operand::BinaryOp(
                                             Box::new( Operand::UnaryOp( Operand::Ingredient("water 6 cups"), "boil")),
                                             Ingredient("macarroni noodles 2 cups"),
                                             "add")),
                                    "drain")),
                                Operand::Ingredient("butter 1/4 cup"), "stir until melted")), Operand::Ingredient("milk 1/3 cup"), "stir")), Operand::Ingredient("dried cheese one pouch"), "stir until well mixed"),
        );
    }
}
