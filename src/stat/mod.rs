#[derive(Debug, PartialEq, Eq)]
pub enum StatType {
    Vitality,
    Attack,
    Defence,
    Invalid,
}

#[derive(Debug)]
pub struct StatEntry {
    stat_type: StatType,
    stat: u64,
}

#[derive(Debug)]
pub struct StatSet {
    set: Vec<StatEntry>,
}

impl StatSet {
    pub fn new(set: Vec<StatEntry>) -> StatSet {
        StatSet { set }
    }

    pub fn zeros() -> StatSet {
        StatSet { set: vec![] }
    }

    pub fn from_string(string: &str) -> StatSet {
        let mut set = vec![];
        string
            .split('/')
            .map(StatEntry::try_from)
            .map(|result| result.unwrap_or(StatEntry::invalid()))
            .filter(StatEntry::is_valid)
            .for_each(|entry| set.push(entry));

        StatSet { set }
    }

    pub fn base_player_stats() -> StatSet {
        StatSet {
            set: vec![
                StatEntry::new(StatType::Vitality, 0),
                StatEntry::new(StatType::Attack, 5),
                StatEntry::new(StatType::Defence, 0),
            ],
        }
    }

    pub fn get_stat(self: &Self, stat_type: StatType) -> u64 {
        let mut stat = 0;
        self.set
            .iter()
            .filter(|entry| entry.stat_type == stat_type)
            .map(|entry| entry.stat)
            .for_each(|value| stat += value);
        stat
    }
}

impl StatEntry {
    pub fn new(stat_type: StatType, stat: u64) -> StatEntry {
        StatEntry { stat_type, stat }
    }

    pub fn is_valid(self: &Self) -> bool {
        self.stat_type.is_valid()
    }

    pub fn invalid() -> StatEntry {
        StatEntry {
            stat_type: StatType::Invalid,
            stat: 0,
        }
    }
}

impl TryFrom<&str> for StatEntry {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chars = value.chars().collect::<Vec<char>>();
        if chars.len() < 3 {
            return Err("Invalid stat entry char length");
        }
        let stat_type = StatType::from(chars[0]);
        let stat = chars[2..].iter().collect::<String>().parse::<u64>();
        if stat.is_err() {
            return Err("Could not parse stat value");
        }
        Ok(Self {
            stat_type,
            stat: stat.unwrap().clone(),
        })
    }
}

impl StatType {
    pub fn is_valid(self: &Self) -> bool {
        self != &(StatType::Invalid)
    }
}

impl From<char> for StatType {
    fn from(value: char) -> Self {
        match value {
            'v' => Self::Vitality,
            'a' => Self::Attack,
            'd' => Self::Defence,
            _ => Self::Invalid,
        }
    }
}
