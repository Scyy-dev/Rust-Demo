use std::io;
use std::convert::From;

enum MainMenuOption {
    Load,
    Play,
    Exit,
    Invalid
}

impl From<usize> for MainMenuOption {

    fn from(option: usize) -> MainMenuOption {
        match option {
            1 => Self::Load,
            2 => Self::Play,
            3 => Self::Exit,
            _ => Self::Invalid
        }
    }
}

fn main() {
    main_menu();
}

fn read_menu_option(prompt: &str, min: usize, max: usize) -> usize {
    let mut input = String::new();
    loop {
        println!("{}", prompt);
        if io::stdin().read_line(&mut input).is_err() {
            println!("Invalid input!")
        }

        if let Ok(option) = input.parse::<usize>() {
            if min <= option && option <= max {
                return option;
            }
        }
    }
}

fn main_menu() -> MainMenuOption {
    show_menu();
    let option = read_menu_option("Please enter a number in the range 1-3", 1, 3);
    MainMenuOption::from(option)
}

fn show_menu() {
    println!("Please select an option below:");
    println!("  1) Load Game");
    println!("  2) Save Game");
    println!("  3) Exit");
}
