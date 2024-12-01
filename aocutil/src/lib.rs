pub mod grid;
pub mod directon;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Coord(i64, i64);
