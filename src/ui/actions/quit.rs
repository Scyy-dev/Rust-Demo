use crate::{session::Session, ui::MenuAction};

pub struct QuitAction {}

impl QuitAction {
    pub fn new() -> QuitAction {
        QuitAction {}
    }
}

impl MenuAction for QuitAction {
    fn is_valid(&self) -> bool {
        true
    }

    fn handle(&self, session: &Session) {
        session.end();
    }

    fn char(&self) -> char {
        'q'
    }
}
