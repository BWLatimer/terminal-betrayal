// game-state.rs
use crate::house::{House, RoomId, Direction};
use crate::player::Player;

pub struct GameState {
    pub house: House,
    pub player: Player,
}

impl GameState {
    pub fn new(house: House, player: Player) -> Self {
        GameState { house, player }
    }

    pub fn build_house() -> House {
        let mut house = House::new();
        House::add_room(&mut house, RoomId(0), &"Entrance");
        House::add_room(&mut house, RoomId(1), &"Kitchen");
        House::add_room(&mut house, RoomId(2), &"Library");
        House::add_room(&mut house, RoomId(3), &"Basement");

        House::connect_two_way(&mut house, RoomId(0), Direction::North, RoomId(1))
            .expect("Failed to connect Entrance to Kitchen");
        House::connect_two_way(&mut house, RoomId(1), Direction::East, RoomId(2))
            .expect("Failed to connect Kitchen to Library");
        House::connect_two_way(&mut house, RoomId(2), Direction::South, RoomId(3));
        House::connect_two_way(&mut house, RoomId(3), Direction::West, RoomId(0));
        house
    }

    pub fn new_player() -> Player {
        let mut player = Player::new()
        //Player::attributes(name: , job: ,
        
        player
    }
    pub fn move_player(&mut self, dir: Direction) -> Result<(), MoveError> {
        self.player.move_player(&self.house, dir)
    }
}
