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
        // TODO: print current room name + list its exits
        println!("Location: {}", house.room(&Room));
        // hint: house.room(current_room) gives you a &Room
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "quit" {
            break;
        }
    }
}
        // TODO: parse input into a Direction, look up whether current_room
        // has an exit in that direction, and if so update current_room

