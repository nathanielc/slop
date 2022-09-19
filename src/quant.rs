use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, Result};
use measurements::{Mass, Measurement, Volume};

#[derive(Debug, Clone)]
pub enum Amount {
    Volume(Volume),
    Mass(Mass),
    Arbitrary(Arbitrary),
}

// Map of unit to amount for a single ingredient
#[derive(Debug, Clone)]
pub struct Amounts(HashMap<String, Amount>);

#[derive(Debug, Clone)]
pub struct Arbitrary {
    pub units: String,
    pub value: f64,
}
impl Amount {
    fn get_base_units_name(&self) -> String {
        match self {
            Amount::Volume(a) => a.get_base_units_name().to_string(),
            Amount::Mass(a) => a.get_base_units_name().to_string(),
            Amount::Arbitrary(a) => a.units.to_string(),
        }
    }

    fn as_base_units(&self) -> f64 {
        match self {
            Amount::Volume(a) => a.as_base_units(),
            Amount::Mass(a) => a.as_base_units(),
            Amount::Arbitrary(a) => a.value,
        }
    }
}

impl Display for Amounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, (_, amount)) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(
                f,
                "{} {}",
                amount.as_base_units(),
                amount.get_base_units_name(),
            )?;
        }
        Ok(())
    }
}
impl Amounts {
    pub fn update(&mut self, mut other: Amounts) {
        let mut new: Vec<(String, Amount)> = Vec::new();
        for (key, s) in self.0.iter_mut() {
            if let Some(o) = other.0.remove(key) {
                match (s, o) {
                    (Amount::Volume(s), Amount::Volume(o)) => *s = *s + o,
                    (Amount::Mass(s), Amount::Mass(o)) => *s = *s + o,
                    (Amount::Arbitrary(s), Amount::Arbitrary(o)) => {
                        if s.units == o.units {
                            s.value += o.value
                        }
                    }
                    (_, o) => {
                        // We have a mismatch of types, append to new vector
                        new.push((key.to_owned(), o));
                    }
                }
            }
        }
        for (key, amount) in new {
            self.0.insert(key, amount);
        }
        for (key, o) in other.0 {
            if !self.0.contains_key(&key) {
                self.0.insert(key, o);
            }
        }
    }
}

impl From<(String, Amount)> for Amounts {
    fn from(pair: (String, Amount)) -> Self {
        Amounts(HashMap::from([(pair.0, pair.1)]))
    }
}

pub fn compute_amounts(quantity: &Option<String>, unit: &Option<String>) -> Amounts {
    match (quantity, unit) {
        (Some(q), Some(u)) => {
            if let Ok(value) = parse_quantity(q) {
                match u.as_str() {
                    "cup" | "cups" => {
                        (u.to_owned(), Amount::Volume(Volume::from_cups(value))).into()
                    }
                    "tablespoon" | "tablespoons" | "tbsp" => (
                        u.to_owned(),
                        Amount::Volume(Volume::from_tablespoons(value)),
                    )
                        .into(),
                    _ => (
                        u.to_owned(),
                        Amount::Arbitrary(Arbitrary {
                            units: u.to_string(),
                            value,
                        }),
                    )
                        .into(),
                }
            } else {
                (
                    "".to_string(),
                    Amount::Arbitrary(Arbitrary {
                        units: u.to_string(),
                        value: 1.0,
                    }),
                )
                    .into()
            }
        }
        (None, Some(u)) => (
            u.to_owned(),
            Amount::Arbitrary(Arbitrary {
                units: u.to_string(),
                value: 1.0,
            }),
        )
            .into(),
        (Some(q), None) => {
            if let Ok(value) = parse_quantity(q) {
                (
                    "".to_string(),
                    Amount::Arbitrary(Arbitrary {
                        units: "unknown".to_string(),
                        value,
                    }),
                )
                    .into()
            } else {
                (
                    "".to_string(),
                    Amount::Arbitrary(Arbitrary {
                        units: "unknown".to_string(),
                        value: 1.0,
                    }),
                )
                    .into()
            }
        }
        (None, None) => (
            "".to_string(),
            Amount::Arbitrary(Arbitrary {
                units: "count".to_string(),
                value: 1.0,
            }),
        )
            .into(),
    }
}

fn parse_quantity(q: &String) -> Result<f64> {
    if let Ok(q) = q.parse::<f64>() {
        Ok(q)
    } else {
        Err(anyhow!("not a valid quantity"))
    }
}
