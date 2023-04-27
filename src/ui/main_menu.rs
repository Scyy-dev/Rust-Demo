use crate::ui::console;
use std::{convert::From, fmt::Display};

#[derive(Debug)]
pub enum MainMenuOption {
    Load,
    Play,
    Exit,
    Invalid,
}

impl From<i64> for MainMenuOption {
    fn from(option: i64) -> MainMenuOption {
        match option {
            1 => Self::Load,
            2 => Self::Play,
            3 => Self::Exit,
            _ => Self::Invalid,
        }
    }
}

impl Display for MainMenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn show_menu() {
    println!("Please select an option below:");
    println!("  1) Load & Play Game");
    println!("  2) Play New Game");
    println!("  3) Exit");
}

pub fn main_menu() -> MainMenuOption {
    show_menu();
    let option = console::read_int_range("Please enter a number in the range 1-3", 1, 3);
    MainMenuOption::from(option)
}
