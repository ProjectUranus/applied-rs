use crate::item::{Item};
use std::collections::BTreeMap;
use std::cmp::{max, min, Ordering};
use std::slice::Iter;
use std::hash::Hash;
use std::ops::Add;
use std::convert::TryInto;

pub enum StoredItemTypes {
    Item,
    Fluid
}

pub trait StoredItemType: Sized + Sync + Send + PartialEq + PartialOrd + Ord {
    fn stored_type() -> StoredItemTypes;
}

/// Capacity, max item types
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct StorageCellType(i32, i32);

impl StorageCellType {
    pub fn new(capacity: i32) -> Self {
        StorageCellType(capacity, 63)
    }

    pub fn get_bytes_per_type(&self) -> i32 {
        self.0 / 128
    }
}

pub const CELL_TYPE_1K: StorageCellType = StorageCellType(1024, 63);
pub const CELL_TYPE_4K: StorageCellType = StorageCellType(4096, 63);
pub const CELL_TYPE_16K: StorageCellType = StorageCellType(16384, 63);
pub const CELL_TYPE_64K: StorageCellType = StorageCellType(65536, 63);

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct StoredItem<'a, T: StoredItemType> {
    pub item: &'a T,
    pub count: i32,
}

impl<'a, T: StoredItemType> Clone for StoredItem<'a, T> {
    fn clone(&self) -> Self {
        StoredItem {
            item: self.item.clone(),
            count: self.count.clone()
        }
    }
}

impl<'a, T: StoredItemType> StoredItem<'a, T> {
    pub fn new(item: &'a T, count: i32) -> Self {
        StoredItem {
            item, count
        }
    }
}

impl<T: StoredItemType> Add for StoredItem<'_, T> {
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

#[derive(Debug, PartialEq, Clone, Default, Ord, PartialOrd, Eq)]
pub struct StorageCellConfig {
    pub priority: i32
}

#[derive(Debug, Clone, Eq, Ord)]
pub struct StorageCell<'a, T: StoredItemType> {
    pub config: StorageCellConfig,
    pub stored_types: i32,
    pub bytes_used: i32,
    pub stored_items: BTreeMap<&'a T, StoredItem<'a, T>>,
    pub stored_items_count: i32,
    pub cell_type: &'static StorageCellType,
}

impl<'a, T: StoredItemType> PartialEq<Self> for StorageCell<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.stored_items.eq(&other.stored_items)
    }
}

impl<'a, T: StoredItemType> PartialOrd for StorageCell<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.config.priority.partial_cmp(&other.config.priority)
    }
}

impl<'a, T: StoredItemType> StorageCell<'a, T> {
    pub fn calc_stored_bytes(cell_type: &StorageCellType, stored_items: &BTreeMap<&T, StoredItem<T>>) -> i32 {
        let bytes_per_type = cell_type.get_bytes_per_type();
        let mut bytes: i32 = bytes_per_type * stored_items.keys().count() as i32;
        for (_, stored_item) in stored_items {
            bytes += (stored_item.count as f32 / 8.0f32).ceil() as i32
        }
        bytes
    }

    pub fn calc_free_space(stored_item: &StoredItem<T>, free_bytes: i32) -> i32 {
        let bytes_occupied = (stored_item.count as f32 / 8.0f32).ceil() as i32;
        bytes_occupied * 8 - stored_item.count + free_bytes * 8
    }

    pub fn new(cell_type: &'static StorageCellType) -> Self {
        StorageCell {
            config: Default::default(),
            stored_types: 0,
            bytes_used: 0,
            stored_items: BTreeMap::default(),
            stored_items_count: 0,
            cell_type
        }
    }

    pub fn get_free_bytes(&self) -> i32 {
        self.cell_type.0 - self.bytes_used
    }

    fn get_free_space(&self, item: &StoredItem<T>) -> i32 {
        let bytes_per_type = self.cell_type.get_bytes_per_type();
        let stored_items = &self.stored_items;
        if stored_items.contains_key(item.item) {
            let stored_item = stored_items.get(item.item).unwrap();
            return min(stored_item.count + item.count, Self::calc_free_space(stored_item, self.get_free_bytes()));
        } else {
            if self.bytes_used + bytes_per_type + 1 >= self.cell_type.0 || self.stored_types >= self.cell_type.1 {
                return 0
            }
            let free_space = Self::calc_free_space(item, self.get_free_bytes() - self.cell_type.get_bytes_per_type());
            return min(item.count, free_space);
        }
    }

    pub fn insert(&mut self, item: StoredItem<'a, T>) -> i32 {
        if self.bytes_used == self.cell_type.0
        {
            return 0
        }
        let count = self.get_free_space(&item);
        if count > 0 {
            let to_store = StoredItem {
                item: item.item,
                count
            };
            self.stored_items.insert(item.item, to_store);
            self.refresh_cache();
        }
        count
    }

    pub fn refresh_cache(&mut self) {
        self.stored_types = self.stored_items.keys().count() as i32;
        self.bytes_used = Self::calc_stored_bytes(self.cell_type, &self.stored_items);
        self.stored_items_count = self.stored_items.values().map(|x| x.count).sum();
    }

    pub fn insert_many(&mut self, items: Iter<StoredItem<'a, T>>) -> Vec<i32> {
        let mut vec = vec![];
        for item in items {
            if self.stored_items.contains_key(item.item) || self.stored_types < self.cell_type.1 {
                vec.push(self.insert(item.clone()));
            } else {
                vec.push(0);
            }
        }
        vec
    }

    pub fn take(&mut self, item: &StoredItem<'a, T>) -> i32 {
        if self.stored_items.contains_key(item.item) {
            let stored_item = self.stored_items.get_mut(item.item).unwrap();
            let count = min(stored_item.count, item.count);
            stored_item.count -= count;
            if stored_item.count == 0 {
                self.stored_items.remove(item.item);
            }
            self.refresh_cache();
            return count;
        }
        0
    }

    pub fn take_many(&mut self, items: Iter<StoredItem<'a, T>>) -> Vec<i32> {
        items.map(|x| self.take(x)).collect()
    }

}