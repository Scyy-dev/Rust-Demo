use std::fmt::Display;

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

impl StatType {
    pub fn short_name(&self) -> &str {
        match self {
            Self::Vitality => "VIT",
            Self::Attack => "ATT",
            Self::Defence => "DEF",
            Self::Invalid => "INVALID",
        }
    }
}

impl StatSet {
    pub fn new(set: Vec<StatEntry>) -> StatSet {
        StatSet { set }
    }

    pub fn zeros() -> StatSet {
        StatSet { set: vec![] }
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

    pub fn get_stat(&self, stat_type: &StatType) -> u64 {
        let mut stat = 0;
        self.set
            .iter()
            .filter(|entry| &entry.stat_type == stat_type)
            .map(|entry| entry.stat)
            .for_each(|value| stat += value);
        stat
    }
}

impl TryFrom<&str> for StatSet {
    type Error = String;

    // To ensure all errors from each StatEntry are grabbed, errors are compiled into a list
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut set = vec![];
        let errs = value
            .split('/')
            .map(StatEntry::try_from)
            .fold(vec![], |mut errs, result| match result {
                Ok(entry) => {
                    set.push(entry);
                    errs
                }
                Err(err) => {
                    errs.push(err);
                    errs
                }
            });
        if errs.len() == 0 {
            return Ok(StatSet { set });
        } else {
            return Err(errs.join(", "));
        }
    }
}

impl Display for StatSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats: Vec<String> = self
            .set
            .iter()
            .filter(|stat| stat.is_valid() && stat.stat != 0)
            .map(|entry| format!("{}", entry))
            .collect();
        let stats = stats.join(", ");
        write!(f, "Stats: [{}]", stats)
    }
}

impl StatEntry {
    pub fn new(stat_type: StatType, stat: u64) -> StatEntry {
        StatEntry { stat_type, stat }
    }

    pub fn is_valid(&self) -> bool {
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
            stat: stat.unwrap(),
        })
    }
}

impl Display for StatEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.stat_type.short_name(), self.stat)
    }
}

impl StatType {
    pub fn is_valid(&self) -> bool {
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

impl Display for StatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[test]
fn parse_stat_set_test() {
    let result = StatSet::try_from("a:1/d:2/v:3");
    assert!(result.is_ok());

    let set = result.unwrap();

    let att = set.get_stat(&StatType::Attack);
    assert_eq!(1, att);

    let def = set.get_stat(&StatType::Defence);
    assert_eq!(2, def);

    let vit = set.get_stat(&StatType::Vitality);
    assert_eq!(3, vit);

    let invalid = set.get_stat(&StatType::Invalid);
    assert_eq!(0, invalid);
}
