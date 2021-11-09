use crate::registry::ItemRegistry;
use crate::item::{Item, StoredItem};
use crate::storage::{StorageCell, CELL_TYPE_1K};
use std::time::Instant;

#[cfg(test)]
mod test {
    use crate::storage::{StorageCell, CELL_TYPE_1K};
    use crate::item::{Item, StoredItem};

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
}

fn main() {
    let mut registry = ItemRegistry::new();
    registry.register(Item::new("minecraft:stone"));
    let mut cell = StorageCell::new(&CELL_TYPE_1K);
    let mut items: Vec<Item> = vec![];
    for i in 0..64 {
        items.push(Item::new(i.to_string().as_str()));
    }

    let start = Instant::now();
    for i in 0..64 {
        cell.insert(StoredItem::new(&items[i], 5));
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    let item = Item::new("minecraft:stone");
    let inserted = cell.insert(StoredItem::new(&item, 8192));
    println!("Inserted {} items", inserted);
}

pub fn register_item() {

}

pub mod item;
pub mod tag;
pub mod registry;
pub mod grid;
pub mod storage;