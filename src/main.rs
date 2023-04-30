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
    ui::{console, main_menu, MenuAction},
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
        let command = console::read_command("");
        let player_action: Action;

        // Menu Interaction
        if command.is_menu_interaction() {
            let actions = command.try_into();
            if let Err(err) = actions {
                println!("\n{}", err);
                continue;
            }

            let actions: Vec<Box<dyn MenuAction>> = actions.unwrap();

            let errs = actions.iter().fold(String::new(), |mut s, action| {
                if !action.is_valid() {
                    s = s + "\n  Invalid action: " + &action.char().to_string();
                    s
                } else {
                    s
                }
            });

            if errs.len() != 0 {
                println!("Error parsing command: {}", errs);
                continue;
            }

            for action in &actions {
                action.handle(&session);
            }

            continue;

        // Player Action
        } else {
            let action = command.try_into().unwrap_or(Action::Invalid);
            if !action.is_valid() {
                println!("Invalid action.");
                continue;
            } else {
                player_action = action;
            }
        }

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
    } else if session.player().is_dead() {
        println!("Oh no! You died!")
    } else {
        println!("Session ended early.")
    }
}
