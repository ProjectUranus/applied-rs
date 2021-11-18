use crate::registry::ItemRegistry;
use crate::item::{Item};
use crate::storage::{StorageCell, CELL_TYPE_1K, StoredItem, CELL_TYPE_64K};
use std::time::Instant;
use std::slice::Iter;

#[cfg(test)]
mod test {
    use crate::storage::{StorageCell, CELL_TYPE_1K, StoredItem, CELL_TYPE_16K};
    use crate::item::{Item};
    use crate::grid::Grid;

    #[test]
    fn test_free_space() {
        let item = Item::new("minecraft:stone");
        let stored_item = StoredItem::new(&item, 15);
        assert_eq!(StorageCell::calc_free_space(&stored_item, 8), 65);
    }

    #[test]
    fn test_insert() {
        let mut cell = StorageCell::new(&CELL_TYPE_1K);
        let mut items: Vec<Item> = vec![];
        for i in 0..64 {
            items.push(Item::new(i.to_string().as_str()));
        }
        for i in 0..64 {
            cell.insert(StoredItem::new(&items[i], 5));
        }
        assert_eq!(cell.stored_types, 63);
        assert_eq!(cell.stored_items_count, 63 * 5);
        cell.insert(StoredItem::new(&items[0], 40960));
        assert_eq!(cell.stored_types, 63);
        assert_eq!(cell.bytes_used, 1024);
    }

    #[test]
    fn test_grid() {
        let mut grid = Grid::default();
        for i in 0..3 {
            grid.insert_storage_cell(StorageCell::new(&CELL_TYPE_16K));
        }
        let mut items: Vec<Item> = vec![];
        for i in 0..64 {
            items.push(Item::new(i.to_string().as_str()));
        }
        for i in 0..64 {
            assert_eq!(grid.insert(StoredItem::new(&items[i], 1024)), 0);
        }
        assert_eq!(grid.stored_items_cache.len(), items.len());
        assert_eq!(grid.take(StoredItem::new(&items[0], 5)), 5);

    }

    #[test]
    fn test_grid_union() {
        let mut grid = Grid::default();
        for i in 0..3 {
            grid.insert_storage_cell(StorageCell::new(&CELL_TYPE_16K));
        }
        let mut items: Vec<Item> = vec![];
        for i in 0..64 {
            items.push(Item::new(i.to_string().as_str()));
        }
        for i in 0..64 {
            assert_eq!(grid.insert(StoredItem::new(&items[i], 1024)), 0);
        }

        let mut grid2 = Grid::default();
        for i in 0..3 {
            grid2.insert_storage_cell(StorageCell::new(&CELL_TYPE_16K));
        }
        grid2.insert_many((0..64usize).into_iter().map(|i| StoredItem::new(&items[i], 1024)).collect());
        grid = grid + grid2;
        grid.sort();
        std::fs::write("grid.json", serde_json::to_string_pretty(&grid).unwrap());
        assert_eq!(grid.stored_items_cache.len(), items.len());
        assert_eq!(grid.take(StoredItem::new(&items[0], 5)), 5);
    }
}

fn main() {
    let mut registry = ItemRegistry::new();
    registry.register(Item::new("minecraft:stone"));
    let mut cell = StorageCell::new(&CELL_TYPE_64K);
    let mut items: Vec<Item> = vec![];
    for i in 0..63 {
        items.push(Item::new(i.to_string().as_str()));
    }
    let stored_items: Vec<StoredItem<Item>> = items.iter().map(|x| StoredItem::new(x, 320)).collect();
    let start = Instant::now();
    println!("{:?}", cell.insert_many(stored_items.iter()));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    let item = Item::new("minecraft:stone");
    let result = cell.insert(StoredItem::new(&item, 8192));
    println!("Insertion transactions: {:?}", result);
}

pub fn register_item() {

}

pub mod item;
pub mod tag;
pub mod registry;
pub mod grid;
pub mod storage;
pub mod cache;
pub mod log;
pub mod fluid;