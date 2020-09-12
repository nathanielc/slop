// Initialize rocket crate
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket_contrib::templates::Template;

// Bring in generated parser for rp
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(rp);

use rocket::http::ContentType;
use rocket::response::content::Content;
use std::fs;
use std::path::{Path, PathBuf};

// Local modules
mod ast;
#[cfg(test)]
mod rp_test;
mod semantic;
mod svg;

#[get("/recipe/card/<name..>")]
fn recipe_card(name: PathBuf) -> Content<Vec<u8>> {
    let mut filepath = Path::new("recipes/").join(name);
    filepath.set_extension("rp");
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let recipe_ast = rp::RecipeParser::new().parse(&contents).unwrap();
    let recipe_sem = semantic::convert_recipe(recipe_ast);
    Content(ContentType::SVG, svg::to_svg(recipe_sem))
}

#[derive(serde::Serialize)]
struct IndexTemplateContext {
    items: Vec<(String, String)>,
}
#[get("/recipes/<name..>")]
fn recipes_index(name: PathBuf) -> Template {
    _recipes_index(name)
}
#[get("/recipes")]
fn recipes_root_index() -> Template {
    _recipes_index(PathBuf::new())
}
fn _recipes_index(name: PathBuf) -> Template {
    let mut items: Vec<(String, String)> = Vec::new();
    let dir = Path::new("recipes/").join(name);
    for entry in fs::read_dir(&dir).expect("failed to read recipe directory") {
        let entry = entry.expect("valid entry");
        let pb = entry.path();
        let path = pb.strip_prefix(&dir).unwrap();
        if pb.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            if name.starts_with(".") {
                continue;
            }
            items.push((name, path.to_str().unwrap().to_string()));
        } else if pb.extension().unwrap() == "rp" {
            let mut l = PathBuf::from("../recipe/card").join(pb.strip_prefix("recipes/").unwrap());
            l.set_extension("");
            let name = l.file_name().unwrap().to_str().unwrap().to_string();
            if name.starts_with(".") {
                continue;
            }
            items.push((name, l.to_str().unwrap().to_string()));
        }
    }
    let context = IndexTemplateContext { items: items };
    Template::render("recipe-index", &context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![recipes_index, recipes_root_index, recipe_card])
        .launch();
}
