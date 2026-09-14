//map_state.rs
use std::collections::{VecDeque, HashMap};
use crate::house::{House, RoomId, Direction, Floor};
use crate::exit::{ExitTarget, Traversability};

pub fn compute_positions(house: &House, start: RoomId, floor: Floor) -> HashMap<RoomId, (i32, i32)> {
    let mut positions = HashMap::new();
    positions.insert(start, (0, 0));
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let (cx, cy) = positions[&current];
        if let Ok(room) = house.room(current) {
            // Only process rooms on the same floor
            if room.floor != floor {
                continue;
            }

            for (dir, exit) in &room.exits {
                // Only follow open, resolved exits on the same floor
                if !matches!(exit.traversability, Traversability::Open) {
                    continue;
                }
                if let ExitTarget::Resolved(neighbor) = &exit.target {
                    if let Ok(neighbor_room) = house.room(*neighbor) {
                        if neighbor_room.floor != floor {
                            continue;
                        }

                        if !positions.contains_key(neighbor) {
                            let (dx, dy) = dir.delta();
                        positions.insert(*neighbor, (cx + dx, cy + dy));
                        queue.push_back(*neighbor);
                        }
                    }
                }
            }
        }
    }
    normalize_positions(positions)
}

fn normalize_positions(positions: HashMap<RoomId, (i32, i32)>)-> HashMap<RoomId, (i32, i32)> {
    let min_x = positions.values().map(|(x, _)| *x).min().unwrap_or(0);
    let min_y = positions.values().map(|(_, y)| *y).min().unwrap_or(0);
    positions.into_iter()
        .map(|(id, (x, y))| (id, (x - min_x, y - min_y)))
        .collect()
}

pub fn grid_bounds(positions: &HashMap<RoomId, (i32, i32)>) -> (i32, i32) {
    let max_x = positions.values().map(|(x, _)| *x).max().unwrap_or(0);
    let max_y = positions.values().map(|(_, y)| *y).max().unwrap_or(0);
    (max_x + 1, max_y + 1)
}

