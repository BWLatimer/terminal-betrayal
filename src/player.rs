/// src/player.rs
use crate::house::{House, RoomId, Direction};
use thiserror::Error;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub current_room: RoomId,
        //TODO: what other things does a player need to track? ex) 
        //health, sanity, strength, inventory, location, etc.
}


#[derive(Debug, Error)]
pub enum MoveError {
    #[error("You can't go this way: {0:?}")]
    InvalidMovement(Direction),
}

impl Player {
    pub fn new_player() -> Player {
        Player {name: "Adventurer".to_string(), current_room: RoomId(0) }
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
            None => Err(MoveError::InvalidMovement(dir)),
        }
    }
}
