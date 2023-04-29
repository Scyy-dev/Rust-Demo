use crate::{
    entity::{action::Action, enemy::SimpleEnemy, player::Player, Entity},
    stat::StatType,
};

pub struct Session {
    player: Player,
    enemy: SimpleEnemy,
}

impl Session {
    pub fn new(player: Player, enemy: SimpleEnemy) -> Session {
        Session { player, enemy }
    }

    pub fn player_action(&mut self, player_action: &Action) {
        resolve(&mut self.player, &mut self.enemy, player_action);
    }

    pub fn enemy_action(&mut self, enemy_action: &Action) {
        resolve(&mut self.enemy, &mut self.player, enemy_action);
    }

    pub fn is_over(&self) -> bool {
        self.player.is_dead() || self.enemy.is_dead()
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn enemy(&self) -> &SimpleEnemy {
        &self.enemy
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
        _ => {}
    };
}
