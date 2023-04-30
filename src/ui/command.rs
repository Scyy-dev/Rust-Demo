use crate::entity::action::Action;

use super::menu_action::MenuAction;

#[derive(Debug)]
pub struct PlayerCommand {
    chars: Vec<char>,
}

impl PlayerCommand {
    pub fn is_menu_interaction(&self) -> bool {
        unsafe { self.chars.get_unchecked(0) == &':' }
    }
}

impl From<String> for PlayerCommand {
    fn from(value: String) -> Self {
        let mut chars: Vec<char> = value.trim().chars().collect();
        while chars.len() < 3 {
            chars.push(' ');
        }
        PlayerCommand { chars }
    }
}

impl TryInto<Vec<MenuAction>> for PlayerCommand {
    type Error = &'static str;

    fn try_into(self) -> Result<Vec<MenuAction>, Self::Error> {
        if !self.is_menu_interaction() {
            return Err("Command is not a menu interaction");
        }

        let actions: Vec<MenuAction> = self
            .chars
            .iter()
            .map(|c| MenuAction::from(c.clone()))
            .filter(|action| action.is_valid())
            .collect();

        if actions.len() == 0 {
            return Err("No menu interactions found");
        } else {
            return Ok(actions);
        }
    }
}

impl TryFrom<PlayerCommand> for Action {
    type Error = String;

    fn try_from(value: PlayerCommand) -> Result<Self, Self::Error> {
        let c = value.chars.get(0);
        if c.is_none() {
            return Err(String::from("No action provided"));
        }

        let result = Action::try_from(c.unwrap().clone());
        match result {
            Ok(action) => Ok(action),
            Err(err) => Err(err),
        }
    }
}
