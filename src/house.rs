// src/house.rs
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomId(pub usize);

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub exits: Vec<(Direction, RoomId)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North, South, East, West,
}

pub struct House {
    rooms: HashMap<RoomId, Room>,
}

impl House {
    pub fn new() -> Self {
        House { rooms: HashMap::new() }
    }

    pub fn add_room(&mut self, id: RoomId, name: &str) {
        self.rooms.insert(id, Room { name: name.to_string(), exits: Vec::new() });
    }

    pub fn connect(&mut self, from: RoomId, dir: Direction, to: RoomId) {
        self.rooms.get_mut(&from).unwrap().exits.push((dir, to));
    }

    pub fn room(&self, id: RoomId) -> &Room {
        &self.rooms[&id]
    }
}
