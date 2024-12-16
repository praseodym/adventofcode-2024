use std::ops::{Add, AddAssign};

pub const NORTH: Point = Point::new(0, -1);
pub const SOUTH: Point = Point::new(0, 1);
pub const WEST: Point = Point::new(-1, 0);
pub const EAST: Point = Point::new(1, 0);
pub const UP: Point = NORTH;
pub const DOWN: Point = SOUTH;
pub const LEFT: Point = WEST;
pub const RIGHT: Point = EAST;
pub const NESW: [Point; 4] = [NORTH, EAST, SOUTH, WEST];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn from_ascii(c: u8) -> Self {
        match c {
            b'^' => NORTH,
            b'v' => SOUTH,
            b'<' => WEST,
            b'>' => EAST,
            _ => unreachable!(),
        }
    }

    pub const fn rotate_90deg_cw(self) -> Point {
        Point::new(-self.y, self.x)
    }

    pub const fn rotate_90deg_ccw(self) -> Point {
        Point::new(self.y, -self.x)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
