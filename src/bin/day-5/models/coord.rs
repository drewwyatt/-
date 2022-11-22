#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x: x, y: y }
    }
}
