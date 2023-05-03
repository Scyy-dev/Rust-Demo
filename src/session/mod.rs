use crate::{
    entity::{
        action::{Action, ActionSet},
        enemy::SimpleEnemy,
        enemy_generator,
        player::Player,
        Entity,
    },
    inventory::{Inventory, Item},
    stat::{StatSet, StatType},
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
            difficulty: 1,
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

    pub fn enemy(&self) -> &SimpleEnemy {
        &self.enemy
    }

    pub fn end(&mut self) {
        self.active = false;
    }

    pub fn increment_difficulty(&mut self) {
        self.difficulty = self.difficulty + 1;
    }

    pub fn next_enemy(&mut self) {
        self.enemy = enemy_generator::generate_enemy(self.difficulty);
    }
}

pub fn demo_session() -> Session {
    // Prepare simple player
    let item1 = Item::try_from("Robe of Healing|v:2/a:0/d:3|h").unwrap();
    let item2 = Item::try_from("Wooden Sword|a:1|a").unwrap();
    let player_inv = Inventory::new(vec![item1, item2]);
    let player_stats = StatSet::try_from("v:2/a:3/d:0").unwrap();
    let player = Player::create(player_stats, player_inv);

    // Prepare a simple enemy
    let enemy_stats = StatSet::try_from("v:0/a:5/d:0");
    let enemy_actions = ActionSet::try_from("ann");
    let enemy = SimpleEnemy::new(enemy_stats.unwrap(), enemy_actions.unwrap(), 40);

    Session::new(player, enemy)
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
