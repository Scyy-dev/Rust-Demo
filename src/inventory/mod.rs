use crate::{
    entity::action::ActionSet,
    stat::{StatSet, StatType},
};

#[derive(Debug)]
pub struct Item {
    stats: StatSet,
    actions: ActionSet,
}

impl Item {
    pub fn new(stats: StatSet, actions: ActionSet) -> Item {
        Item { stats, actions }
    }

    pub fn apply_stats(self: &Self, stat_type: StatType, base: u64) -> u64 {
        base + self.stats.get_stat(stat_type)
    }
}

impl TryFrom<&str> for Item {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let args = value.split('|').collect::<Vec<&str>>();
        if args.len() != 2 {
            return Err("Invalid item format - must be [StatSet]|[ActionSet]".to_string());
        }

        // Use of unsafe for efficiency - we did a bounds check earlier
        let stats: &str = unsafe { args.get_unchecked(0) };
        let stats = StatSet::try_from(stats);
        if let Err(err) = stats {
            return Err(err);
        }

        let actions: &str = unsafe { args.get_unchecked(1) };
        let actions = ActionSet::try_from(actions);
        if let Err(err) = actions {
            return Err(err);
        }

        Ok(Item {
            stats: stats.unwrap(),
            actions: actions.unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn new(items: Vec<Item>) -> Inventory {
        Inventory { items }
    }

    pub fn get_stat(self: &Self, stat_type: &StatType) -> u64 {
        todo!()
    }
}

impl TryFrom<&str> for Inventory {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut items = vec![];

        let errs = value
            .split("\n")
            .map(Item::try_from)
            .fold(vec![], |mut errs, result| match result {
                Ok(item) => {
                    items.push(item);
                    errs
                }
                Err(err) => {
                    errs.push(err);
                    errs
                }
            });

        if errs.len() == 0 {
            return Ok(Inventory { items });
        } else {
            return Err(errs.join(", "));
        }
    }
}
