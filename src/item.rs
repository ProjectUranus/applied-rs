use nbt::Blob;
use std::ops::Add;
use std::cmp::Ordering;

/// Representing a "definition stack"
#[derive(PartialEq)]
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct StoredItem<'a> {
    pub item: &'a Item,
    pub count: i32,
}

impl<'a> StoredItem<'a> {
    pub fn new(item: &'a Item, count: i32) -> Self {
        StoredItem {
            item, count
        }
    }
}

impl Add for StoredItem<'_> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.item == rhs.item {
            StoredItem {
                item: self.item,
                count: self.count + rhs.count
            }
        } else {
            self
        }
    }
}