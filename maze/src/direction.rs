pub type Type = u8;

pub const NORTH: u8 = 0x01;
pub const EAST: u8 = 0x02;
pub const SOUTH: u8 = 0x04;
pub const WEST: u8 = 0x08;

pub fn opposite(direction: Type) -> Type {
    match direction {
        NORTH => SOUTH,
        EAST => WEST,
        SOUTH => NORTH,
        WEST => EAST,
        _ => panic!("Invalid direction"),
    }
}
