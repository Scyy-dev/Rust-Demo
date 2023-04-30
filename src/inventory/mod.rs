use std::fmt::Display;

use crate::{
    entity::action::ActionSet,
    stat::{StatSet, StatType},
};

#[derive(Debug)]
pub struct Item {
    stats: StatSet,
    actions: ActionSet,
}

#[derive(Debug)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Item {
    pub fn new(stats: StatSet, actions: ActionSet) -> Item {
        Item { stats, actions }
    }

    pub fn get_stat(&self, stat_type: &StatType) -> u64 {
        self.stats.get_stat(stat_type)
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

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ITEM_NAME_TODO:\n{}{}", &self.stats, &self.actions)
    }
}

impl Inventory {
    pub fn new(items: Vec<Item>) -> Inventory {
        Inventory { items }
    }

    pub fn get_stat(&self, stat_type: &StatType) -> u64 {
        self.items
            .iter()
            .fold(0, |stat, item| stat + item.get_stat(stat_type))
    }

    pub fn get_actions(&self) -> ActionSet {
        ActionSet::merge_all(self.items.iter().map(|item| &item.actions).collect())
    }

    pub fn items(&self) -> &[Item] {
        &self.items
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

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Inventory:\n")?;
        for item in &self.items {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}
