use serde::{Deserialize, Serialize};
use crate::house::RoomId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Traversability {
    Open,
    Locked,
    OneWay,
    Impassable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Visible,
    Secret,
    Hidden,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExitTarget {
    Resolved(RoomId),
    Unresolved {
        pos: (i32, i32),
        floor: crate::house::Floor,
    },
    SealedWall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exit {
    pub target: ExitTarget,
    pub traversability: Traversability,
    pub visibility: Visibility,
}

impl Exit {
    pub fn new_open(target:RoomId) -> Self {
        Exit {
            target: ExitTarget::Resolved(target),
            traversability: Traversability::Open,
            visibility: Visibility::Visible,
        }
    }

    pub fn new_unresolved(pos: (i32, i32), floor: crate::house::Floor) -> Self {
         Exit {
            target: ExitTarget::Unresolved { pos, floor},
            traversability: Traversability::Open,
            visibility: Visibility::Visible,
        }
    }

    pub fn sealed_wall() -> Self {
         Exit {
            target: ExitTarget::SealedWall,
            traversability: Traversability::Impassable,
            visibility: Visibility::Visible,
        }
    }

    pub fn locked(target: RoomId) -> Self {
         Exit {
            target: ExitTarget::Resolved(target),
            traversability: Traversability::Locked,
            visibility: Visibility::Visible,
        }
    }

    pub fn one_way(target: RoomId) -> Self {
         Exit {
            target: ExitTarget::Resolved(target),
            traversability: Traversability::OneWay,
            visibility: Visibility::Visible,
        }
    }

    pub fn secret(target: RoomId) -> Self {
         Exit {
            target: ExitTarget::Resolved(target),
            traversability: Traversability::Open,
            visibility: Visibility::Secret,
        }
    }
}



