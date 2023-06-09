use rust_demo::{
    entity::{action::Action, enemy::SimpleEnemy, player::Player, Actionable, Entity},
    session::Session,
    stat::StatType,
    ui::{
        command::PlayerCommand,
        console,
        main_menu::{self, MainMenuOption},
        MenuAction,
    },
};

fn main() {
    loop {
        let option = main_menu::main_menu();

        match option {
            MainMenuOption::Help => {
                main_menu::print_help();
                continue;
            }
            MainMenuOption::Exit => {
                println!("Goodbye!");
                break;
            }
            MainMenuOption::Play => println!("Good luck!"),

            _ => {
                println!("Unknown menu option chosen.");
                continue;
            }
        }

        let player = Player::demo();
        let enemy = SimpleEnemy::demo();

        let mut session = Session::new(player, enemy);
        session.increment_difficulty();

        let mut bonus_stat = 0;

        while !session.is_over() {
            let command = console::read_command("");
            let player_action: Action;

            if command.is_menu_interaction() {
                let actions = handle_menu_interaction(command);
                if let Some(action_set) = actions {
                    for action in action_set {
                        action.handle(&mut session)
                    }
                }
                continue;
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

                session.increment_difficulty();

                if bonus_stat > 1 {
                    let stat_type = StatType::random();
                    session.player_mut().add_stat(&stat_type, 1);
                    bonus_stat = 0;
                    println!("You earned a {} stat!\n\n", stat_type.short_name());
                } else {
                    bonus_stat += 1;
                }

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

    println!("\n===================\nPlayer used {}!", player_action);

    // Prevent the enemy from making an action while dead
    if session.enemy().is_dead() {
        return;
    }

    let enemy_action = &session.enemy().next_action();
    session.enemy_action(&enemy_action);

    println!(
        "Enemy used {}!\n   ------------\n{}\n{}\n===================",
        &enemy_action,
        session.player(),
        session.enemy()
    );
}
