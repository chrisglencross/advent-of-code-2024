pub type Coord = (i64, i64);

pub fn add(c0: Coord, c1: Coord) -> Coord {
    (c0.0 + c1.0, c0.1 + c1.1)
}

pub fn sub(c0: Coord, c1: Coord) -> Coord {
    (c0.0 - c1.0, c0.1 - c1.1)
}

pub fn mul(c0: Coord, m: i64) -> Coord {
    (c0.0 * m, c0.1 * m)
}

pub fn manhattan_distance(c0: Coord, c1: Coord) -> i64 {
    (c0.0 - c1.0).abs() + (c0.1 - c1.1).abs()
}
