use nbt::Blob;
use std::ops::Add;
use std::cmp::Ordering;
use std::hash::Hash;
use crate::storage::{StoredItemType, StoredItemTypes};
use std::convert::TryInto;

/// Representing a "definition stack"
#[derive(PartialEq, Debug)]
pub struct Item {
    pub id: String,
    pub damage: i32,
    pub max_stack_size: i32,
    pub tag: nbt::Blob,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id).then(self.damage.cmp(&other.damage))
    }
}

impl StoredItemType for Item {
    fn stored_type() -> StoredItemTypes {
        StoredItemTypes::Item
    }
}

impl Item {
    pub fn new(id: &str) -> Self {
        Item {
            id: id.to_string(),
            damage: 0,
            max_stack_size: 64,
            tag: Blob::default()
        }
    }
}