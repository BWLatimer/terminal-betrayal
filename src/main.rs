/// src/main.rs
mod house;
use house::{House, RoomId, Direction};
use std::io::{self, Write};

fn build_house() -> House {
    let mut house = House::new();

    // TODO: add_room for Entrance (0), Kitchen (1), Library(2), Basement(3)
    House::add_room(&mut house, RoomId(0), &"Entrancece");
    House::add_room(&mut house, RoomId(1), &"Kitchen");
    House::add_room(&mut house, RoomId(2), &"Library");
    House::add_room(&mut house, RoomId(3), &"Basement");

    // TODO: connect them with Direction::North/South/East/West
    House::connect(&mut house, RoomId(0), Direction::North, RoomId(1));
    House::connect(&mut house, RoomId(1), Direction::East, RoomId(2));
    House::connect(&mut house, RoomId(2), Direction::South, RoomId(3));
    House::connect(&mut house, RoomId(3), Direction::West, RoomId(0));

    house
}

fn main() {
    let house = build_house();
    let mut current_room: RoomId = RoomId(0); //start at the entrance
    
    loop {
        let room = house.room(current_room);
        println!("Location: {}", room.name);
        print!("Exits: ");
        for (dir, _) in &room.exits {
            print!("{:?} ", dir);
        }
        println!();
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "quit" {
           break;
        } 

        let dir = match input.to_lowercase().as_str() {
            "n" | "north" => Some(Direction::North),
            "s" | "south" => Some(Direction::South),
            "e" | "east" => Some(Direction::East),
            "w" | "west" => Some(Direction::West),
            "q" => break,
            _ => None,
        };
        match dir {
            None => {
                println!("Oops! That's not an available direction. Please try n, s, e, w, or q");
            }
            Some(d) => {
                let found = room.exits.iter().find(|(exit_dir, _)| *exit_dir == d);
                match found {
                    Some((_, target)) => {
                        current_room = *target;
                    }
                
                    None => {
                        println!("I think that's a wall... Maybe try a different direction?");
                    }
                }
            }
        }
    }
}

