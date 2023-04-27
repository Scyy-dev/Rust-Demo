#[derive(Debug)]
pub struct PlayerCommand {
    char_sequence: [char; 3],
}

impl PlayerCommand {
    pub fn new_from_string(raw_command: &str) -> PlayerCommand {
        let mut chars: Vec<char> = raw_command.trim().chars().collect::<Vec<char>>();
        while chars.len() < 3 {
            chars.push(' ');
        }
        PlayerCommand {
            char_sequence: [chars[0], chars[1], chars[2]],
        }
    }

    pub fn new(char_sequence: [char; 3]) -> PlayerCommand {
        PlayerCommand { char_sequence }
    }

    pub fn is_menu_interaction(&self) -> bool {
        self.char_sequence[0] == ':'
    }
}
