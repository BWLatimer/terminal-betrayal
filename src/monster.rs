//monster.rs
use crate::house::RoomId;
use std::collections::HashMap;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum MonsterError {
    #[error("no such monster: {0:?}")]
    MonsterNotFound(MonsterId),
}

impl MonsterRegistry {
    pub fn new() -> Self {
        MonsterRegistry { monsters: HashMap::new() }
    }

    pub fn add_monster(&mut self, id: MonsterId, name: &str, room: RoomId, health: i32) {
        self.monsters.insert(id, Monster { id, name: name.to_string(), current_room: room, health});
    }
    
    pub fn monster(&self, id: MonsterId) -> Result<&Monster, MonsterError> {
        self.monsters.get(&id).ok_or(MonsterError::MonsterNotFound(id))
    }

    pub fn monsters_in(&self, room: RoomId) -> Vec<&Monster> {
        self.monsters.values().filter(|m| m.current_room == room).collect()
    }

    pub fn all_monsters_mut(&mut self, room: RoomId) -> Vec<&mut Monster> {
        self.monsters.values_mut().filter(|m| m.current_room == room).collect()
    }
}
