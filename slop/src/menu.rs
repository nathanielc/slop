use std::collections::HashMap;

use crate::{
    quant::{compute_amounts, Amounts},
    semantic::{Operand, Recipe},
};

pub fn aggregate_ingredients(recipes: impl Iterator<Item = Recipe>) -> Vec<(String, Amounts)> {
    let mut ingredients: HashMap<String, Amounts> = HashMap::new();
    for r in recipes {
        let ings = find_ingredients(&r.root);
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
            let amounts = compute_amounts(&ing.quantities, &ing.unit);
            vec![(ing.text.to_owned(), amounts)]
        }
        Operand::Operator { operands, .. } => operands.iter().flat_map(find_ingredients).collect(),
        Operand::MissingOperand { position } => todo!(),
        Operand::UnusedOperands { position, operands } => todo!(),
    }
}
