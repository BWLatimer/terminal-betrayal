//map_state.rs
use std::collections::HashMap;
use crate::house::RoomId;

pub fn room_positions() -> HashMap<RoomId, (i32, i32)> {
    let mut positions = HashMap::new();
    positions.insert(RoomId(0), (1, 2)); //Entrance
    positions.insert(RoomId(1), (2, 2)); //Hallway - East of Entrance
    positions.insert(RoomId(2), (3, 2)); //Kitchen - East of Hallway
    positions.insert(RoomId(3), (3, 0)); //Library - East of Landing
    positions.insert(RoomId(4), (1, 0)); //Bedroom - West of Landing
    positions.insert(RoomId(5), (2, 1)); //Staircase - North of Hallway
    positions.insert(RoomId(6), (3, 1)); //Basement - South of Library
    positions.insert(RoomId(7), (2, 0)); //Landing - North of Staircase
    positions.insert(RoomId(8), (1, 1)); //Balcony - South of Bedroom
    positions
}
