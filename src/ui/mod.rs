use crate::session::Session;

use self::actions::{QuitAction, SaveAction, ViewInventoryAction};

pub mod actions;
pub mod command;
pub mod console;
pub mod main_menu;

pub trait MenuAction {
    fn is_valid(&self) -> bool;
    fn handle(&self, session: &Session);
    fn char(&self) -> char;
}

pub struct RootAction {}
pub struct InvalidAction {}

impl MenuAction for RootAction {
    fn is_valid(&self) -> bool {
        true
    }

    fn handle(&self, _session: &Session) {
        // The root action can never be called upon, and it is invalid to do so
        panic!("Cannot handle root function")
    }

    fn char(&self) -> char {
        'r'
    }
}

impl MenuAction for InvalidAction {
    fn is_valid(&self) -> bool {
        false
    }

    fn handle(&self, _session: &Session) {
        panic!("Cannot handle invalid function")
    }

    fn char(&self) -> char {
        'i'
    }
}

pub fn invalid_action() -> Box<dyn MenuAction> {
    Box::new(InvalidAction {})
}

pub fn get_action(c: char) -> Box<dyn MenuAction> {
    match c {
        'i' => Box::new(ViewInventoryAction::new()),
        'q' => Box::new(QuitAction::new()),
        'w' => Box::new(SaveAction::new()),
        _ => Box::new(InvalidAction {}),
    }
}
