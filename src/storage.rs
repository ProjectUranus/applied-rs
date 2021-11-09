use crate::item::{StoredItem, Item};
use std::collections::BTreeMap;
use std::cmp::{max};

/// Capacity, max item types
#[derive(Ord, PartialOrd, Eq, PartialEq)]
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

pub struct StorageCell<'a> {
    pub stored_types: i32,
    pub bytes_used: i32,
    pub stored_items: BTreeMap<&'a Item, StoredItem<'a>>,
    pub stored_items_count: i32,
    pub cell_type: &'static StorageCellType,
}

impl<'a> StorageCell<'a> {
    pub fn calc_stored_bytes(cell_type: &StorageCellType, stored_items: &BTreeMap<&Item, StoredItem>) -> i32 {
        let bytes_per_type = cell_type.get_bytes_per_type();
        let mut bytes: i32 = bytes_per_type * stored_items.keys().count() as i32;
        for (_, stored_item) in stored_items {
            bytes += (stored_item.count as f32 / 8.0f32).ceil() as i32
        }
        bytes
    }

    pub fn new(cell_type: &'static StorageCellType) -> Self {
        StorageCell {
            stored_types: 0,
            bytes_used: 0,
            stored_items: BTreeMap::new(),
            stored_items_count: 0,
            cell_type
        }
    }

    /// Simulate insertion operation without actually operating the storage cell
    ///
    /// # Arguments
    ///
    /// * `item`: Items to insert
    ///
    /// returns: inserted item count
    ///
    /// # Examples
    ///
    /// ```
    /// 1015 + 8 + 5
    /// ```
    fn insert_simulate(&self, item: &StoredItem) -> i32 {
        let bytes_per_type = self.cell_type.get_bytes_per_type();
        let mut stored_items = self.stored_items.clone();
        let mut count = item.count;
        if stored_items.contains_key(item.item) {
            count += stored_items.get(item.item).unwrap().count;
        } else {
            if self.bytes_used + bytes_per_type + 1 >= self.cell_type.0 || self.stored_types >= self.cell_type.1 {
                return 0
            }
            stored_items.insert(item.item, item.clone());
        }
        let stored_item = stored_items.get_mut(item.item).unwrap();
        stored_item.count = count;

        let stored_bytes = Self::calc_stored_bytes(self.cell_type, &stored_items);
        return if stored_bytes > self.cell_type.0 {
            max(count - (stored_bytes - self.cell_type.0) * 8, 0)
        } else {
            count
        }
    }

    pub fn insert(&mut self, item: StoredItem<'a>) -> i32 {
        if self.bytes_used == self.cell_type.0
        {
            return 0
        }
        let count = self.insert_simulate(&item);
        if count > 0 {
            let to_store = StoredItem {
                item: item.item,
                count
            };
            self.stored_items.insert(item.item, to_store);
            self.stored_types = self.stored_items.keys().count() as i32;
            self.bytes_used = Self::calc_stored_bytes(self.cell_type, &self.stored_items);
            self.stored_items_count = self.stored_items.values().map(|x| x.count).sum();
        }
        count
    }
}