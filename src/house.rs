// src/house.rs
use std::collections::{HashMap, VecDeque};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomId(pub usize);

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub exits: Vec<(Direction, RoomId)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North, South, East, West,
}

pub struct House {
    rooms: HashMap<RoomId, Room>,
}

#[derive(Debug, Error)]
pub enum HouseError {
    #[error("no such room: {0:?}")]
    RoomNotFound(RoomId),
}

impl House {
    pub fn new() -> Self {
        House { rooms: HashMap::new() }
    }

    pub fn next_step_toward(&self, from: RoomId, to: RoomId) -> Option<RoomId> {
        if from == to {
            return None;
        }

        let mut visited: HashMap<RoomId, RoomId> = HashMap::new(); //tracking the room we
        //came from
        let mut queue: VecDeque<RoomId> = VecDeque::new();
        queue.push_back(from);

        while let Some(current) = queue.pop_front() {
            if current == to {
                // walk backward from 'to' until we find the stop right after 'from'
                let mut step = to;
                while let Some(&prev) = visited.get(&step) {
                    if prev == from {
                        return Some(step);
                    }
                    step = prev;
                }
            }
            if let Ok(room) = self.room(current) {
                for (_, neighbor) in &room.exits {
                    if !visited.contains_key(neighbor) && *neighbor != from {
                        visited.insert(*neighbor, current);
                        queue.push_back(*neighbor);
                    }
                }
            }
        }
        None //no path found
    }
    pub fn add_room(&mut self, id: RoomId, name: &str) {
        self.rooms.insert(id, Room { name: name.to_string(), exits: Vec::new()});
    }
//Unidirectional connection
    pub fn connect(&mut self, from: RoomId, dir: Direction, to: RoomId) -> Result<(), HouseError> {
        let room = self.rooms.get_mut(&from).ok_or(HouseError::RoomNotFound(from))?;
        room.exits.push((dir, to));
        Ok(())
    }

    pub fn room(&self, id: RoomId) -> Result<&Room, HouseError> {
        self.rooms.get(&id).ok_or(HouseError::RoomNotFound(id))
    }

//Bidirectional connection
    pub fn connect_two_way(&mut self, a: RoomId, dir: Direction, b: RoomId) -> Result<(),HouseError> {
        self.connect(a, dir, b)?;
        self.connect(b, dir.opposite(), a)?;
        Ok(())
    }
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::North=> Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}
