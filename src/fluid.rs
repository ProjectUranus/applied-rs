use nbt::Blob;
use std::ops::Add;
use std::cmp::Ordering;
use std::hash::Hash;
use crate::storage::{StoredItemType, StoredItemTypes};
use std::convert::TryInto;
use serde::{Serialize, Deserialize, Serializer};
use serde::ser::{Error};

/// Representing a "definition stack"
#[derive(PartialEq, Debug, Deserialize)]
pub struct Fluid {
    pub id: String,
    pub tag: nbt::Blob,
}

impl PartialOrd for Fluid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Fluid {}

impl Ord for Fluid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Serialize for Fluid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.id)
    }
}

impl StoredItemType for Fluid {
    fn stored_type() -> StoredItemTypes {
        StoredItemTypes::Fluid
    }
}

impl Fluid {
    pub fn new(id: &str) -> Self {
        Fluid {
            id: id.to_string(),
            tag: Blob::default()
        }
    }
}