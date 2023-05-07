use crate::ui::console;
use std::{convert::From, fmt::Display};

#[derive(Debug)]
pub enum MainMenuOption {
    Help,
    Play,
    Exit,
    Invalid,
}

impl From<i64> for MainMenuOption {
    fn from(option: i64) -> MainMenuOption {
        match option {
            1 => Self::Help,
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
    println!("  1) Help");
    println!("  2) Play New Game");
    println!("  3) Exit");
}

pub fn main_menu() -> MainMenuOption {
    show_menu();
    let option = console::read_int_range("Please enter a number in the range 1-3", 1, 3);
    MainMenuOption::from(option)
}

pub fn print_help() {
    println!(
        "\
===============================================
How to Play:

The objective is to progress as far as possible!

You interact by running commands.

Commands take two forms - menu actions or actions

Menu actions operate similar to Vim, where they're prefixed with
a ':' and a letter follows which decides the action.

Menu Actions available:
:q - quit the game
:i - show your inventory
:a - list your available actions
:h - bring up this help guide

Actions are abilities or moves that you perform in combat.
Actions are gained by collecting items.

Example Actions:
a - perform an attack for 100% of ATT
h - heal for 100% of DEF

Good luck!
===============================================
        "
    );
}
