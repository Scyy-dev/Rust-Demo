use crate::stat::StatType;

use super::item::Item;

pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn new(items: Vec<Item>) -> Inventory {
        Inventory { items }
    }

    pub fn from_string(string: &str) -> Inventory {
        todo!()
    }

    pub fn apply_bonuses(self: &Self, stat_type: &StatType, base: u64) -> u64 {
        todo!()
    }
}
