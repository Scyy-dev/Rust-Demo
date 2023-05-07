use crate::session::Session;

use self::actions::{HelpAction, ListActionsAction, QuitAction, SaveAction, ViewInventoryAction};

pub mod actions;
pub mod command;
pub mod console;
pub mod main_menu;

pub trait MenuAction {
    fn is_valid(&self) -> bool;
    fn handle(&self, session: &mut Session);
    fn char(&self) -> char;
}
pub struct InvalidAction {}

impl MenuAction for InvalidAction {
    fn is_valid(&self) -> bool {
        false
    }

    fn handle(&self, _session: &mut Session) {
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
        'a' => Box::new(ListActionsAction::new()),
        'h' => Box::new(HelpAction::new()),
        'i' => Box::new(ViewInventoryAction::new()),
        'q' => Box::new(QuitAction::new()),
        'w' => Box::new(SaveAction::new()),
        _ => Box::new(InvalidAction {}),
    }
}
