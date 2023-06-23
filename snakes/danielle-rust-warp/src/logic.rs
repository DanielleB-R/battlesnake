use crate::geometry;
use crate::state::{Battlesnake, Board, Coordinate, GameState, Move, MoveDirection};

use log::info;

pub fn start_game(_state: GameState) {
    info!("Started game");
}

pub fn end_game(_state: GameState) {
    info!("Ended game");
}

#[derive(Debug, Clone, Copy)]
pub struct Weights {
    pub left: f64,
    pub right: f64,
    pub up: f64,
    pub down: f64,
}

impl Weights {
    pub fn new() -> Self {
        Self {
            left: 1.0,
            right: 1.0,
            up: 1.0,
            down: 1.0,
        }
    }

    pub fn avoid(&mut self, direction: MoveDirection) {
        match direction {
            MoveDirection::Left => self.left = 0.0,
            MoveDirection::Right => self.right = 0.0,
            MoveDirection::Up => self.up = 0.0,
            MoveDirection::Down => self.down = 0.0,
        }
    }

    pub fn avoid_if_adjacent(&mut self, head: Coordinate, obstacle: Coordinate) {
        if let Some(dir) = geometry::adjacent_direction(head, obstacle) {
            self.avoid(dir);
        }
    }

    pub fn avoid_snake(&mut self, head: Coordinate, body: &[Coordinate]) {
        for segment in body {
            self.avoid_if_adjacent(head, segment);
        }
    }

    pub fn choose_move(self) -> MoveDirection {
        if self.up > 0 {
            MoveDirection::Up
        } else if self.down > 0 {
            MoveDirection::Down
        } else if self.left > 0 {
            MoveDirection::Left
        } else {
            MoveDirection::Right
        }
    }
}

fn avoid_myself(weights: &mut Weights, you: &Battlesnake) {
    weights.avoid_snake(you.head, &you.body[1..]);
}

fn avoid_walls(weights: &mut Weights, you: &Battlesnake, board: &Board) {
    if you.head.x == 0 {
        weights.avoid(MoveDirection::Left);
    }
    if you.head.x == board.width - 1 {
        weights.avoid(MoveDirection::Right);
    }
    if you.head.y == 0 {
        weights.avoid(MoveDirection::Down);
    }
    if you.head.y == board.height - 1 {
        weights.avoid(MoveDirection::Up);
    }
}

fn avoid_opponents(weights: &mut Weights, you: &Battlesnake, snakes: &[Battlesnake]) {
    for snake in snakes {
        if snake.id == you.id {
            continue;
        }
        weights.avoid_snake(you.head, snake);
    }
}

pub fn make_move(state: GameState) -> Move {
    let mut weights = Weights::new();

    avoid_myself(&mut weights, &state.you);
    avoid_walls(&mut weights, &state.you, &state.board);
    avoid_opponents(&mut weights, &state.you, &state.board.snakes);

    weights.choose_move().into()
}
