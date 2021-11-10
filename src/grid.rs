use crate::storage::{StorageCell, StoredItemType, StoredItem};
use crate::item::Item;
use std::collections::BTreeMap;

pub struct InsertBatch {

}

/// Network grid
#[derive(Debug, Default)]
pub struct Grid<'a, T: StoredItemType> {
    storage_cells: Vec<StorageCell<'a, T>>,

    stored_items_cache: BTreeMap<&'a T, StoredItem<'a, T>>,
}

impl<'a, T: StoredItemType> Grid<'a, T> {
    pub fn refresh_cache(&mut self) {
        self.stored_items_cache = self.storage_cells
            .iter()
            .flat_map(|x| x.stored_items.iter().map(|y| (y.0.clone(), y.1.clone())))
            .collect();
    }

    pub fn insert_storage_cell(&mut self, cell: StorageCell<'a, T>) {
        self.storage_cells.push(cell);
        self.storage_cells.sort();
        self.refresh_cache();
    }

    pub fn insert(&mut self, item: T) {

    }
}

pub struct GridNetwork<'a> {
    item_grid: Grid<'a, Item>
}
