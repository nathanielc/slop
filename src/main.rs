// Initialize rocket crate
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

// Bring in generated parser for rp
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(rp);

use std::fs;
use std::path::{Path,PathBuf};
use rocket::response::content::Content;
use rocket::http::ContentType;

// Local modules
mod ast;
mod semantic;
mod svg;
#[cfg(test)]
mod rp_test;


#[get("/recipe/card/<name..>")]
fn recipe_card(name: PathBuf) -> Content<Vec<u8>> {
    let mut filepath = Path::new("recipes/").join(name);
        filepath.set_extension("rp");
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let ast = rp::RecipeParser::new().parse(&contents).unwrap();
    let sem = semantic::convert_graph(ast);
    Content(ContentType::SVG,svg::to_svg(sem))
}

fn main() {
    rocket::ignite().mount("/", routes![recipe_card]).launch();
}
