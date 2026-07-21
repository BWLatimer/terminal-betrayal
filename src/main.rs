/// src/main.rs
mod house;
use house::{House, RoomId, Direction};
mod player;
use player::{Player};
mod game_state;
use game_state::{GameState};
use std::io::{self, Write};

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
    House::connect_two_way(&mut house, RoomId(2), Direction::South, RoomId(3))
        .expect("Failed to connect Library to Basement");
    House::connect_two_way(&mut house, RoomId(3), Direction::West, RoomId(0))
        .expect("Failed to connect Basement to Entrance");
    house
}

fn create_player() -> Player {
    let player = Player::new(&"Adventurer", RoomId(0));
    player
}

fn main() -> anyhow::Result<()> {
    let house = build_house();
    let player = create_player();
    let mut game_state = GameState::new(house, player);
    println!("Welcome {}, make yourself at home.", game_state.player.name);

    loop {
        // check current room and exits
        let room = game_state.current_room()
            .expect("player rooms should be valid");
        println!("Location: {}", &room.name);
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
                match game_state.move_player(d) {
                    Ok(()) => {}
                    Err(_) => println!("I think that's a wall... Maybe try a different direction?"),
                }
            }
        }
    }
    Ok(())
}
