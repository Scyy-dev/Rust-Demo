use crate::ui::{main_menu, MenuAction};

pub struct HelpAction {}

impl HelpAction {
    pub fn new() -> HelpAction {
        HelpAction {}
    }
}

impl MenuAction for HelpAction {
    fn is_valid(&self) -> bool {
        true
    }

    fn handle(&self, _session: &mut crate::session::Session) {
        main_menu::print_help();
    }

    fn char(&self) -> char {
        'h'
    }
}
