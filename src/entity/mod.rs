use crate::stat::StatType;

use self::action::{Action, ActionSet};

pub mod action;
pub mod enemy;
pub mod player;

pub trait Entity {
    fn get_stat(self: &Self, stat_type: &StatType) -> u64;

    fn get_max_health(self: &Self) -> u64;

    fn get_health(self: &Self) -> u64;

    fn set_health(self: &mut Self, health: u64) -> &Self;

    fn damage(self: &mut Self, amount: u64) -> &Self;

    fn heal(self: &mut Self, amount: u64) -> &Self;

    fn is_dead(self: &Self) -> bool {
        self.get_health() == 0
    }

    fn get_action_set(self: &Self) -> &ActionSet;
}

pub trait Actionable {
    fn next_action(self: &Self) -> Action;
}
