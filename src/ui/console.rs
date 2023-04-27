use std::io;

use super::command::PlayerCommand;

pub fn read_int_range(prompt: &str, min: i64, max: i64) -> i64 {
    loop {
        let mut input = String::new();
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
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        if !io::stdin().read_line(&mut input).is_err() {
            return PlayerCommand::from(input);
        }
    }
}
