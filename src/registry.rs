use std::collections::HashMap;
use crate::item::Item;

pub struct ItemRegistry {
    pub items: HashMap<String, Item>,
}

impl ItemRegistry {
    pub fn new() -> Self {
        ItemRegistry {
            items: HashMap::new()
        }
    }

    pub fn register(&mut self, item: Item) {
        if self.items.contains_key(&item.id) {
            panic!("Cannot register duplicate key {}", item.id);
        }
        self.items.insert(item.id.to_string(), item);
    }
}