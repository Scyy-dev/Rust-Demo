use std::collections::BTreeMap;

#[derive(PartialEq, Eq)]
pub enum StatType {
    Vitality,
    Attack,
    Defence,
}

pub struct StatEntry {
    stat_type: StatType,
    stat: u64,
}

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
}
