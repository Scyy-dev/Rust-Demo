use rust_demo::{
    entity::{action::Action, Actionable, Entity},
    session::{self, Session},
    ui::{command::PlayerCommand, console, main_menu, MenuAction},
};

fn main() {
    let option = main_menu::main_menu();
    // TODO - interpreting of menu options
    println!("You chose option: {}", option);

    let mut session = session::demo_session();

    println!("Good luck!");

    while !session.is_over() {
        let command = console::read_command("");
        let player_action: Action;

        // Menu Interaction
        if command.is_menu_interaction() {
            let actions = handle_menu_interaction(command);
            if let Some(action_set) = actions {
                for action in action_set {
                    action.handle(&mut session)
                }
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

        handle_player_interaction(&mut session, &player_action);

        if session.enemy().is_dead() {
            println!("Congrats! You killed the enemy!\n\n");
            // TODO - give the player a random stat
            session.next_enemy();
            println!("Another enemy has spawned! Good luck!\n\n")
        }
    }

    if session.enemy().is_dead() {
        println!("Congrats! You defeated the enemy!");
    } else if session.player().is_dead() {
        println!("Oh no! You died!")
    } else {
        println!("Session ended early.")
    }
}

fn handle_menu_interaction(command: PlayerCommand) -> Option<Vec<Box<dyn MenuAction>>> {
    let actions = command.try_into();
    if let Err(err) = actions {
        println!("\n{}", err);
        return None;
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
        return None;
    }

    Some(actions)
}

fn handle_player_interaction(session: &mut Session, player_action: &Action) {
    session.player_action(&player_action);

    println!(
        "Player used {}!\n{}\n{}",
        player_action,
        session.player(),
        session.enemy()
    );

    // Prevent the enemy from making an action while dead
    if session.enemy().is_dead() {
        return;
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
