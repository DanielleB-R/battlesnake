use crate::state::{Coordinate, MoveDirection};

pub fn adjacent_direction(origin: Coordinate, adjacent: Coordinate) -> Option<MoveDirection> {
    if origin.x + 1 == adjacent.x && origin.y == adjacent.y {
        Some(MoveDirection::Right)
    } else if origin.x - 1 == adjacent.x && origin.y == adjacent.y {
        Some(MoveDirection::Left)
    } else if origin.y + 1 == adjacent.y && origin.x == adjacent.x {
        Some(MoveDirection::Up)
    } else if origin.y - 1 == adjacent.y && origin.x == adjacent.x {
        Some(MoveDirection::Down)
    } else {
        None
    }
}
