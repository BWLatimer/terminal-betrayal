// game-state.rs
use crate::house::{House, Room, RoomId, Direction};
use crate::player::{Player, MoveError};

pub struct GameState {
    pub house: House,
    pub player: Player,
}

impl GameState {
    pub fn new(house: House, player: Player) -> GameState {
        let mut house = House::new();
        let mut player = Player::new();
        GameState {house, player}
    }
    pub fn build_house(house: House) {
            house.add_room(&mut house, RoomId(0), &"Entrance");
            house.add_room(&mut house, RoomId(1), &"Kitchen");
            house.add_room(&mut house, RoomId(2), &"Library");
            house.add_room(&mut house, RoomId(3), &"Basement");
            House::connect_two_way(&mut house, RoomId(0), Direction::North, RoomId(1))
                .expect("Failed to connect Entrance to Kitchen");
            House::connect_two_way(&mut house, RoomId(1), Direction::East, RoomId(2))
                .expect("Failed to connect Kitchen to Library");
            House::connect_two_way(&mut house, RoomId(2), Direction::South, RoomId(3))
                .expect("Failed to connect Library to Basement");
            House::connect_two_way(&mut house, RoomId(3), Direction::West, RoomId(0))
                .expect("Failed to connect Basement to Entrance");
    }

    pub fn current_room(&self) -> &Room {

    }

    pub fn move_player(&mut self, dir: Direction) -> Result<(), MoveError> {
        self.player.move_player(&self.house, dir)
    }
}


