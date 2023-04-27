use crate::inventory::Inventory;
use crate::stat::{StatSet, StatType};

pub struct Player {
    stats: StatSet,
    health: u64,
    inventory: Inventory,
}

impl Player {

    pub fn new() -> Player {
        Player {
            stats: StatSet::base_player_stats(),
            health: 100,
            inventory: Inventory::new(vec![])
        }
    }

    pub fn damage(self: &mut Self, amount: u64) -> &Player {
        if self.health < amount {
            self.health = 0;
        } else {
            self.health = self.health - amount;
        }
        self
    }

    pub fn calculate_max_health(&self) -> u64 {
        let vit = self.stats.get_stat(&StatType::Vitality);
        let vit = vit + self.inventory.get_stat(&StatType::Vitality);

        100 + vit * 15
    }

    pub fn heal(self: &mut Self, amount: u64) -> &Player {
        let max = self.calculate_max_health();
        if self.health + amount >= max {
            self.health = max;
        } else {
            self.health += amount;
        }
        self
    }

}
