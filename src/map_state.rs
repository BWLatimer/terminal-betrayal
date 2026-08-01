//map_state.rs
use std::collections::HashMap;
use crate::house::RoomId;

pub fn room_positions() -> HashMap<RoomId, (i32, i32)> {
    let mut positions = HashMap::new();
    positions.insert(RoomId(0), (1, 1)); //Entrance
    positions.insert(RoomId(1), (1, 0)); //Kitchen - North of Entrance
    positions.insert(RoomId(2), (2, 0)); //Library - East of Kitchen
    positions.insert(RoomId(3), (2, 1)); //Basement - South of Library
    positions.insert(RoomId(4), (3, 0));
    positions
}
