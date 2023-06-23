use crate::state::{GameState, Move, MoveDirection};

use log::info;

pub fn start_game(_state: GameState) {
    info!("Started game");
}

pub fn end_game(_state: GameState) {
    info!("Ended game");
}

pub fn make_move(_state: GameState) -> Move {
    MoveDirection::Up.into()
}
