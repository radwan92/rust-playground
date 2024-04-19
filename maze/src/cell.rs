use crate::direction;

#[derive(Copy, Clone)]
pub struct Cell {
    pub visited: bool,
    pub paths: direction::Type,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            visited: false,
            paths: 0,
        }
    }
}
