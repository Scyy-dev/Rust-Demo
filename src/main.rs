use rust_demo::{
    entity::{
        action::{Action, ActionSet},
        enemy::SimpleEnemy,
        player::Player,
        Actionable, Entity,
    },
    inventory::{Inventory, Item},
    session::Session,
    stat::StatSet,
    ui::main_menu,
};

fn main() {
    let option = main_menu::main_menu();
    // TODO - interpreting of menu options
    println!("You chose option: {}", option);

    // Prepare simple player
    let item1 = Item::try_from("v:2/a:1/d:3|h").unwrap();
    let player_inv = Inventory::new(vec![item1]);
    let player_stats = StatSet::try_from("v:2/a:3/d:0").unwrap();
    let player = Player::create(player_stats, player_inv);

    // Prepare a simple enemy
    let enemy_stats = StatSet::try_from("v:0/a:5/d:0");
    let enemy_actions = ActionSet::try_from("ann");
    let enemy = SimpleEnemy::new(enemy_stats.unwrap(), enemy_actions.unwrap(), 40);

    let mut session = Session::new(player, enemy);

    while !session.is_over() {

        let player_action = Action::Attack;
        session.player_action(&player_action);

        println!(
            "Player used {}!\n{}\n{}",
            player_action,
            session.player(),
            session.enemy()
        );

        // Prevent the enemy from making an action while dead
        if session.enemy().is_dead() {
            break;
        }

        let enemy_action = &session.enemy().next_action();
        session.enemy_action(&enemy_action);

        println!(
            "Enemy used {}!\n{}\n{}",
            &enemy_action,
            session.player(),
            session.enemy()
        );
    }

    if session.enemy().is_dead() {
        println!("Congrats! You defeated the enemy!");
    } else {
        println!("Oh no! You died!")
    }

    println!("Player items:");
    for item in session.player().inventory().items() {
        println!("{}", item);
    }
}
