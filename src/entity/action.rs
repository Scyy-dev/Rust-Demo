use std::fmt::Display;

use rand::Rng;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Action {
    // Moves from Items
    Attack,
    Heal,

    // Default Moveset
    WeakAttack,
    Nothing,

    // Illegal Actions
    Invalid,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActionSet {
    actions: Vec<Action>,
}

impl Action {
    pub fn get_char(&self) -> char {
        match self {
            Self::Attack => 'a',
            Self::Heal => 'h',
            Self::Nothing => 'n',
            Self::WeakAttack => 'w',
            _ => '!',
        }
    }

    pub fn get_action(raw_action: char) -> Action {
        match raw_action {
            'a' => Self::Attack,
            'h' => Self::Heal,
            'n' => Self::Nothing,
            'w' => Self::WeakAttack,
            _ => Self::Invalid,
        }
    }

    pub fn is_valid(&self) -> bool {
        self != &Action::Invalid
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

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

    pub fn get_random_action(&self) -> Action {
        let index = rand::thread_rng().gen_range(0..self.actions.len());
        self.actions.get(index).unwrap_or(&Action::Invalid).clone()
    }

    pub fn get_base_actions() -> ActionSet {
        ActionSet {
            actions: vec![Action::WeakAttack, Action::Nothing],
        }
    }

    pub fn contains(&self, action: &Action) -> bool {
        self.actions.contains(action)
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

impl Clone for ActionSet {
    fn clone(&self) -> Self {
        let actions = self.actions.iter().map(|action| action.clone()).collect();
        ActionSet { actions }
    }
}

impl Display for ActionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let actions: Vec<String> = self
            .actions
            .iter()
            .filter(|action| action.is_valid())
            .map(|action| format!("{}: {}", action, action.get_char()))
            .collect();
        let actions = actions.join(", ");
        write!(f, "Actions: [{}]", actions)
    }
}
