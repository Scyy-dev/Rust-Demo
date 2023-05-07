use std::fmt::Display;

use crate::{
    entity::action::ActionSet,
    stat::{StatSet, StatType},
};

#[derive(Debug)]
pub struct Item {
    name: String,
    stats: StatSet,
    actions: ActionSet,
}

#[derive(Debug)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Item {
    pub fn new(name: String, stats: StatSet, actions: ActionSet) -> Item {
        Item {
            name,
            stats,
            actions,
        }
    }

    pub fn get_stat(&self, stat_type: &StatType) -> u64 {
        self.stats.get_stat(stat_type)
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl TryFrom<&str> for Item {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let args = value.split('|').collect::<Vec<&str>>();
        if args.len() != 3 {
            return Err("Invalid item format - must be [Name]|[StatSet]|[ActionSet]".to_string());
        }

        let name = unsafe { args.get_unchecked(0) };

        // Use of unsafe for efficiency - we did a bounds check earlier
        let stats: &str = unsafe { args.get_unchecked(1) };
        let stats = StatSet::try_from(stats);
        if let Err(err) = stats {
            return Err(err);
        }

        let actions: &str = unsafe { args.get_unchecked(2) };
        let actions = ActionSet::try_from(actions);
        if let Err(err) = actions {
            return Err(err);
        }

        Ok(Item {
            name: name.to_string(),
            stats: stats.unwrap(),
            actions: actions.unwrap(),
        })
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n  {}\n  {}", self.name, self.stats, self.actions)
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
        write!(f, "\nInventory:\n\n")?;
        for (index, item) in self.items.iter().enumerate() {
            write!(f, "[{}]: {}\n", index, item)?;
        }
        Ok(())
    }
}

#[test]
fn parse_item_test() {
    let result = Item::try_from("Staff of Healing|a:1/d:2/v:3|h");
    assert!(result.is_ok());

    let item = result.unwrap();

    assert_eq!("Staff of Healing", item.name());
    assert_eq!(1, item.get_stat(&StatType::Attack));
    assert_eq!(2, item.get_stat(&StatType::Defence));
    assert_eq!(3, item.get_stat(&StatType::Vitality));
}

#[test]
fn parse_inventory_test() {
    let raw_inventory = "Staff of Attack|a:3/d:2/v:1|a\nShield of Defence|a:1/d:5/v:2|";

    let result = Inventory::try_from(raw_inventory);
    assert!(result.is_ok());

    let inv = result.unwrap();
    assert_eq!(4, inv.get_stat(&StatType::Attack));
    assert_eq!(7, inv.get_stat(&StatType::Defence));
    assert_eq!(3, inv.get_stat(&StatType::Vitality));
    assert_eq!(0, inv.get_stat(&StatType::Invalid));
}
