/// src/player.rs
use crate::house::{House, RoomId, Direction};
use crate::game_state::GameState;
use crate::item::{ItemId, ItemRegistry, Owner, ItemError};
use thiserror::Error;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub current_room: RoomId,
        //TODO: what other things does a player need to track? ex) 
        //health, sanity, strength, inventory, location, etc.
    pub inventory: Vec<ItemId>,
    pub found_item: ItemId,
}


#[derive(Debug, Error)]
pub enum MoveError {
    #[error("You can't go this way: {0:?}")]
    InvalidMovement(Direction),
}

impl Player {
    pub fn new(name: &str, start: RoomId, inventory: Vec<ItemId>, empty: ItemId ) -> Player {
        Player {name: name.to_string(), current_room: start, inventory: inventory, found_item: empty }
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
        let found_item = registry.ownership.iter().find(|(_, item_owner)| *item_owner == &Owner::Room(self.current_room));
        match found_item {
            Some((target, _)) => {
                self.found_item = *target;
                Ok(())
            }
            None => Err(ItemError::NoItemsInRoom(self.current_room))
            }
    }

    pub fn pick_up_item(&mut self, state: &mut GameState) -> Result <(), ItemError> {
        state.pick_up_item(self.found_item)
    }

}
