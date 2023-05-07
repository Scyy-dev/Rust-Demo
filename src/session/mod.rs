use crate::{
    entity::{action::Action, enemy::SimpleEnemy, enemy_generator, player::Player, Entity},
    stat::StatType,
};

pub struct Session {
    player: Player,
    enemy: SimpleEnemy,
    difficulty: u64,
    active: bool,
}

impl Session {
    pub fn new(player: Player, enemy: SimpleEnemy) -> Session {
        Session {
            player,
            enemy,
            difficulty: 0,
            active: true,
        }
    }

    pub fn player_action(&mut self, player_action: &Action) {
        resolve(&mut self.player, &mut self.enemy, player_action);
    }

    pub fn enemy_action(&mut self, enemy_action: &Action) {
        resolve(&mut self.enemy, &mut self.player, enemy_action);
    }

    pub fn is_over(&self) -> bool {
        self.player.is_dead() || !self.active
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn enemy(&self) -> &SimpleEnemy {
        &self.enemy
    }

    pub fn enemy_mut(&mut self) -> &mut SimpleEnemy {
        &mut self.enemy
    }

    pub fn end(&mut self) {
        self.active = false;
    }

    pub fn increment_difficulty(&mut self) {
        self.difficulty = self.difficulty + 1;
        self.next_enemy();
    }

    fn next_enemy(&mut self) {
        self.enemy = enemy_generator::generate_enemy(self.difficulty);
    }
}

fn resolve(entity: &mut impl Entity, target: &mut impl Entity, action: &Action) {
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
