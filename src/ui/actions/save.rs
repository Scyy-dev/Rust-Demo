use crate::{session::Session, ui::MenuAction};

pub struct SaveAction {}

impl SaveAction {
    pub fn new() -> SaveAction {
        SaveAction {}
    }
}

impl MenuAction for SaveAction {
    fn is_valid(&self) -> bool {
        true
    }

    fn handle(&self, _session: &Session) {
        todo!("Implement session writing to file")
    }

    fn char(&self) -> char {
        'w'
    }
}
