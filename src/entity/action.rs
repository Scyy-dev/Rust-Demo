#[derive(Debug)]
pub enum Action {
    Attack,
    Block,
    Invalid,
}

#[derive(Debug)]
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
