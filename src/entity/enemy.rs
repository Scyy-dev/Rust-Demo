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

    pub fn demo() -> SimpleEnemy {
        let enemy_stats = StatSet::try_from("v:0/a:5/d:0");
        let enemy_actions = ActionSet::try_from("ann");
        SimpleEnemy::new(enemy_stats.unwrap(), enemy_actions.unwrap(), 40)
    }
}

impl Entity for SimpleEnemy {
    fn get_stat(&self, stat_type: &StatType) -> u64 {
        self.stats.get_stat(stat_type)
    }

    fn add_stat(&mut self, stat_type: &StatType, stat: u64) {
        self.stats.add_stat(stat_type, stat);
    }

    fn get_max_health(&self) -> u64 {
        self.max_health
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
        let health = self.get_health();
        if health + amount >= max {
            self.health = max;
        } else {
            self.health += amount;
        }
    }

    fn is_dead(&self) -> bool {
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
