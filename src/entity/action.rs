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

    // O(n^2) algorithm for merging action sets
    // due to the common use case of action sets only containing a 'small' (n < 10) number of items,
    // the performance gains from using a set are neglible
    //
    // Creates a new set
    pub fn merge_all(sets: Vec<&ActionSet>) -> ActionSet {
        let mut unique_actions: Vec<Action> = vec![];
        let actions: Vec<&Action> = sets
            .iter()
            .flat_map(|set| set.actions.iter())
            .collect::<Vec<&Action>>();

        for action in actions {
            if !unique_actions.contains(&action) {
                unique_actions.push(action.clone());
            }
        }
        ActionSet {
            actions: unique_actions,
        }
    }
}

impl TryFrom<&str> for ActionSet {
    type Error = String;

    // Regex format:
    // [a-z]+
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
