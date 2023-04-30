use crate::ui::MenuAction;

pub struct ListActionsAction {}

impl ListActionsAction {
    pub fn new() -> ListActionsAction {
        ListActionsAction {}
    }
}

impl MenuAction for ListActionsAction {
    fn is_valid(&self) -> bool {
        true
    }

    fn handle(&self, session: &crate::session::Session) {
        let actions = session.player().actions();
        println!("Available actions:\n\n{}", actions);
    }

    fn char(&self) -> char {
        'a'
    }
}
