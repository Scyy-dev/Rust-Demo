use crate::stat::StatType;

use super::item::Item;

pub struct Inventory {
    items: [Item; 5],
}

impl Inventory {
    pub fn new(items: [Item; 5]) -> Inventory {
        Inventory { items }
    }

    pub fn apply_bonuses(self: &Self, stat_type: StatType, base: u64) -> u64 {
        let mut base = base;
        self.items.iter().map(|item| item.apply_bonuses(stat_type, base))
        //self.items.iter().flat_map(|item| item.)
    }
}
