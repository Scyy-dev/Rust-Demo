use crate::player::command::PlayerCommand;
use std::io;

pub fn read_int_range(prompt: &str, min: i64, max: i64) -> i64 {
    let mut input = String::new();
    loop {
        println!("{}", prompt);
        if io::stdin().read_line(&mut input).is_err() {
            println!("Invalid input!")
        }

        if let Ok(option) = input.trim().parse::<i64>() {
            if min <= option && option <= max {
                return option;
            }
        }
    }
}

pub fn read_command(prompt: &str) -> PlayerCommand {
    let mut input = String::new();
    loop {
        println!("{}", prompt);
        if !io::stdin().read_line(&mut input).is_err() {
            return PlayerCommand::new_from_string(&input);
        }
    }
}
