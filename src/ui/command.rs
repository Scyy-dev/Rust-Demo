use crate::entity::action::Action;

use super::MenuAction;

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
        let chars: Vec<char> = value.trim().chars().collect();
        PlayerCommand { chars }
    }
}

impl TryFrom<PlayerCommand> for Vec<Box<dyn MenuAction>> {
    type Error = String;

    fn try_from(value: PlayerCommand) -> Result<Self, Self::Error> {
        let c = value.chars.get(0);
        if c.is_none() {
            return Err(String::from("Not a valid menu command"));
        }

        let actions: Vec<Box<dyn MenuAction>> = value.chars[1..]
            .iter()
            .map(|c| crate::ui::get_action(c.clone()))
            .filter(|action| action.is_valid())
            .collect();

        // We don't know which characters were invalid, so just show the command used
        if actions.len() != value.chars[1..].len() {
            let err = format!(
                "Invalid menu actions found in '{}'",
                value.chars[1..].iter().collect::<String>()
            );
            return Err(err);
        }

        Ok(actions)
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
