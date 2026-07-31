// game-state.rs
use crate::house::{House, Room, RoomId, Direction, HouseError};
use crate::player::{Player, MoveError};
use crate::item::{ItemId, Item, ItemError, Owner, ItemRegistry};
use std::collections::HashMap;

pub struct GameState {
    pub house: House,
    pub player: Player,
    pub registry: ItemRegistry,
}

impl GameState {
    pub fn new(house: House, player: Player, registry: ItemRegistry) -> GameState {
        GameState {house, player, registry}
    }

    pub fn current_room(&self) -> Result<&Room, HouseError> {
        self.house.room(self.player.current_room)
    }
    pub fn move_player(&mut self, dir: Direction) -> Result<(), MoveError> {
        self.player.move_player(&self.house, dir)
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

    pub fn drop_last_item(&mut self) -> Result<(), ItemError> {
        match self.registry.items_owned_by(Owner::Player).last().copied() {
            Some(item_id) => self.drop_item(item_id),
            None => Err(ItemError::InventoryEmpty),
        }
    }
}


