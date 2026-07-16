/// src/main.rs
mod house;
use house::{House, RoomId, Direction};
mod player;
use player::Player;
use std::io::{self, Write};

fn build_house() -> House {
    let mut house = House::new();

    //add_rooms manually for Entrance (0), Kitchen (1), Library(2), Basement(3)
    House::add_room(&mut house, RoomId(0), &"Entrance");
    House::add_room(&mut house, RoomId(1), &"Kitchen");
    House::add_room(&mut house, RoomId(2), &"Library");
    House::add_room(&mut house, RoomId(3), &"Basement");

    // connect them manually with Direction::North/South/East/West (reversible)
    House::connect_two_way(&mut house, RoomId(0), Direction::North, RoomId(1))
        .expect("Failed to connect Entrance and Kitchen");
    House::connect_two_way(&mut house, RoomId(1), Direction::East, RoomId(2))
        .expect("Failed to connect Kitchen and Library");
    House::connect_two_way(&mut house, RoomId(2), Direction::South, RoomId(3))
        .expect("Failed to connect Library and Basement");
    House::connect_two_way(&mut house, RoomId(3), Direction::West, RoomId(0))
        .expect("Failed to connect Basement and Entrance");

    house
}

fn main() -> anyhow::Result<()> {
    let house = build_house();
    let mut player = Player { name: "Adventurer".to_string(), current_room: RoomId(0) };
    
    loop {
        let room = house.room(player.current_room)
            .expect("player's current_room should always be valid");
        println!("Location: {}", room.name);
        print!("Exits: ");
        for (dir, _) in &room.exits {
            print!("{:?} ", dir);
        }
        println!();
        print!(">");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let dir = match input.to_lowercase().as_str() {
            "n" | "north" => Some(Direction::North),
            "s" | "south" => Some(Direction::South),
            "e" | "east" => Some(Direction::East),
            "w" | "west" => Some(Direction::West),
            "q" | "quit" => break,
            _ => None,
        };

        match dir {
            None => println!("Oops! That's not an available direction. Please try n, s, e, w, or q"),
            Some(d) => {
                match player.move_player(&house, d) {
                    Ok(()) => {}
                    Err(_) => println!("I think that's a wall... Maybe try a different direction?"),
                }
            }
        }
    }
    Ok(())
}
