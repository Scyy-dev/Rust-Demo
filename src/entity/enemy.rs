use std::fmt::Display;

use crate::stat::{StatSet, StatType};

use super::{
    action::{Action, ActionSet},
    Actionable, Entity,
};

pub struct SimpleEnemy {
    stats: StatSet,
    actions: ActionSet,
    max_health: u64,
    health: u64,
}

impl SimpleEnemy {
    pub fn new(stats: StatSet, actions: ActionSet, max_health: u64) -> SimpleEnemy {
        SimpleEnemy {
            stats,
            actions,
            max_health,
            health: max_health,
        }
    }
}

impl Entity for SimpleEnemy {
    fn get_stat(self: &Self, stat_type: &StatType) -> u64 {
        self.stats.get_stat(stat_type)
    }

    fn get_max_health(self: &Self) -> u64 {
        self.max_health
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

impl Display for SimpleEnemy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Simple Enemy: {}/{}", self.health, self.max_health)
    }
}

impl Actionable for SimpleEnemy {
    fn next_action(self: &Self) -> Action {
        self.actions.get_random_action()
    }
}
