use std::{collections::HashMap, fmt::Display};

use measurements::{mass, volume, Mass, Measurement, Volume};

// Map of unit to amount for a single ingredient
#[derive(Debug, Clone)]
pub struct Amounts {
    volume: Option<Volume>,
    mass: Option<Mass>,
    arbitrary: HashMap<String, Arbitrary>,
}

#[derive(Debug, Clone)]
pub struct Arbitrary {
    pub units: String,
    pub value: f64,
}

impl Amounts {
    fn get_volume_units(&self) -> Option<(&'static str, f64)> {
        if let Some(v) = self.volume {
            let list = [
                ("tsp", 1.0 / volume::LITER_TEASPOONS_FACTOR),
                ("tbsp", 1.0 / volume::LITER_TABLESPOONS_FACTOR),
                ("fl oz", 1.0 / volume::LITER_FLUID_OUNCES_FACTOR),
                ("cup", 1.0 / volume::LITER_CUP_FACTOR),
                ("gal", 1.0 / volume::LITER_GALLONS_FACTOR),
            ];
            Some(v.pick_appropriate_units(&list))
        } else {
            None
        }
    }
    fn get_mass_units(&self) -> Option<(&'static str, f64)> {
        if let Some(m) = self.mass {
            let list = [
                ("oz", 1.0 / mass::KILOGRAM_OUNCES_FACTOR),
                ("lbs", 1.0 / mass::KILOGRAM_POUNDS_FACTOR),
            ];
            Some(m.pick_appropriate_units(&list))
        } else {
            None
        }
    }
}

impl Display for Amounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((units, value)) = self.get_volume_units() {
            write!(f, "{:.2} {}", value, units,)?;
        }
        if let Some((units, value)) = self.get_mass_units() {
            write!(f, " {:.2} {}", value, units,)?;
        }
        for (_, amount) in self.arbitrary.iter() {
            write!(f, " ")?;
            write!(f, "{:.2} {}", amount.value, amount.units)?;
        }
        Ok(())
    }
}
impl Amounts {
    pub fn update(&mut self, mut other: Amounts) {
        match (self.volume, other.volume) {
            (Some(s), Some(o)) => self.volume = Some(s + o),
            (None, Some(o)) => self.volume = Some(o),
            _ => {}
        };
        match (self.mass, other.mass) {
            (Some(s), Some(o)) => self.mass = Some(s + o),
            (None, Some(o)) => self.mass = Some(o),
            _ => {}
        };
        for (key, s) in self.arbitrary.iter_mut() {
            if let Some(o) = other.arbitrary.remove(key) {
                s.value += o.value
            }
        }
        for (key, o) in other.arbitrary {
            self.arbitrary.entry(key).or_insert(o);
        }
    }
}

impl From<Volume> for Amounts {
    fn from(src: Volume) -> Self {
        Amounts {
            volume: Some(src),
            mass: None,
            arbitrary: HashMap::new(),
        }
    }
}
impl From<Mass> for Amounts {
    fn from(src: Mass) -> Self {
        Amounts {
            volume: None,
            mass: Some(src),
            arbitrary: HashMap::new(),
        }
    }
}
impl From<(String, Arbitrary)> for Amounts {
    fn from(src: (String, Arbitrary)) -> Self {
        Amounts {
            volume: None,
            mass: None,
            arbitrary: HashMap::from([src]),
        }
    }
}

pub fn compute_amounts(quantity: &Option<(String, f64)>, unit: &Option<String>) -> Amounts {
    match (quantity, unit) {
        (Some((_, q)), Some(u)) => match u.as_str() {
            "cup" | "cups" => Volume::from_cups(*q).into(),
            "tablespoon" | "tablespoons" | "tbsp" => Volume::from_tablespoons(*q).into(),
            "fl oz" => Volume::from_fluid_ounces(*q).into(),
            "oz" => Mass::from_ounces(*q).into(),
            "lbs" | "lb" => Mass::from_pounds(*q).into(),
            _ => (
                u.to_owned(),
                Arbitrary {
                    units: u.to_string(),
                    value: *q,
                },
            )
                .into(),
        },
        (None, Some(u)) => (
            u.to_owned(),
            Arbitrary {
                units: u.to_string(),
                value: 1.0,
            },
        )
            .into(),
        (Some((_, q)), None) => (
            "".to_string(),
            Arbitrary {
                units: "".to_string(),
                value: *q,
            },
        )
            .into(),
        (None, None) => (
            "".to_string(),
            Arbitrary {
                units: "".to_string(),
                value: 1.0,
            },
        )
            .into(),
    }
}
