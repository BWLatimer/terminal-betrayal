// game-state.rs
use crate::house::{House, Room, Direction, HouseError};
use crate::player::{Player, MoveError};

pub struct GameState {
    pub house: House,
    pub player: Player,
}

impl GameState {
    pub fn new(house: House, player: Player,) -> GameState {
        GameState {house, player}
    }

    pub fn current_room(&self) -> Result<&Room, HouseError> {
        self.house.room(self.player.current_room)
    }

    pub fn move_player(&mut self, dir: Direction) -> Result<(), MoveError> {
        self.player.move_player(&self.house, dir)
    }
}


