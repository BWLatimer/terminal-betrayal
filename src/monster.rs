//monster.rs
use crate::house::RoomId;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonsterId(pub usize);

#[derive(Debug, Clone)]
pub struct Monster {
    pub id: MonsterId,
    pub name: String,
    pub current_room: RoomId,
    pub health: i32,
    //TODO: add in other identifiers or attributes
}

pub struct MonsterRegistry {
    monsters: HashMap<MonsterId, Monster>,
}

impl MonsterRegistry {
    pub fn new() -> Self {
        MonsterRegistry { monsters: HashMap::new() }
    }

    pub fn add_monster(&mut self, id: MonsterId, name: &str, room: RoomId, health: i32) {
        self.monsters.insert(id, Monster { id, name: name.to_string(), current_room: room, health});
    }
    
    pub fn monsters_in(&self, room: RoomId) -> Vec<&Monster> {
        self.monsters.values().filter(|m| m.current_room == room).collect()
    }
}
