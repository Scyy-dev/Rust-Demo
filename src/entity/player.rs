use std::fmt::Display;

use crate::inventory::Inventory;
use crate::stat::{StatSet, StatType};

use super::Entity;

#[derive(Debug)]
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
            inventory: Inventory::new(vec![]),
        }
    }

    pub fn create(stats: StatSet, health: u64, inventory: Inventory) -> Player {
        Player {
            stats,
            health,
            inventory,
        }
    }
}

impl Entity for Player {
    fn get_stat(self: &Self, stat_type: &StatType) -> u64 {
        self.stats.get_stat(stat_type) + self.inventory.get_stat(stat_type)
    }

    fn get_max_health(self: &Self) -> u64 {
        25 + self.get_stat(&StatType::Vitality) * 15
    }

    fn get_health(self: &Self) -> u64 {
        self.health
    }

    fn set_health(self: &mut Self, health: u64) {
        self.health = health;
    }

    fn damage(self: &mut Self, amount: u64) {
        if amount >= self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
    }

    fn heal(self: &mut Self, amount: u64) {
        let max = self.get_max_health();
        if amount >= max {
            self.health = max;
        } else {
            self.health += amount;
        }
    }

    fn is_dead(self: &Self) -> bool {
        self.get_health() == 0
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player: {}/{}", self.health, self.get_max_health())
    }
}
