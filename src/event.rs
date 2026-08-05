//event.rs
use crate::house::RoomId;

#[derive(Debug, Clone)]
pub enum GameEvent {
    PlayerMoved { from: RoomId, to: RoomId },
    // TODO: more variants to come later
}

pub struct EventQueue {
    events: Vec<GameEvent>,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue { events: Vec::new() }
    }

    pub fn push(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.events)
    }
}
