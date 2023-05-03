use std::fmt::Display;

use crate::inventory::{Inventory, Item};
use crate::stat::{StatSet, StatType};

use super::action::{Action, ActionSet};
use super::Entity;

#[derive(Debug)]
pub struct Player {
    stats: StatSet,
    health: u64,
    inventory: Inventory,
}

impl Player {
    pub fn new() -> Player {
        let stats = StatSet::base_player_stats();
        let max_health = calculate_max_health(&stats.get_stat(&StatType::Vitality));
        Player {
            stats,
            health: max_health,
            inventory: Inventory::new(vec![]),
        }
    }

    pub fn create(stats: StatSet, inventory: Inventory) -> Player {
        let vit_stat = &stats.get_stat(&StatType::Vitality);
        let inventory_vit = &inventory.get_stat(&StatType::Vitality);
        let max_health = calculate_max_health(&(vit_stat + inventory_vit));
        Player {
            stats,
            health: max_health,
            inventory,
        }
    }

    pub fn can_perform(&self, action: &Action) -> bool {
        self.actions().contains(action)
    }

    pub fn actions(&self) -> ActionSet {
        ActionSet::merge_all(vec![
            &ActionSet::get_base_actions(),
            &self.inventory.get_actions(),
        ])
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn demo() -> Player {
        let item1 = Item::try_from("Robe of Healing|v:2/a:0/d:3|h").unwrap();
        let item2 = Item::try_from("Wooden Sword|a:1|a").unwrap();
        let player_inv = Inventory::new(vec![item1, item2]);
        let player_stats = StatSet::try_from("v:2/a:3/d:0").unwrap();
        Player::create(player_stats, player_inv)
    }
}

impl Entity for Player {
    fn get_stat(&self, stat_type: &StatType) -> u64 {
        self.stats.get_stat(stat_type) + self.inventory.get_stat(stat_type)
    }

    fn get_max_health(&self) -> u64 {
        25 + self.get_stat(&StatType::Vitality) * 5
    }

    fn get_health(&self) -> u64 {
        self.health
    }

    fn set_health(&mut self, health: u64) {
        self.health = health;
    }

    fn damage(&mut self, amount: u64) {
        if amount >= self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
    }

    fn heal(&mut self, amount: u64) {
        let max = self.get_max_health();
        if amount >= max {
            self.health = max;
        } else {
            self.health += amount;
        }
    }

    fn is_dead(&self) -> bool {
        self.get_health() == 0
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player: {}/{}", self.health, self.get_max_health())
    }
}

fn calculate_max_health(vit: &u64) -> u64 {
    vit * 5 + 25
}
