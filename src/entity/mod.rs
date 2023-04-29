use crate::stat::StatType;

use self::action::Action;

pub mod action;
pub mod enemy;
pub mod player;

pub trait Entity {
    fn get_stat(self: &Self, stat_type: &StatType) -> u64;

    fn get_max_health(self: &Self) -> u64;

    fn get_health(self: &Self) -> u64;

    fn set_health(self: &mut Self, health: u64);

    fn damage(self: &mut Self, amount: u64);

    fn heal(self: &mut Self, amount: u64);

    fn is_dead(self: &Self) -> bool {
        self.get_health() == 0
    }
}

pub trait Actionable {
    fn next_action(self: &Self) -> Action;
}
