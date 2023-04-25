pub enum Action {
    Attack,
    Block,
    Invalid,
}

impl Action {
    pub fn get_action(raw_action: char) -> Action {
        match raw_action {
            'a' => Self::Attack,
            'b' => Self::Block,
            _ => Self::Invalid,
        }
    }

    pub fn parse_actionset(actionset: &str) -> Vec<Action> {
        actionset
            .chars()
            .map(Self::get_action)
            .collect::<Vec<Action>>()
    }
}
