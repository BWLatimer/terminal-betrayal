// game-state.rs
use crate::house::{House, Room, RoomId, Direction, HouseError};
use crate::player::{Player, MoveError};
use crate::item::{ItemId, Item, ItemError, Owner, ItemRegistry};
use crate::monster::{MonsterRegistry, MonsterId, Monster};
use crate::event::{GameEvent, EventQueue};
use std::collections::HashMap;

pub struct GameState {
    pub house: House,
    pub player: Player,
    pub registry: ItemRegistry,
    pub monsters: MonsterRegistry,
    pub events: EventQueue,
}

impl GameState {
    pub fn process_events(&mut self) -> Vec<String> {
        let events = self.events.drain();
        let mut notices = Vec::new();
        for event in events {
            match event {
                GameEvent::PlayerMoved { to, .. } => {
                    for monster in self.monsters.monsters_in(to) {
                        notices.push(format!("{} notices you", monster.name));
                    }
                }
            }
        }
        notices
    }

    pub fn new(house: House, player: Player, registry: ItemRegistry, monsters: MonsterRegistry, events: EventQueue) -> GameState {
        GameState {house, player, registry, monsters, events}
    }

    pub fn current_room(&self) -> Result<&Room, HouseError> {
        self.house.room(self.player.current_room)
    }
    pub fn move_player(&mut self, dir: Direction) -> Result<(), MoveError> {
        let from = self.player.current_room;
        self.player.move_player(&self.house, dir)?;
        let to = self.player.current_room;
        self.events.push(GameEvent::PlayerMoved { from, to});
        Ok(())
    }

    pub fn pick_up_item(&mut self, item_id: ItemId) -> Result<(), ItemError> {
        let current = Owner::Room(self.player.current_room);
        if self.registry.owner_of(item_id) != Some(current) {
            return Err(ItemError::ItemNotFound(item_id));
        }
        self.registry.assign(item_id, Owner::Player);
        Ok(())
    }

    pub fn drop_item(&mut self, item_id: ItemId) -> Result<(), ItemError> {
        if self.registry.owner_of(item_id) != Some(Owner::Player) {
            return Err(ItemError::ItemNotFound(item_id));
        }
        self.registry.assign(item_id, Owner::Room(self.player.current_room));
        Ok(())
    }
}


