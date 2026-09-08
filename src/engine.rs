//engine.rs
use crate::game_state::GameState;
use crate::house::Direction;
use crate::item::{ItemId, Owner};
use crate::monster::MonsterId;
use crate::combat::CombatOutcome;
use std::fmt;
///Player action intent - what the user wants to do
#[derive(Debug, Clone)]
pub enum PlayerAction {
    Move(Direction),
    Search,
    PickUp,
    Drop(ItemId),
    EndTurn,
    EngageMonster,
    Attack,
    Flee,
}

///What changed in the game after an action
#[derive(Debug)]
pub struct GameUpdate {
    pub messages: Vec<String>,
    ///Deprecated: use engine.in_combat() instead
    pub pending_combat: Option<MonsterId>,
    //TODO: add any other updates to game_state that are relevant
}
impl GameUpdate{
    pub fn new(messages: Vec<String>, pending_combat: Option<MonsterId>) -> Self {
        GameUpdate { messages, pending_combat }
    }

    pub fn message(msg: &str) -> Self {
        GameUpdate {
            messages: vec![msg.to_string()],
            pending_combat: None,
        }
    }

    pub fn empty() -> Self {
        GameUpdate {
            messages: Vec::new(),
            pending_combat: None,
        }
    }
}

#[derive(Debug)]
pub enum GameError {
    InvalidMove,
    NoMovesRemaining,
    NoItemsInRoom,
    NothingToPickUp,
    ItemNotFound,
    NoMonsterToFight,
    MonsterNotFound(MonsterId),
    //other errors
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::InvalidMove => write!(f, "Can't go that way"),
            GameError::NoMovesRemaining => write!(f, "No moves left"),
            GameError::NoItemsInRoom => write!(f, "Nothing here to pick up"),
            GameError::NothingToPickUp => write!(f, "Nothing to pick up - search first with [s]"),
            GameError::ItemNotFound => write!(f, "Item not found"),
            GameError::NoMonsterToFight => write!(f, "There's nothing to fight here"),
            GameError::MonsterNotFound(_) => write!(f, "Monster not found"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CombatInfo {
    pub monster_id: MonsterId,
    pub monster_attacks_first: bool,
}

pub struct GameEngine {
    state: GameState,
    combat_state: Option<CombatInfo> //If not NONE, player is currently in combat 
}

impl GameEngine {
    pub fn new(state: GameState) -> Self {
        GameEngine {state, combat_state: None}
    }

///Apply and action, return changes
    pub fn apply_action(&mut self, action: PlayerAction) -> Result<GameUpdate, GameError> {
        match action {
            PlayerAction::Move(dir) => self.handle_move(dir),
            PlayerAction::Search => self.handle_search(),
            PlayerAction::PickUp => self.handle_pickup(),
            PlayerAction::Drop(item_id) => self.handle_drop(item_id),
            PlayerAction::EndTurn => self.handle_end_turn(),
            PlayerAction::EngageMonster => self.handle_engage_monster(),
            PlayerAction::Attack => self.handle_attack(),
            PlayerAction::Flee => self.handle_flee(),
        }
    }
///Snapshot of current GameState for rendering
    pub fn get_state(&self) -> &GameState {
        &self.state
    }
/// Check if currently in combat
    pub fn in_combat(&self) -> bool {
        self.combat_state.is_some()
    }
///Get current Combat Opponent if any
    pub fn current_combat(&self) -> Option<&CombatInfo> {
        self.combat_state.as_ref()
    }
///End Combat by participant death or retreat
    fn end_combat(&mut self) {
        self.combat_state = None;
    }

    // =====Exploration Actions=====

   fn handle_move(&mut self, dir: Direction) -> Result<GameUpdate, GameError> {
       match self.state.move_player(dir) {
            Ok(()) => {
                // Move succeeded, now check for ambushes
                let ambush_notices = self.detect_ambush();
                let pending = self.combat_state.as_ref().map(|c| c.monster_id);
                Ok(GameUpdate::new(ambush_notices, pending))
            }
            Err(_) => {
                Ok(GameUpdate::message(&format!("{}", GameError::InvalidMove)))
            }
        }
    }

    fn handle_search(&mut self) -> Result<GameUpdate, GameError> {
        let search = match self.state.player.search_room(&self.state.registry) {
            Result::Ok(()) => {
                let item_id = self.state.player.found_item.expect("search_room set found_item on Ok");
                let name = self.state.registry.name_of(item_id).unwrap_or("something");
                GameUpdate::message(&format!("You found: {}", name))
            },
            Result::Err(_) => GameUpdate::message(&format!("{}", GameError::NoItemsInRoom)),
        };
        Ok(search)
    }

    fn handle_pickup(&mut self) -> Result<GameUpdate, GameError> {
        let pick_up = match self.state.player.found_item {
            Some(item_id) => match self.state.pick_up_item(item_id) {
                Result::Ok(()) => GameUpdate::message("You picked it up."),
                Result::Err(_) => GameUpdate::message(&format!("{}", GameError::NothingToPickUp)),
            },
            None => GameUpdate::new(vec![format!("{}", GameError::NothingToPickUp)].into(), None),
        };
        Ok(pick_up)
    }

    fn handle_drop(&mut self, item_id: ItemId) -> Result<GameUpdate, GameError> {
        let drop = match self.state.drop_item(item_id) {
            Result::Ok(()) => GameUpdate::message("Dropped it"),
            Result::Err(_) => GameUpdate::message(&format!("{}", GameError::ItemNotFound)),
        };
        Ok(drop)
    }

    fn handle_end_turn(&mut self) -> Result<GameUpdate, GameError> {
        self.state.end_turn();
        let mut notices = vec!["You end your turn.".to_string()];
        let ambush_notices = self.detect_ambush();
        notices.extend(ambush_notices);
        let pending = self.combat_state.as_ref().map(|c| c.monster_id);
        Ok(GameUpdate::new(notices, pending))
    }

    fn handle_engage_monster(&mut self) -> Result<GameUpdate, GameError> {
        let monsters_here = self.state.monsters.monsters_in(self.state.player.current_room);
        match monsters_here.first() {
            Some(monster) => {
                let monster_id = monster.id;
                self.combat_state = Some(CombatInfo {
                    monster_id, 
                    monster_attacks_first: true,
                });
                Ok(GameUpdate::message(&format!("A {} blocks your path", monster.name)))
            },
            None => Err(GameError::NoMonsterToFight),
        }
    }

    // ===== Combat Actions ======

    fn handle_attack(&mut self) -> Result<GameUpdate, GameError> {
        let combat = self.combat_state.as_ref()
            .ok_or(GameError::NoMonsterToFight)?;

        let monster_id = combat.monster_id;
        let monster_attacks_first = combat.monster_attacks_first;

        let (outcome, log) = self.state.attack(monster_id, monster_attacks_first);
        match outcome {
            CombatOutcome::PlayerWon => {
                self.end_combat();
                Ok(GameUpdate::new(log, None))
            },
            CombatOutcome::PlayerDefeated => {
                self.end_combat();
                Ok(GameUpdate::new(log, None))
            },
            CombatOutcome::Ongoing => {
                Ok(GameUpdate::new(log, Some(monster_id)))
            },
            CombatOutcome::PlayerFled => {
                self.end_combat();
                Ok(GameUpdate::new(log, None))
            },
        }
    }

    fn handle_flee(&mut self) -> Result<GameUpdate, GameError> {
        let combat = self.combat_state.as_ref()
            .ok_or(GameError::NoMonsterToFight)?;
        let monster_id = combat.monster_id;
        let log = self.state.flee(monster_id);

        self.end_combat();
        Ok(GameUpdate::new(log, None))
    }

    // ===== Helper Methods =====
    // add as needed. Examples:

    fn detect_ambush(&mut self) -> Vec< String >  {
        let notices = self.state.process_events();
        if !notices.is_empty() {
            if let Some(monster_id) = self.state.pending_ambush.take() {
                self.combat_state = Some(CombatInfo {
                    monster_id,
                    monster_attacks_first: true
                })
            }
        };
        notices
    }

    pub fn get_player_inventory(&self) -> Vec<ItemId> {
        self.state.registry.items_owned_by(Owner::Player)
    }
}
