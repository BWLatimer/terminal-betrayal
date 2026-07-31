/// src/main.rs
mod house;
use house::{House, RoomId, Direction};
mod player;
use player::{Player};
mod game_state;
use game_state::{GameState};
mod app;
use app::{App};
mod map_state;
mod item;
use item::{ItemRegistry, Item, ItemId};
use std::io::{self, Write};

// handle panic error states for ratatui
fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = ratatui::restore();
        original_hook(panic_info);
    }));
}

 pub fn new_registry() -> ItemRegistry {
    let mut items = ItemRegistry::new();
    ItemRegistry::add_item(& mut items, ItemId(1), &"Room Key", &"A small key for opening rooms");
    ItemRegistry::add_item(& mut items, ItemId(2), &"Crowbar", &"A large piece of metal build for prying... or clobbering.");
    ItemRegistry::add_item(& mut items, ItemId(0), &"Backpack", &"A basic bag for holding your inventory");
    ItemRegistry::assign(& mut items, ItemId(0), item::Owner::Player);
    ItemRegistry::assign(& mut items, ItemId(1), item::Owner::Room(RoomId(3)));
    ItemRegistry::assign(& mut items, ItemId(2), item::Owner::Nowhere);
    items
}

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
    install_panic_hook();
    let house = build_house();
    let player = create_player();
    let registry = new_registry();
    let mut game_state = GameState::new(house, player, registry);
    let mut app = App::new_game(game_state);

    let mut terminal = ratatui::init();
    while !app.should_quit {
        terminal.draw(|frame| App::render(frame, &app))?;
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                App::handle_key(&mut app, key);
            }
        }
    }
    ratatui::restore();
    Ok(())
}
