/// src/main.rs
mod house;
use house::Direction;
mod player;
mod game_state;
use game_state::GameState;
use std::io::{self, Write};


fn main() -> anyhow::Result<()> {
    let mut game_state = GameState::new(house, player);
   
    loop {
        let room = game_state.room(player.current_room)
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
