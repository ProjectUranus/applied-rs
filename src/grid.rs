use crate::storage::{StorageCell, StoredItemType, StoredItem};
use crate::item::Item;
use std::collections::{BTreeMap, BTreeSet};
use serde::Serialize;
use std::ops::Add;

pub struct InsertBatch {

}

/// Network grid
#[derive(Debug, Serialize)]
pub struct Grid<'a, T: StoredItemType> {
    pub storage_cells: Vec<StorageCell<'a, T>>,

    pub stored_items_cache: BTreeMap<&'a T, StoredItem<'a, T>>,

    pub stored_items_priority_cache: BTreeMap<&'a T, Vec<usize>>
}

impl<'a, T: StoredItemType> Default for Grid<'a, T> {
    fn default() -> Self {
        Grid {
            storage_cells: Vec::default(),
            stored_items_cache: BTreeMap::default(),
            stored_items_priority_cache: BTreeMap::default()
        }
    }
}

impl<'a, T: StoredItemType> Grid<'a, T> {
    pub fn sort(&mut self) {
        self.refresh_cache();
        let stored_items: Vec<StoredItem<'a, T>> = self.stored_items_cache.iter().map(|x| x.1.clone()).collect();
        self.stored_items_cache.clear();
        self.stored_items_priority_cache.clear();
        for x in self.storage_cells.iter_mut() {
            x.clear();
        }
        self.storage_cells.sort();
        self.insert_many(stored_items);
        self.refresh_cache();
    }

    pub fn refresh_cache(&mut self) {
        self.stored_items_cache.clear();
        self.stored_items_priority_cache.clear();
        for (cell_index, cell) in self.storage_cells.iter().enumerate() {
            for (item, stored_item) in cell.stored_items.iter() {
                if self.stored_items_cache.contains_key(item) {
                    let mut cached_item = self.stored_items_cache.get_mut(item).unwrap();
                    cached_item.count += stored_item.count;
                    let mut priority_list = self.stored_items_priority_cache.get_mut(item).unwrap();
                    priority_list.push(cell_index);
                } else {
                    self.stored_items_cache.insert(item, stored_item.clone());
                    self.stored_items_priority_cache.insert(item, vec![cell_index]);
                }
            }
        }
    }

    pub fn insert_storage_cell(&mut self, cell: StorageCell<'a, T>) {
        self.storage_cells.push(cell);
        self.storage_cells.sort();
        self.refresh_cache();
    }

    fn do_insert_many(&mut self, items: Vec<StoredItem<'a, T>>) -> Vec<i32> {
        let mut item_keys = BTreeSet::new();
        let mut remaining_count: Vec<i32> = items.iter().map(|x| x.count).collect();
        for x in items.iter() {
            item_keys.insert(x.item);
            remaining_count.push(x.count);
        }
        let item_keys = item_keys;

        let priority_caches: BTreeMap<&'a T, &Vec<usize>> =
            self.stored_items_priority_cache
                .iter()
                .filter(|(item, _)| item_keys.contains(*item))
                .map(|(item, value)| (*item, value))
                .collect();

        for (i, item) in items.iter().enumerate() {
            let mut count = remaining_count.get_mut(i).unwrap();
            if priority_caches.contains_key(item.item) {
                let priority_list = *priority_caches.get(item.item).unwrap();
                for cell_index in priority_list.iter() {
                    if *count > 0 {
                        if let Some(storage_cell) = self.storage_cells.get_mut(*cell_index) {
                            if !storage_cell.is_full() {
                                let to_insert = StoredItem {
                                    item: item.item,
                                    count: count.clone()
                                };
                                *count -= storage_cell.insert(to_insert);
                            }
                        }
                    }
                }
            }
            // Still remain items to be inserted
            if *count > 0 {
                for cell in self.storage_cells.iter_mut() {
                    if *count == 0 {
                        break;
                    }
                    if !cell.is_full() {
                        let to_insert = StoredItem {
                            item: item.item,
                            count: count.clone()
                        };
                        *count -= cell.insert(to_insert);
                    }
                }
            }
        }

        remaining_count
    }

    pub fn insert_many(&mut self, items: Vec<StoredItem<'a, T>>) -> Vec<i32> {
        let ret = self.do_insert_many(items);
        self.refresh_cache();
        ret
    }

    fn do_insert(&mut self, item: StoredItem<'a, T>) -> i32 {
        let mut count = item.count;
        if let Some(priority_list) = self.stored_items_priority_cache.get(item.item) {
            for cell_index in priority_list.iter() {
                if count > 0 {
                    if let Some(storage_cell) = self.storage_cells.get_mut(*cell_index) {
                        if !storage_cell.is_full() {
                            let to_insert = StoredItem {
                                item: item.item,
                                count
                            };
                            count -= storage_cell.insert(to_insert);
                        }
                    }
                }
            }
        }
        // Still remain items to be inserted
        if count > 0 {
            for cell in self.storage_cells.iter_mut() {
                if count == 0 {
                    return 0;
                }
                if !cell.is_full() {
                    let to_insert = StoredItem {
                        item: item.item,
                        count
                    };
                    count -= cell.insert(to_insert);
                }
            }
        }
        count
    }

    pub fn insert(&mut self, item: StoredItem<'a, T>) -> i32 {
        let ret = self.do_insert(item);
        self.refresh_cache();
        ret
    }

    fn do_take(&mut self, item: StoredItem<'a, T>) -> i32 {
        let mut count = item.count;
        let mut taken_count = 0;
        if let Some(priority_list) = self.stored_items_priority_cache.get(item.item) {
            for cell_index in priority_list.iter().rev() { // Reverse order take out
                if count > 0 {
                    if let Some(storage_cell) = self.storage_cells.get_mut(*cell_index) {
                        let result = storage_cell.take(&item);
                        taken_count += result;
                        count -= result;
                    }
                }
            }
        }
        taken_count
    }

    pub fn take(&mut self, item: StoredItem<'a, T>) -> i32 {
        let ret = self.do_take(item);
        self.refresh_cache();
        ret
    }

    pub fn union(&mut self, other: Self) {
        for x in other.storage_cells.into_iter() {
            self.storage_cells.push(x);
        }
        self.storage_cells.sort();
        self.refresh_cache();
    }
}

impl<'a, T: StoredItemType> Add for Grid<'a, T> {
    type Output = Grid<'a, T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut grid = self;
        grid.union(rhs);
        grid
    }
}

pub struct GridNetwork<'a> {
    item_grid: Grid<'a, Item>
}
