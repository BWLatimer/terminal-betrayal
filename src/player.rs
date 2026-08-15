/// src/player.rs
use crate::house::{House, RoomId, Direction};
use crate::item::{ItemId, ItemRegistry, Owner, ItemError};
use thiserror::Error;
use serde::Deserialize;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub current_room: RoomId,
    pub found_item: Option <ItemId>,
    pub health: i32,
    pub strength: i32,
    pub speed: i32,
//    pub knowledge: i32,
//    pub sanity: i32,
//    pub magic: i32,
    pub moves_remaining: i32,
    pub max_health: i32,
}

#[derive(Deserialize)]
pub struct PlayerConfig {
    pub name: String,
    pub health: i32,
    pub strength: i32,
    pub speed: i32,
//    pub knowledge: i32,
//    pub sanity: i32,
//    pub magic: i32,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("check the toml file")]
    InvalidConfig,
}


#[derive(Debug, Error)]
pub enum MoveError {
    #[error("You can't go this way: {0:?}")]
    InvalidMovement(Direction),

    #[error("No moves remaining")]
    NoMovesRemaining,
}

impl Player {
    pub fn new(name: String, start: RoomId, health: i32, strength: i32, speed: i32) -> Player {
        let speed_to_moves = speed.clone();
        let player_max_health = health.clone();
        Player { name, current_room: start, found_item: None, health, strength, speed, moves_remaining: speed_to_moves, max_health: player_max_health }
    }

    pub fn move_player(&mut self, house: &House, dir: Direction) -> Result <(), MoveError> {
        let room = house.room(self.current_room)
            .expect("player's room should always be valid");
        let found = room.exits.iter().find(|(exit_dir, _)| *exit_dir == dir);
        match found {
            Some((_, target)) => {
                self.current_room = *target;
                Ok(())
            }
            None => Err(MoveError::InvalidMovement(dir))
        }
    }

    pub fn search_room(&mut self, registry: &ItemRegistry) -> Result<(), ItemError> {
        let items_here = registry.items_owned_by(Owner::Room(self.current_room));
        match items_here.first() {
            Some(item_id) => {
                self.found_item = Some(*item_id);
                Ok(())
            }
            None => Err(ItemError::NoItemsInRoom(self.current_room))
            }
    }
}
