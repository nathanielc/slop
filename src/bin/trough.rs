// Initialize rocket crate
#[macro_use]
extern crate rocket;

use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rocket::http::ContentType;
use rocket::response::Redirect;
use rocket::{form::Form, fs::FileServer};
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use slop::{self, menu::compile_menu};
use slop::{menu::MealType, svg};
use slop::{menu::RecipeLink, semantic};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;
use url::form_urlencoded;

#[get("/recipe/card/<name..>")]
fn recipe_card(name: PathBuf) -> (ContentType, Vec<u8>) {
    let mut filepath = Path::new("recipes/").join(name);
    filepath.set_extension("slop");
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let src_ast = slop::parse(&contents).expect("parsing failed");
    let src_sem = semantic::convert_source_file(src_ast);
    (ContentType::SVG, svg::to_svg(&src_sem))
}

#[derive(serde::Serialize)]
struct IndexTemplateContext {
    items: Vec<IndexItem>,
}

#[derive(serde::Serialize)]
struct IndexItem {
    is_dir: bool,
    name: String,
    link: String,
    slop_path: String,
}

#[derive(serde::Serialize)]
struct IngredientTemplateContext {
    ingredients: Vec<String>,
}

#[derive(serde::Serialize)]
struct PreviewMenuTemplateContext {
    recipes: Vec<(String, PathBuf, PathBuf)>,
    ingredients: Vec<(String, String)>,
}
#[derive(serde::Serialize)]
struct CreateMenuTemplateContext {
    types: Vec<(String, String)>,
}

#[derive(FromForm, Debug)]
struct GenerateMenuData {
    breakfast: usize,
    lunch: usize,
    dinner: usize,
    snack: usize,
    dessert: usize,
}
#[derive(FromForm, PartialEq, Debug)]
struct PreviewMenuData {
    recipe: Vec<String>,
}

#[derive(FromForm, PartialEq, Debug, Deserialize, Serialize)]
struct SaveMenuData {
    recipe: Vec<String>,
}

#[get("/ingredients")]
fn ingredients() -> Template {
    _ingredients()
}

#[get("/menu")]
fn menu() -> std::result::Result<Redirect, String> {
    match _menu() {
        Ok(r) => Ok(r),
        Err(err) => Err(err.to_string()),
    }
}

#[get("/create_menu")]
fn create_menu() -> Template {
    _create_menu()
}
#[get("/generate_menu?<data..>")]
fn generate_menu(data: GenerateMenuData) -> Redirect {
    _generate_menu(data)
}
#[get("/preview_menu?<recipe..>")]
fn preview_menu(recipe: PreviewMenuData) -> Template {
    _preview_menu(recipe)
}
#[post("/save_menu", data = "<data>")]
fn save_menu(data: Form<SaveMenuData>) -> std::result::Result<Redirect, String> {
    match _save_menu(data.into_inner()) {
        Ok(r) => Ok(r),
        Err(err) => Err(err.to_string()),
    }
}
#[get("/recipes/<name..>")]
fn recipes_index(name: PathBuf) -> Template {
    _recipes_index(name)
}
#[get("/recipes")]
fn recipes_root_index() -> Template {
    _recipes_index(PathBuf::new())
}
#[get("/")]
fn recipes_root_index_redirect() -> Redirect {
    Redirect::to(uri!(recipes_root_index()))
}
fn _recipes_index(name: PathBuf) -> Template {
    let mut items: Vec<IndexItem> = Vec::new();
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
            items.push(IndexItem {
                is_dir: true,
                name,
                link: Path::new("/recipes")
                    .join(path)
                    .to_str()
                    .unwrap()
                    .to_string(),
                slop_path: "".to_string(),
            });
        } else if pb.extension().unwrap() == "slop" {
            let mut l = PathBuf::from("/recipe/card").join(pb.strip_prefix("recipes/").unwrap());
            l.set_extension("");
            let name = l.file_name().unwrap().to_str().unwrap().to_string();
            if name.starts_with(".") {
                continue;
            }
            items.push(IndexItem {
                is_dir: false,
                name,
                link: l.to_str().unwrap().to_string(),
                slop_path: pb.to_str().unwrap().to_string(),
            });
        }
    }
    items.sort_by(|a, b| a.name.cmp(&b.name));
    let context = IndexTemplateContext { items };
    Template::render("recipe-index", &context)
}

const RECIPE_ROOT: &'static str = "recipes";
const MENU_ROOT: &'static str = "menus";

fn _ingredients() -> Template {
    let recipes = list_all_recipes(&PathBuf::from(RECIPE_ROOT)).expect("list all recipes");
    let menu = compile_menu(recipes).expect("compile menu");

    let mut ingredients = menu
        .ingredients
        .iter()
        .map(|(name, _amounts)| (name.to_owned()))
        .collect::<Vec<String>>();
    ingredients.sort();
    let context = IngredientTemplateContext { ingredients };

    Template::render("ingredients", &context)
}

fn _menu() -> Result<Redirect> {
    let data = fs::read_to_string(PathBuf::from(MENU_ROOT).join("menu.json"))?;
    let menu: SaveMenuData = serde_json::from_str(&data)?;

    let mut serializer = form_urlencoded::Serializer::new(String::new());
    for r in menu.recipe {
        serializer.append_pair("recipe", &r);
    }
    Ok(Redirect::to(format!(
        "/preview_menu?{}",
        serializer.finish()
    )))
}

fn capitalize(s: String) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().nth(0).unwrap()
            } else {
                c
            }
        })
        .collect::<String>()
}
fn _create_menu() -> Template {
    let context = CreateMenuTemplateContext {
        types: MealType::iter()
            .map(|t| (t.name().to_owned(), capitalize(t.name().to_owned())))
            .collect::<Vec<(String, String)>>(),
    };
    Template::render("create_menu", &context)
}
fn _generate_menu(data: GenerateMenuData) -> Redirect {
    let root = PathBuf::from(RECIPE_ROOT);
    let mut recipes = Vec::with_capacity(data.breakfast + data.lunch + data.dinner);
    recipes.extend(pick_random(
        list_recipes(&root, MealType::Breakfast).expect("failed to read breakfasts"),
        data.breakfast,
    ));
    recipes.extend(pick_random(
        list_recipes(&root, MealType::Lunch).expect("failed to read lunches"),
        data.lunch,
    ));
    recipes.extend(pick_random(
        list_recipes(&root, MealType::Dinner).expect("failed to read dinners"),
        data.dinner,
    ));
    recipes.extend(pick_random(
        list_recipes(&root, MealType::Snack).expect("failed to read snacks"),
        data.snack,
    ));
    recipes.extend(pick_random(
        list_recipes(&root, MealType::Dessert).expect("failed to read dessert"),
        data.dessert,
    ));

    let mut serializer = form_urlencoded::Serializer::new(String::new());
    for r in recipes {
        serializer.append_pair("recipe", r.slop_path.to_str().unwrap());
    }
    Redirect::to(format!("/preview_menu?{}", serializer.finish()))
}
fn _preview_menu(mut data: PreviewMenuData) -> Template {
    let recipes = data
        .recipe
        .drain(..)
        .map(|slop_path| parse_slop_path(PathBuf::from(slop_path)))
        .collect::<Result<Vec<RecipeLink>>>()
        .unwrap();

    let menu = compile_menu(recipes).expect("compile menu");

    let mut context = PreviewMenuTemplateContext {
        recipes: menu
            .recipes
            .iter()
            .map(|recipe| {
                (
                    recipe.title().to_owned(),
                    recipe.link.card_url.to_owned(),
                    recipe.link.slop_path.to_owned(),
                )
            })
            .collect(),
        ingredients: menu
            .ingredients
            .iter()
            .map(|(name, amounts)| (name.to_owned(), format!("{}", amounts)))
            .collect(),
    };
    context.ingredients.sort_unstable_by_key(|i| i.0.clone());
    Template::render("preview_menu", &context)
}

fn _save_menu(data: SaveMenuData) -> Result<Redirect> {
    fs::create_dir_all(MENU_ROOT)?;
    let f = File::create(PathBuf::from(MENU_ROOT).join("menu.json"))?;
    serde_json::to_writer(&f, &data)?;

    Ok(Redirect::to(uri!(menu)))
}

const CARD_PATH: &'static str = "recipe/card";

fn list_all_recipes(root: &Path) -> Result<Vec<RecipeLink>> {
    let mut recipes = Vec::new();
    for typ in MealType::iter() {
        recipes.extend(list_recipes(root, typ)?);
    }
    Ok(recipes)
}
fn list_recipes(root: &Path, typ: MealType) -> Result<Vec<RecipeLink>> {
    let dir = root.join(typ.clone().name());
    let mut recipes: Vec<RecipeLink> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(ext) = file_path.extension() {
                    if ext == "slop" {
                        let card_url = PathBuf::from(CARD_PATH)
                            .join(file_path.strip_prefix(root).unwrap())
                            .with_extension("");
                        recipes.push(RecipeLink {
                            typ: typ.clone(),
                            slop_path: file_path,
                            card_url,
                        });
                    }
                }
            }
        }
    }
    Ok(recipes)
}
fn parse_slop_path(path: PathBuf) -> anyhow::Result<RecipeLink> {
    let p = path.strip_prefix(RECIPE_ROOT)?.to_owned();
    let mut mt: MealType = MealType::Breakfast;
    for typ in MealType::iter() {
        if p.starts_with(typ.name()) {
            mt = typ;
            break;
        }
    }
    Ok(RecipeLink {
        typ: mt,
        slop_path: path,
        card_url: PathBuf::from(CARD_PATH).join(p).with_extension(""),
    })
}
fn pick_random<T>(mut list: Vec<T>, count: usize) -> Vec<T> {
    let mut rng = thread_rng();
    list.shuffle(&mut rng);
    list.truncate(count);
    list
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                recipes_index,
                recipes_root_index,
                recipes_root_index_redirect,
                recipe_card,
                menu,
                create_menu,
                generate_menu,
                preview_menu,
                save_menu,
                ingredients,
            ],
        )
        .mount("/static", FileServer::from("./static"))
}
