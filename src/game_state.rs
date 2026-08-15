// game-state.rs
use crate::house::{House, Room, RoomId, Direction, HouseError};
use crate::player::{Player, MoveError};
use crate::item::{ItemId, Item, ItemError, Owner, ItemRegistry};
use crate::monster::{MonsterRegistry, MonsterId, Monster};
use crate::event::{GameEvent, EventQueue};
use crate::room_event::{RoomEventRegistry, RoomEventKind};
use crate::combat::{CombatOutcome, resolve_round, resolve_flee, flee_drops_item};
use std::collections::HashMap;

pub struct GameState {
    pub house: House,
    pub player: Player,
    pub registry: ItemRegistry,
    pub monsters: MonsterRegistry,
    pub events: EventQueue,
    pub room_events: RoomEventRegistry,
    pub pending_ambush: Option<MonsterId>
}

impl GameState {

    pub fn new(house: House, player: Player, registry: ItemRegistry, monsters: MonsterRegistry, events: EventQueue, room_events: RoomEventRegistry, pending_ambush: Option<MonsterId>) -> GameState {
        GameState {house, player, registry, monsters, events, room_events, pending_ambush: None}
    }

    pub fn attack(&mut self, monster_id: MonsterId, monster_attacks_first: bool) -> (CombatOutcome, Vec<String>) {
        let monster = self.monsters.monster_mut(monster_id)
            .expect("attack called on a monster that doesn't exist");
        let (outcome, log) = resolve_round(&mut self.player, monster, monster_attacks_first);
        match outcome {
            CombatOutcome::PlayerWon => {
                self.monsters.remove_monster(monster_id);
            }
            CombatOutcome::PlayerDefeated => {
                self.respawn_player();
            }
            CombatOutcome::Ongoing => {}
            CombatOutcome::PlayerFled => {}
        }
        (outcome, log)
    }

    pub fn flee(&mut self, monster_id: MonsterId) -> Vec<String> {
        let monster = self.monsters.monster(monster_id)
            .expect("flee called on a monster that doesn't exist");
        let mut log = resolve_flee(&mut self.player, monster);
        
        if flee_drops_item() {
            let carried = self.registry.items_owned_by(Owner::Player);
            if let Some(&item_id) = carried.first() {
                let name = self.registry.name_of(item_id).unwrap_or("something").to_string();
                self.registry.assign(item_id, Owner::Room(self.player.current_room));
                log.push(format!("You drop your {} in the panic!", name));
            }
        }
        log
    }

    pub fn process_events(&mut self) -> Vec<String> {
        let events = self.events.drain();
        let mut notices = Vec::new();
        for event in events {
            match event {
                GameEvent::PlayerMoved { to, .. } => {
                    let mut spawned_this_turn = false;

                    if let Some(kind) = self.room_events.check_and_trigger(to) {
                        match kind {
                            RoomEventKind::SpawnMonster(monster_id) => {
                                self.monsters.move_to(monster_id, to);
                                notices.push("Something lurches out of the shadows!".to_string());
                                spawned_this_turn = true;
                            }
                        }
                    }
                    let monsters_here: Vec<MonsterId> = self.monsters.monsters_in(to)
                        .iter().map(|m| m.id).collect();
                    if let Some(&monster_id) = monsters_here.first() {
                        if !spawned_this_turn {
                            let name: String = self.monsters.monster(monster_id).iter().map(|m| m.name.clone()).collect();
                            notices.push(format!("a {} notices you", name)); //enter id, return Monster
                        }
                        self.pending_ambush = Some(monster_id);
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

    pub fn respawn_player(&mut self) {
        self.player.current_room = RoomId(0);
        self.player.health = self.player.max_health / 2;
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


