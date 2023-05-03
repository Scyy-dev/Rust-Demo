use crate::{session::Session, ui::MenuAction};

pub struct ViewInventoryAction {}

impl ViewInventoryAction {
    pub fn new() -> ViewInventoryAction {
        ViewInventoryAction {}
    }
}

impl MenuAction for ViewInventoryAction {
    fn is_valid(&self) -> bool {
        true
    }

    fn handle(&self, session: &mut Session) {
        println!("{}", session.player().inventory());
    }

    fn char(&self) -> char {
        'i'
    }
}
