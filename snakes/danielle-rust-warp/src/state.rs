use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub game: Game,
    pub turn: usize,
    pub board: Board,
    pub you: Battlesnake,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: String,
    pub ruleset: serde_json::Value,
    pub map: String,
    pub timeout: usize,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub height: usize,
    pub width: usize,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
    pub snakes: Vec<Battlesnake>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: usize,
    pub body: Vec<Coordinate>,
    pub latency: String,
    pub head: Coordinate,
    pub length: usize,
    pub shout: String,
    pub squad: String,
    pub customizations: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
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
