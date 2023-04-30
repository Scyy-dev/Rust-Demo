#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MenuAction {
    Quit,
    Invalid,
}

impl MenuAction {
    pub fn is_valid(&self) -> bool {
        self == &Self::Invalid
    }
}

impl From<char> for MenuAction {
    fn from(value: char) -> Self {
        match value {
            'q' => Self::Quit,
            _ => Self::Invalid,
        }
    }
}
