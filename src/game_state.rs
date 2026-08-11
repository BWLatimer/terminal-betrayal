// game-state.rs
use crate::house::{House, Room, RoomId, Direction, HouseError};
use crate::player::{Player, MoveError};
use crate::item::{ItemId, Item, ItemError, Owner, ItemRegistry};
use crate::monster::{MonsterRegistry, MonsterId, Monster};
use crate::event::{GameEvent, EventQueue};
use crate::room_event::{RoomEventRegistry, RoomEventKind};
use crate::combat::{CombatOutcome, Combat};
use std::collections::HashMap;

pub struct GameState {
    pub house: House,
    pub player: Player,
    pub registry: ItemRegistry,
    pub monsters: MonsterRegistry,
    pub events: EventQueue,
    pub room_events: RoomEventRegistry,
}

impl GameState {
    pub fn attack(&mut self, monster_id: MonsterId, monster_attacks_first: bool) -> (CombatOutcome, Vec<String>) {
        let monster = self.monsters.monster_mut(monster_id)
            .expect("attack called on a monster that doesn't exist");
        let (outcome, log) = Combat::resolve_round(&mut self.player, monster, monster_attacks_first);
        if let CombatOutcome::PlayerWon = outcome {
            self.monsters.remove_monster(monster_id);
        }
        (outcome, log)
    }

    pub fn process_events(&mut self) -> Vec<String> {
        let events = self.events.drain();
        let mut notices = Vec::new();
        for event in events {
            match event {
                GameEvent::PlayerMoved { to, .. } => {
                    if let Some(kind) = self.room_events.room_check(to) {
                        match kind {
                            RoomEventKind::SpawnMonster(monster_id) => {
                                self.monsters.move_to(monster_id, to);
                                notices.push("Something lurches out of the shadows!".to_string());
                            }
                        }
                        self.end_turn();
                    } else {
                         let monster_ids: Vec<MonsterId> = self.monsters.monsters_in(to)
                        .iter().map(|m| m.id).collect();
                        for id in monster_ids {
                            let name: String = self.monsters.monster(id).iter().map(|m| m.name.clone()).collect();
                            notices.push(format!("a {} notices you", name)); //enter id, return Monster
                        }
                    } 
                }
            }
        }
        notices
    }
    
    pub fn end_turn(&mut self) {
        let to = self.player.current_room;
        for monster in self.monsters.all_monsters_mut() {
            if let Some(current) = monster.current_room {
                for _ in 0..monster.speed {
                    if let Some(next) = self.house.next_step_toward(current, to) {
                        monster.current_room = Some(next);
                    } else {
                        break;
                    }
                }
            }
        }
        self.player.moves_remaining = self.player.speed;
    }

    pub fn new(house: House, player: Player, registry: ItemRegistry, monsters: MonsterRegistry, events: EventQueue, room_events: RoomEventRegistry) -> GameState {
        GameState {house, player, registry, monsters, events, room_events}
    }

    pub fn current_room(&self) -> Result<&Room, HouseError> {
        self.house.room(self.player.current_room)
    }
    pub fn move_player(&mut self, dir: Direction) -> Result<(), MoveError> {
        if self.player.moves_remaining == 0 {
            return Err(MoveError::NoMovesRemaining);
        }
        let from = self.player.current_room;
        self.player.move_player(&self.house, dir)?;
        self.player.moves_remaining -= 1;
        let to = self.player.current_room;
        self.events.push(GameEvent::PlayerMoved { from, to});

        if self.player.moves_remaining == 0 {
            self.end_turn();
        }
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


