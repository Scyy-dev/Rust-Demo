use crate::entity::action::Action;

use super::MenuAction;

#[derive(Debug)]
pub struct PlayerCommand {
    chars: Vec<char>,
}

impl PlayerCommand {
    pub fn is_menu_interaction(&self) -> bool {
        self.chars.get(0).map_or(' ', |c| *c) == ':'
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

#[test]
fn parse_action_command_test() {
    let command_result = PlayerCommand::try_from("a".to_string());
    assert!(command_result.is_ok());

    let command = command_result.unwrap();
    assert!(!command.is_menu_interaction());

    let action_result = Action::try_from(command);
    assert!(action_result.is_ok());

    let action = action_result.unwrap();
    assert_eq!(action, Action::Attack);
}

#[test]
fn parse_menu_action_command_test() {
    let command_result = PlayerCommand::try_from(":q".to_string());
    assert!(command_result.is_ok());

    let command = command_result.unwrap();
    assert!(command.is_menu_interaction());

    let menu_actions_result = command.try_into();
    assert!(menu_actions_result.is_ok());

    let menu_actions: Vec<Box<dyn MenuAction>> = menu_actions_result.unwrap();
    let menu_action_option = menu_actions.get(0);
    assert!(menu_action_option.is_some());

    let menu_action = menu_action_option.unwrap();
    assert!(menu_action.is_valid());
    assert_eq!('q', menu_action.char());
}
