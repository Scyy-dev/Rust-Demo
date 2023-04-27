use crate::inventory::Inventory;
use crate::stat::StatSet;

pub struct Player {
    stats: StatSet,
    health: u64,
    inventory: Inventory,
}

impl Player {
    pub fn damage(self: &mut Self, amount: u64) -> &Player {
        if self.health < amount {
            self.health = 0;
        } else {
            self.health = self.health - amount;
        }
        self
    }

    pub fn get_attack(self: &Self) -> u64 {
        5
    }
}
