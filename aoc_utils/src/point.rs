use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, Rem, Sub, SubAssign};

use crate::Direction;

static NEIGHBORS_HV: [Point; 4] = [
    Point { y: -1, x: 0 },
    Point { y: 0, x: -1 },
    Point { y: 0, x: 1 },
    Point { y: 1, x: 0 },
];

static NEIGHBORS: [Point; 8] = [
    Point { y: -1, x: -1 },
    Point { y: -1, x: 0 },
    Point { y: -1, x: 1 },
    Point { y: 0, x: -1 },
    Point { y: 0, x: 1 },
    Point { y: 1, x: -1 },
    Point { y: 1, x: 0 },
    Point { y: 1, x: 1 },
];

#[derive(Debug, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Eq, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    #[inline]
    /// Absolute distance between rows
    pub fn vertical_distance(&self, other: &Self) -> isize {
        self.y.max(other.y) - self.y.min(other.y)
    }

    #[inline]
    /// Absolute distance between cols
    pub fn horizontal_distance(&self, other: &Self) -> isize {
        self.x.max(other.x) - self.x.min(other.x)
    }

    #[inline]
    /// Moves a point on a grid, and wraps, if needed
    pub fn add_with_wrap(&self, rhs: &Point, max: &Self) -> Self {
        Self {
            x: (self.x + rhs.x + max.x) % max.x,
            y: (self.y + rhs.y + max.y) % max.y,
        }
    }

    /// Measures the horizontal and vertical distance between two points
    pub fn distance(&self, other: &Self) -> Self {
        Self {
            x: self.horizontal_distance(other),
            y: self.vertical_distance(other),
        }
    }

    /// True if the points touch horizontally or vertically.
    /// if `check_diag` then checks to see if they touch on the corners
    pub fn touches(&self, other: &Point, check_diag: bool) -> bool {
        let distance = self.distance(other);
        if self == other {
            return false;
        }
        match check_diag {
            true => distance.y == 1 && distance.x == 1,
            false => distance.y == 0 && distance.x == 1 || distance.y == 1 && distance.x == 0,
        }
    }

    pub fn count_neighbors(&self, neighbors: &[Self]) -> usize {
        neighbors
            .iter()
            .fold(0, |acc, p| match self.touches(p, false) {
                true => acc + 1,
                false => acc,
            })
    }

    pub fn neighbors_hv(&self) -> Vec<Point> {
        NEIGHBORS_HV.iter().map(|dir| *self + *dir).collect()
    }

    pub fn neighbors(&self) -> Vec<Point> {
        NEIGHBORS.iter().map(|dir| *self + *dir).collect()
    }

    pub fn rem_euclid(&mut self, rhs: Self) -> Self {
        Self {
            x: (self.x % rhs.x).abs(),
            y: (self.y % rhs.y).abs(),
        }
    }

    #[inline]
    /// *self + Self { x: 0, y: -1 }
    pub fn north(&self) -> Point {
        *self + Self { x: 0, y: -1 }
    }
    #[inline]
    /// *self + Self { x: 1, y: -1 }
    pub fn north_east(&self) -> Point {
        *self + Self { x: 1, y: -1 }
    }
    #[inline]
    /// *self + Self { x: 1, y: 0 }
    pub fn east(&self) -> Point {
        *self + Self { x: 1, y: 0 }
    }
    #[inline]
    /// *self + Self { x: 1, y: 1 }
    pub fn south_east(&self) -> Point {
        *self + Self { x: 1, y: 1 }
    }
    #[inline]
    /// *self + Self { x: 0, y: 1 }
    pub fn south(&self) -> Point {
        *self + Self { x: 0, y: 1 }
    }
    #[inline]
    /// *self + Self { x: -1, y: 1 }
    pub fn south_west(&self) -> Point {
        *self + Self { x: -1, y: 1 }
    }
    #[inline]
    ///  *self + Self { x: -1, y: 0 }
    pub fn west(&self) -> Point {
        *self + Self { x: -1, y: 0 }
    }

    #[inline]
    /// *self + Self { x: -1, y: -1 }
    pub fn north_west(&self) -> Point {
        *self + Self { x: -1, y: -1 }
    }

    /// Returns the point in the provided direction
    pub fn neighbor(&self, dir: Direction) -> Point {
        match dir {
            Direction::North => self.north(),
            Direction::NorthEast => self.north_east(),
            Direction::East => self.east(),
            Direction::SouthEast => self.south_east(),
            Direction::South => self.south(),
            Direction::SouthWest => self.south_west(),
            Direction::West => self.west(),
            Direction::NorthWest => self.north_west(),
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<(u64, u64)> for Point {
    fn from(value: (u64, u64)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Self;
    fn add(self, rhs: &Point) -> Point {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.y += rhs.y;
        self.x += rhs.x;
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.y += rhs.y;
        self.x += rhs.x;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}

impl Sub<&Point> for Point {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        Self {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.y -= rhs.y;
        self.x -= rhs.x;
    }
}

impl SubAssign<&Point> for Point {
    fn sub_assign(&mut self, rhs: &Self) {
        self.y -= rhs.y;
        self.x -= rhs.x;
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Mul<&Point> for Point {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<i8> for Point {
    type Output = Self;
    fn mul(self, rhs: i8) -> Self::Output {
        Self {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

impl Mul<u8> for Point {
    type Output = Self;
    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

impl Mul<u32> for Point {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

impl Mul<i64> for Point {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

impl Mul<u64> for Point {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

impl Mul<isize> for Point {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<usize> for Point {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

impl Rem for Point {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y % rhs.y,
            x: self.x % rhs.x,
        }
    }
}

impl Rem<&Point> for Point {
    type Output = Self;
    fn rem(self, rhs: &Self) -> Self::Output {
        Self {
            y: self.y % rhs.y,
            x: self.x % rhs.x,
        }
    }
}
