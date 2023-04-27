use crate::stat::StatType;

use super::item::Item;

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
