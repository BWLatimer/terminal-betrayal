// src/house.rs
use std::collections::{HashMap, VecDeque, HashSet};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::exit::Exit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoomId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Floor {
    Basement,
    FirstFloor,
    SecondFloor,
    Attic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    North, South, East, West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomType {
    //standard topology
    DeadEnd,
    Corner,
    Passage,
    Split,
    Crossroads,

    //non-standard connectors
    Stairway,
    SecretPassage,
    HoleInFloor,
    Elevator,
    Portal,
}

impl RoomType {
    pub fn exit_count(&self) -> usize {
        match self {
            RoomType::DeadEnd => 1,
            RoomType::Corner => 2,
            RoomType::Passage => 2,
            RoomType::Split => 3,
            RoomType::Crossroads => 4,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExitLogic {
    Stairway { target_floor: Floor },
    Hole { target_floor: Floor },
    Elevator { target_floor: Option<Floor> },
    SecretPassage { target_floor: Option<Floor> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomContent {
    pub name: String,
    pub description: String,

    pub items: Vec<crate::item::ItemId>,
    pub clues: Vec<String>,

    pub monster_theme: String,
    pub monster_spawnable: bool,
    
    pub one_way_exits: HashSet<Direction>, //not sure if this will work with rooms that could be
    //rotated?
    pub exit_logic: Option<ExitLogic>,
}

impl RoomContent {
    pub fn new(name: &str, description: &str) -> Self {
        RoomContent {
            name: name.to_string(),
            description: description.to_string(),
            items: Vec::new(),
            clues: Vec::new(),
            monster_theme: String::new(),
            monster_spawnable: false,
            one_way_exits:HashSet::new(),
            exit_logic: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: RoomId,
    pub pos: (i32, i32),
    pub floor: Floor,
    pub room_type: RoomType,

    pub exits: HashMap<Direction, Exit>,
    pub content: RoomContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct House {
    pub rooms: HashMap<RoomId, Room>,
    pub position_grid: HashMap<(i32, i32, Floor), RoomId>,
    pub entrance_id: RoomId,

    //phase 2 of lazy generation
    pub frontier: VecDeque<FrontierExit>,
    pub room_deck: Vec<RoomTemplate>,
    pub generation_seed: u64,


    pub next_room_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontierExit {
    pub parent_room_id: RoomId,
    pub parent_pos: (i32, i32),
    pub parent_floor: Floor,
    pub exit_direction: Direction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTemplate {
    pub room_type: RoomType,
    pub content_id: String,
}

#[derive(Debug, Error)]
pub enum HouseError {
    #[error("no such room: {0:?}")]
    RoomNotFound(RoomId),
}

impl House {
    pub fn new() -> Self {
        House { 
            rooms: HashMap::new(),
            position_grid:HashMap::new(),
            entrance_id: RoomId(0),
            frontier: VecDeque::new(),
            room_deck: Vec::new(),
            generation_seed: 0,
            next_room_id: 0,
        }
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
                for exit in room.exits.values() {
                    //only traverse open, resolved exits
                    if let crate::exit::ExitTarget::Resolved(neighbor) = &exit.target {
                        if !visited.contains_key(neighbor) && *neighbor != from {
                            visited.insert(*neighbor, current);
                            queue.push_back(*neighbor);
                        }
                    }
                }
            }
        }
        None //no path found
    }

    pub fn add_room(&mut self, id: RoomId, name: &str, floor: Floor, pos: (i32, i32), room_type: RoomType) {
        let content = RoomContent::new(name, "");
        let room = Room {
                id,
                pos,
                floor,
                room_type,
                exits: HashMap::new(),
                content,
        };
            self.rooms.insert(id, room);
            self.position_grid.insert((pos.0, pos.1, floor), id);
    }

//Unidirectional connection
    pub fn connect(&mut self, from: RoomId, dir: Direction, to: RoomId) -> Result<(), HouseError> {
        let room = self.rooms.get_mut(&from).ok_or(HouseError::RoomNotFound(from))?;
        room.exits.insert(dir, Exit::new_open(to));
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

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    pub fn all() -> [Direction; 4] {
        [Direction::North, Direction::South, Direction::East, Direction::West]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Direction::North=> "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
