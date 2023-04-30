use std::cell::Cell;

use crate::{
    entity::{action::Action, enemy::SimpleEnemy, player::Player, Entity},
    stat::StatType,
};

pub struct Session {
    player: Player,
    enemy: SimpleEnemy,
    active: Cell<bool>,
}

impl Session {
    pub fn new(player: Player, enemy: SimpleEnemy) -> Session {
        Session {
            player,
            enemy,
            active: Cell::from(true),
        }
    }

    pub fn player_action(&mut self, player_action: &Action) {
        resolve(&mut self.player, &mut self.enemy, player_action);
    }

    pub fn enemy_action(&mut self, enemy_action: &Action) {
        resolve(&mut self.enemy, &mut self.player, enemy_action);
    }

    pub fn is_over(&self) -> bool {
        self.player.is_dead() || self.enemy.is_dead() || !self.active.get()
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn enemy(&self) -> &SimpleEnemy {
        &self.enemy
    }

    pub fn end(&self) {
        self.active.set(false);
    }
}

fn resolve(entity: &mut dyn Entity, target: &mut dyn Entity, action: &Action) {
    match action {
        Action::Attack => {
            let attack = entity.get_stat(&StatType::Attack);
            let defence = target.get_stat(&StatType::Defence);
            if defence >= attack {
                return;
            }
            target.damage(attack - defence);
        }
        Action::Heal => {
            let amount = entity.get_stat(&StatType::Defence);
            entity.heal(amount);
        }
        Action::WeakAttack => {
            let amount = entity.get_stat(&StatType::Attack);
            let amount = (amount / 5).max(1); // Player always does at least 1 damage
            target.damage(amount);
        }
        _ => {}
    };
}
