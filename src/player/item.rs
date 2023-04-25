use crate::stat::*;

use super::action::Action;

pub struct Item {
    bonuses: StatSet,
    moves: Vec<Action>,
}

impl Item {
    pub fn new(bonuses: StatSet, moves: Vec<Action>) -> Item {
        Item { bonuses, moves }
    }

    pub fn apply_bonuses(self: &Self, stat_type: StatType, base: u64) -> u64 {
        base + self.bonuses.get_stat(stat_type)
    }
}
