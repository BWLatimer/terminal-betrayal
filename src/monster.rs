//monster.rs
use crate::house::{RoomId, House};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonsterId(pub usize);

#[derive(Debug, Clone)]
pub struct Monster {
    pub id: MonsterId,
    pub name: String,
    pub current_room: Option<RoomId>,
    pub strength: u32,
    pub health: u32,
    pub speed: u32
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

    pub fn add_monster(&mut self, id: MonsterId, name: &str, room: Option<RoomId>, health: u32, strength: u32, speed: u32) {
        self.monsters.insert(id, Monster { id, name: name.to_string(), current_room: room, health, strength, speed});
    }

    pub fn remove_monster(&mut self, id: MonsterId) {
        self.monsters.remove(&id);
    }
    
    pub fn monster(&self, id: MonsterId) -> Result<&Monster, MonsterError> {
        self.monsters.get(&id).ok_or(MonsterError::MonsterNotFound(id))
    }

    pub fn monsters_in(&self, room: RoomId) -> Vec<&Monster> {
        self.monsters.values().filter(|m| m.current_room == Some(room)).collect()
    }
    
    pub fn move_to(&mut self, id: MonsterId, room: RoomId) {
        if let Some(monster) = self.monsters.get_mut(&id) {
            monster.current_room = Some(room);
        }
    }

    pub fn monster_mut(&mut self, id: MonsterId) -> Option<&mut Monster> {
        self.monsters.get_mut(&id)
    }

    pub fn all_monsters_mut(&mut self) -> Vec<&mut Monster> {
        self.monsters.values_mut().collect()
    }
}
