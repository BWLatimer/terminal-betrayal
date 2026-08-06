//room_event.rs
use crate::house::{RoomId, HouseError};
use crate::monster::MonsterId;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum RoomEventKind {
    SpawnMonster(MonsterId),
    //add others later like omens, clues, items, etc
}

pub struct RoomEventRegistry {
    events: HashMap<RoomId, RoomEventKind>,
}

impl RoomEventRegistry {
    pub fn new() -> Self {
        RoomEventRegistry { events: HashMap::new() }
    }
    
    pub fn add_event(&mut self, room: RoomId, kind: RoomEventKind) {
        self.events.insert(room, kind);
    }

    pub fn room_check(&mut self, room: RoomId) -> Option<RoomEventKind> {
        self.events.remove(&room)    
    }
}
