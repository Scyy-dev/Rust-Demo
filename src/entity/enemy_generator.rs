use rand::Rng;

use crate::stat::{StatEntry, StatSet, StatType};

use super::{action::ActionSet, enemy::SimpleEnemy};

const MINIMUM_ACTIONS: &str = "nnw";

pub fn generate_enemy(difficulty: u64) -> SimpleEnemy {
    // Health
    let base_health = 20 + difficulty * 3;
    let health_variation = rand::thread_rng().gen_range(0..5);
    let health = base_health + health_variation;

    // Stats
    let base_attack = 1 + difficulty / 2;
    let attack_variation = rand::thread_rng().gen_range(1..2);
    let attack = base_attack + attack_variation;

    let base_defence = difficulty / 3;
    let defence_variation = rand::thread_rng().gen_range(1..3);
    let defence = base_defence + defence_variation;

    let stats = StatSet::new(vec![
        StatEntry::new(StatType::Attack, attack),
        StatEntry::new(StatType::Defence, defence),
    ]);

    // Actions
    let possible_actions = "nnnwwwaaah";
    let mut actions: String = possible_actions
        .chars()
        .filter(|_| rand::thread_rng().gen::<f32>() > 0.5)
        .collect();

    if actions.len() < 3 {
        actions = MINIMUM_ACTIONS.to_string();
    }

    let action_results = ActionSet::try_from(actions.as_str());
    let mut actions = default_actions();

    if let Ok(vald_actions) = action_results {
        actions = vald_actions;
    }

    SimpleEnemy::new(stats, actions, health)
}

fn default_actions() -> ActionSet {
    ActionSet::try_from(MINIMUM_ACTIONS).unwrap()
}
