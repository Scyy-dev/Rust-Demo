#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Action {
    Attack,
    Block,
    Invalid,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActionSet {
    actions: Vec<Action>,
}

impl Action {
    pub fn get_action(raw_action: char) -> Action {
        match raw_action {
            'a' => Self::Attack,
            'b' => Self::Block,
            _ => Self::Invalid,
        }
    }
}

impl TryFrom<char> for Action {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match Self::get_action(value) {
            Self::Invalid => Err(format!("Invalid action char: {}", value)),
            valid => Ok(valid),
        }
    }
}

impl ActionSet {
    pub fn new(actions: Vec<Action>) -> ActionSet {
        ActionSet { actions }
    }

    pub fn merge_all(sets: Vec<ActionSet>) -> ActionSet {
        let mut unique_actions = vec![];
        let actions = sets
            .iter()
            .flat_map(|set| set.actions.iter().map(|action| action.clone()))
            .collect::<Vec<Action>>();

        for action in &actions {
            if unique_actions.contains(&action) {
                unique_actions.push(action);
            }
        }
        ActionSet { actions }
    }
}

impl TryFrom<&str> for ActionSet {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut actions = vec![];
        let errs = value
            .chars()
            .map(|c| Action::try_from(c))
            .fold(vec![], |mut errs, result| match result {
                Ok(action) => {
                    actions.push(action);
                    errs
                }
                Err(err) => {
                    errs.push(err);
                    errs
                }
            });

        if errs.len() == 0 {
            return Ok(ActionSet { actions });
        } else {
            return Err(errs.join(", "));
        }
    }
}

impl Clone for ActionSet {
    fn clone(&self) -> Self {
        let actions = self.actions.iter().map(Action::clone).collect();
        ActionSet { actions }
    }
}
