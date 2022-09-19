use std::path::PathBuf;
use std::{collections::HashMap, fs};

use strum_macros::EnumIter;

use anyhow::Result;

use crate::{
    parse,
    quant::{compute_amounts, Amounts},
    semantic::{self, Operand, Recipe},
};

pub struct Menu {
    pub recipes: Vec<RecipeMeta>,
    pub ingredients: Vec<(String, Amounts)>,
}
pub struct RecipeLink {
    pub typ: MealType,
    pub slop_path: PathBuf,
    pub card_url: PathBuf,
}
impl RecipeLink {
    pub fn file_name(&self) -> &str {
        self.slop_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".slop")
            .unwrap()
    }
}
#[derive(PartialEq, Eq, Hash, Debug, Clone, EnumIter)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    Dessert,
}

impl MealType {
    pub fn name(&self) -> &'static str {
        match self {
            MealType::Breakfast => "breakfast",
            MealType::Lunch => "lunch",
            MealType::Dinner => "dinner",
            MealType::Snack => "snack",
            MealType::Dessert => "dessert",
        }
    }
}

pub struct RecipeMeta {
    recipe: Recipe,
    pub link: RecipeLink,
}

impl RecipeMeta {
    pub fn title(&self) -> &str {
        match &self.recipe.title {
            Some(t) => t.as_str(),
            None => self.link.file_name(),
        }
    }
}

pub fn compile_menu(mut input_recipes: Vec<RecipeLink>) -> Result<Menu> {
    let recipes: Result<Vec<RecipeMeta>> = input_recipes
        .drain(..)
        .map(|link| -> Result<RecipeMeta> {
            let contents = fs::read_to_string(&link.slop_path)?;
            let src_ast = parse(&contents)?;
            let mut src_sem = semantic::convert_source_file(src_ast);
            Ok(RecipeMeta {
                recipe: src_sem.recipes.pop().unwrap(),
                link,
            })
        })
        .collect();
    let recipes = recipes?;

    let ingredients = aggregate_ingredients(&recipes);

    Ok(Menu {
        recipes,
        ingredients,
    })
}
fn aggregate_ingredients(recipes: &Vec<RecipeMeta>) -> Vec<(String, Amounts)> {
    let mut ingredients: HashMap<String, Amounts> = HashMap::new();
    for r in recipes {
        let ings = find_ingredients(&r.recipe.root);
        for (name, amounts) in ings {
            if let Some(existing_amounts) = ingredients.get_mut(&name) {
                existing_amounts.update(amounts);
            } else {
                ingredients.insert(name, amounts);
            }
        }
    }

    ingredients
        .drain()
        .map(|(name, amounts)| (name, amounts))
        .collect()
}

fn find_ingredients(op: &Operand) -> Vec<(String, Amounts)> {
    match op {
        Operand::Ingredient(ing) => {
            if ing.derived {
                // skip derived ingredients
                return vec![];
            }
            let amounts = compute_amounts(&ing.quantity, &ing.unit);
            vec![(ing.name.to_owned(), amounts)]
        }
        Operand::Operator {
            text: _text,
            operands,
        } => operands
            .iter()
            .flat_map(|op| find_ingredients(op))
            .collect(),
    }
}
