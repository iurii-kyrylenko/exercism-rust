use bit_set::BitSet;
use num_enum::TryFromPrimitive;
use std::collections::HashSet;
use std::convert::TryFrom;

pub struct Allergies {
    set: HashSet<Allergen>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let set = BitSet::from_bytes(&[score as u8])
            .iter()
            .map(|b| Allergen::try_from(7 - b as u8).unwrap())
            .collect();

        Self { set }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.set.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.set.iter().cloned().collect()
    }
}
