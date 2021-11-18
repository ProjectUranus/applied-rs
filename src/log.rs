/// Storage transaction
#[derive(Debug)]
pub enum Transactions {
    /// Inserted count
    Insert(i32),

    /// Inserted a new item type
    InsertNewItem
}