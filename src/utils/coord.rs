use std::fmt;

pub struct Coord {
    pub x: i32,
    pub y: i32,
}
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
impl Clone for Coord {
    fn clone(&self) -> Self {
        Self {x:self.x, y: self.y}
    }
}