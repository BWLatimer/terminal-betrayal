/// src/main.rs
mod house;
use house::{House, RoomId, Direction};
use std::io::{self, Write};

fn build_house() -> House {
    let mut house = House::new();

    // TODO: add_room for Entrance (0), Kitchen (1), Library(2), Basement(3)
    // TODO: connect them with Direction::North/South/East/West

    house
}

fn main() {
    let house = build_house();
    let mut current_room: RoomId = RoomId(0); //start at the entrance

    loop {
        // TODO: print current room name + list its exits
        println!("Location: {}", house.room(current_room));
        // hint: house.room(current_room) gives you a &Room

        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            break;
        }
        // TODO: parse input into a Direction, look up whether current_room
        // has an exit in that direction, and if so update current_room
    }
}
