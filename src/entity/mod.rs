use std::fmt::Display;

use crate::stat::StatType;

use self::action::Action;

pub mod action;
pub mod enemy;
pub mod enemy_generator;
pub mod player;

pub trait Entity: Display {
    fn get_stat(&self, stat_type: &StatType) -> u64;

    fn add_stat(&mut self, stat_type: &StatType, stat: u64);

    fn get_max_health(&self) -> u64;

    fn get_health(&self) -> u64;

    fn set_health(&mut self, health: u64);

    fn damage(&mut self, amount: u64);

    fn heal(&mut self, amount: u64);

    fn is_dead(&self) -> bool {
        self.get_health() == 0
    }
}

pub trait Actionable {
    fn next_action(&self) -> Action;
}
