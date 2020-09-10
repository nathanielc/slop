#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(rp);

mod ast;
#[cfg(test)]
mod rp_test;
mod svg;

fn main() {
    let recipe = "<
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
>";
    let ast = rp::RecipeParser::new().parse(recipe).unwrap();
    svg::to_svg(ast);
}
