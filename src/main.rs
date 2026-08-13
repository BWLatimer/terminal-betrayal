/// src/main.rs
mod house;
use house::{House, RoomId, Direction};
mod player;
use player::{Player, PlayerConfig};
mod game_state;
use game_state::{GameState};
mod app;
use app::{App};
mod map_state;
mod item;
use item::{ItemRegistry, Item, ItemId};
mod monster;
use monster::{MonsterRegistry, MonsterId, Monster};
mod event;
use event::{EventQueue, GameEvent};
mod room_event;
use room_event::{RoomEventRegistry, RoomEventKind};
mod combat;
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
    ItemRegistry::assign(& mut items, ItemId(2), item::Owner::Room(RoomId(4)));
    items
}

pub fn build_house() -> House {
    let mut house = House::new();
    House::add_room(&mut house, RoomId(0), &"Entrance");
    House::add_room(&mut house, RoomId(1), &"Hallway");
    House::add_room(&mut house, RoomId(2), &"Kitchen");
    House::add_room(&mut house, RoomId(3), &"Library");
    House::add_room(&mut house, RoomId(4), &"Master Bedroom");
    House::add_room(&mut house, RoomId(5), &"Staircase");
    House::add_room(&mut house, RoomId(6), &"Basement");
    House::add_room(&mut house, RoomId(7), &"2nd floor landing");
    House::add_room(&mut house, RoomId(8), &"Balcony");

    House::connect_two_way(&mut house, RoomId(0), Direction::East, RoomId(1))
        .expect("Failed to connect Entrance to Hallway");
    House::connect_two_way(&mut house, RoomId(1), Direction::East, RoomId(2))
        .expect("Failed to connect Hallway to Kitchen");
    House::connect_two_way(&mut house, RoomId(1), Direction::North, RoomId(5))
        .expect("Failed to connect the Hallway to the Staircase");
    House::connect_two_way(&mut house, RoomId(5), Direction::North, RoomId(7))
        .expect("Failed to connect Staircase to 2nd floor landing");
    House::connect_two_way(&mut house, RoomId(7), Direction::East, RoomId(3))
        .expect("Failed to connect landing to Library");
    House::connect(&mut house, RoomId(3), Direction::South, RoomId(6))
        .expect("Failed to connect Library to Basement");
    House::connect(&mut house, RoomId(6), Direction::South, RoomId(2))
        .expect("Failed to connect Basement to Kitchen");
    House::connect_two_way(&mut house, RoomId(7), Direction::West, RoomId(4))
        .expect("Failed to connect landing to Master Bedroom");
    House::connect_two_way(&mut house, RoomId(4), Direction::South, RoomId(8))
        .expect("Failed to connect Bedroom to Balcony");
    house
}

fn load_player_config() -> anyhow::Result<PlayerConfig> {
    let toml_str = std::fs::read_to_string("assets/player.toml")?;
    let config: player::PlayerConfig = toml::from_str(&toml_str)?;
    Ok(config)
}

fn create_player(config: PlayerConfig) -> Player {
    let player = Player::new(config.name, RoomId(0), config.health, config.strength, config.speed);
    player
}

pub fn new_monsters() -> MonsterRegistry {
    let mut monsters = MonsterRegistry::new();
    monsters.add_monster(MonsterId(0), "Zombie", None, 15, 2, 1);
    monsters
}

pub fn basement_spawn() -> RoomEventRegistry {
    let mut room_event = RoomEventRegistry::new();
    room_event.add_event(RoomId(6), RoomEventKind::SpawnMonster(MonsterId(0)));
    room_event
}

fn main() -> anyhow::Result<()> {
    install_panic_hook();
    let house = build_house();
    let player_config = load_player_config()?;
    let player = create_player(player_config);
    let registry = new_registry();
    let monsters = new_monsters();
    let events = EventQueue::new();
    let room_event = basement_spawn();
    let mut game_state = GameState::new(house, player, registry, monsters, events, room_event);
    let mut app = App::new_game(game_state, app::AppMode::Exploring);

    let mut terminal = ratatui::init();
    while !app.should_quit {
        terminal.draw(|frame| App::render(frame, &mut app))?;
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                App::handle_key(&mut app, key);
            }
        }
    }
    ratatui::restore();
    Ok(())
}
