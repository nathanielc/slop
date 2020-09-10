use super::*;
use ast::Operand;

fn test_parse(src: &str, ast: ast::Operand) {
    assert_eq!(ast, rp::RecipeParser::new().parse(src).unwrap(),)
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
fn simple_recipe() {
    test_parse(
        "<
*water 6 cups =boil
*macarroni noodles 2 cups #boil till soft =drain
*butter 1/4 cup #stir until melted
*milk 1/3 cup #stir
*dried cheese one pouch #stir until well mixed
>",
        Operand::BinaryOp(
            Box::new(Operand::BinaryOp(
                Box::new(Operand::BinaryOp(
                    Box::new(Operand::UnaryOp(
                        Box::new(Operand::BinaryOp(
                            Box::new(Operand::UnaryOp(
                                Box::new(Operand::Ingredient("water 6 cups".to_string())),
                                "boil".to_string(),
                            )),
                            Box::new(Operand::Ingredient("macarroni noodles 2 cups".to_string())),
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
    );
}

#[test]
fn full_recipe() {
    test_parse(
        "<
*10L milk =heat to 32C/90F
*nonchlorinated cool water
*1/8 (0.6ml) tspn calcium chloride #dilute #stir in
*1/8 tspn MA40001 mesophillic #sprinkle on top
*1/8 tspn M030 mesophillic #sprinkle on top =rehydrate for 5m =stir top to bottom =cover allow to ripen for 30m
*nonchlorinated cool water
*1 tspn single strength rennet #dilute #stir for no more than 1m =cover and set for 40m wait longer if needed to get clean break
    =cut curds into 1cm(1/2in) cubes =cover allow to heal for 5m =gently stir, cut any large curds that you see =stir for 40m increasing to 33C/91.4F
    =allow curds to settle for 5m =drain curds through cheese cloth lined collander for 5m keep the whey =remove from cheese cloth keep in collander for 5m
*whey =heat to 33C/91F #place colander over whey =cut curd into 5m/2in slabs and stack on top of each other =drain 10m =restack and drain for 10m
    =break into thumbnail sized pieces into pot
*2 tablespoons cheese salt #mill gently with clean hands =place into cheese mould =press at 5kg/11lbs for 10m =remove from press gently
*~2 tablespoons cheese salt #sprinkle on top and bottom =press at 5kg/11lbs for 10m
*~2 tablespoons cheese salt #sprinkle on top and bottom =press at 10kg/22lbs for 20m
*~2 tablespoons cheese salt #sprinkle on top and bottom =press at 10kg/22lbs for 16h
    =remove and trim off any excess
    =air dry for about 3d turning every ~6h or until touch dry
    =ripen at 13C/55F for 3w turning twice a week =brine wash once a week to prevent extra mold growth
>",
        Operand::Ingredient("asdf".to_string()),
    );
}
