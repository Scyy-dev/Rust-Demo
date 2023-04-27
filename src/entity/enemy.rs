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

    fn set_health(self: &mut Self, health: u64) -> &Self {
        self.health = health;
        self
    }

    fn damage(self: &mut Self, amount: u64) -> &Self {
        if amount >= self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
        self
    }

    fn heal(self: &mut Self, amount: u64) -> &Self {
        let max = self.get_max_health();
        if amount >= max {
            self.health = max;
        } else {
            self.health += amount;
        }
        self
    }

    fn get_action_set(self: &Self) -> ActionSet {
        self.actions.clone()
    }
}

impl Actionable for SimpleEnemy {
    fn next_action(self: &Self) -> Action {
        todo!()
    }
}
