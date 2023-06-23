use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub game: serde_json::Value,
    pub turn: usize,
    pub board: serde_json::Value,
    pub you: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Move {
    #[serde(rename = "move")]
    pub direction: MoveDirection,
}

impl From<MoveDirection> for Move {
    fn from(direction: MoveDirection) -> Self {
        Self { direction }
    }
}
