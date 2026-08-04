//item.rs
use thiserror::Error;
use std::collections::HashMap;
use crate::house::RoomId;
#[derive(Debug, Clone)]
pub struct Item {
    pub id: ItemId,
    pub name: String, 
    pub description: String,
    //TODO: eventually - add effects, modifiers, or properties
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub usize);

#[derive(Debug, Error)]
pub enum ItemError {
    #[error("no such item: {0:?}")]
    ItemNotFound(ItemId),
    #[error("no items found in {0:?}")]
    NoItemsInRoom(RoomId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Owner {
    Room(RoomId),
    Player,
    // Monster(MonsterId)
    Nowhere, //unnassigned items
}

pub struct ItemRegistry {
    pub items: HashMap<ItemId, Item>,
    pub ownership: HashMap<ItemId, Owner>,
}

impl ItemRegistry {
    
    pub fn new() -> Self {
        ItemRegistry {
            items: HashMap::new(),
            ownership: HashMap::new(),
        }
    }
    pub fn add_item(&mut self, id: ItemId, name: &str, description: &str) {
        self.items.insert(id, Item{id, name: name.to_string(), description: description.to_string()});
    }

    pub fn assign(&mut self, item_id: ItemId, owner: Owner) {
        self.ownership.insert(item_id, owner);
    }

    pub fn owner_of(&self, item_id: ItemId) -> Option<Owner> {
        self.ownership.get(&item_id).copied()
    }

    pub fn items_owned_by(&self, owner: Owner) -> Vec<ItemId> {
        self.ownership.iter()
            .filter(|(_, o)| **o == owner)
            .map(|(id, _)| *id)
            .collect()
    }
    
    pub fn name_of(&self, item_id: ItemId) -> Option<&str> {
    self.items.get(&item_id).map(|item| item.name.as_str())
    }
}
