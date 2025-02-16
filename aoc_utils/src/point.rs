//! Cartesian, (x,y) point for maps, grids, etc.
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, Rem, Sub, SubAssign};

use crate::Direction;

static CARDINAL_NEIGHBORS: [Point; 4] = [
    Point { y: -1, x: 0 },
    Point { y: 0, x: -1 },
    Point { y: 0, x: 1 },
    Point { y: 1, x: 0 },
];

static ORDINAL_NEIGHBORS: [Point; 4] = [
    Point { y: -1, x: -1 },
    Point { y: -1, x: 1 },
    Point { y: 1, x: -1 },
    Point { y: 1, x: 1 },
];

/// Typical grid navigation, incouding the ability to determine neighbors, etc.
///
/// Point uses `isize` to ensure we can manage positive and negative positions from an
/// origin in both the x and y directions.
///
/// [Direction] is used to support navigation,  While [Direction] supports all 8 cardinal and ordinal compass
/// positions, most of the time, we only care about the cardinal positions (North, South, East, and West).
#[derive(Debug, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Eq, Default)]
pub struct Point {
    /// Column position
    pub x: isize,
    /// Row position
    pub y: isize,
}

impl Point {
    #[inline]
    /// Absolute distance between rows
    pub fn vertical_distance(&self, other: &Self) -> isize {
        self.y.abs_diff(other.y) as isize
    }

    #[inline]
    /// Absolute distance between cols
    pub fn horizontal_distance(&self, other: &Self) -> isize {
        self.x.abs_diff(other.x) as isize
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

    /// True if the points touch in a cardinal direction.
    /// To include ordinal positions also, use `touches_ord`.
    pub fn touches(&self, other: &Point) -> bool {
        if self == other {
            return false;
        }

        let distance = self.distance(other);
        distance.y == 0 && distance.x == 1 || distance.y == 1 && distance.x == 0
    }

    /// True if the points touches in any direction (cardinal and ordinal).
    /// if `check_diag` then checks to see if they touch on the corners
    pub fn touches_ord(&self, other: &Self) -> bool {
        if self == other {
            return false;
        }

        let distance = self.distance(other);
        distance.y == 1 && distance.x == 1
    }

    /// Given a collection of `Point`, determine how many touch `self` in
    /// a cardinal direction.
    pub fn count_neighbors(&self, neighbors: &[Self]) -> usize {
        neighbors.iter().fold(0, |acc, p| match self.touches(p) {
            true => acc + 1,
            false => acc,
        })
    }

    /// Given a collection of `Point`, determine how many touch `self` in
    /// either a cardinal or ordinal direction.
    pub fn count_neighbors_ord(&self, neighbors: &[Self]) -> usize {
        neighbors
            .iter()
            .fold(0, |acc, p| match self.touches_ord(p) {
                true => acc + 1,
                false => acc,
            })
    }

    /// Returns the collection of cardinal neighbors for this point
    pub fn neighbors(&self) -> Vec<Point> {
        CARDINAL_NEIGHBORS.iter().map(|dir| *self + *dir).collect()
    }

    pub fn neighbors_with_dir(&self) -> Vec<(Point, Direction)> {
        [
            (self.north(), Direction::North),
            (self.east(), Direction::East),
            (self.south(), Direction::South),
            (self.west(), Direction::West),
        ]
        .to_vec()
    }

    /// Returns the collection of cardinal and ordinal neighbors for this point.
    pub fn neighbors_ord(&self) -> Vec<Point> {
        let mut result = self.neighbors();
        result.extend(
            ORDINAL_NEIGHBORS
                .iter()
                .map(|dir| *self + *dir)
                .collect::<Vec<_>>(),
        );
        result
    }

    pub fn neighbors_ord_with_dir(&self) -> Vec<(Point, Direction)> {
        let mut neighbors = self.neighbors_with_dir();
        neighbors.extend(&[
            (self.north(), Direction::NorthEast),
            (self.east(), Direction::SouthEast),
            (self.south(), Direction::SouthWest),
            (self.west(), Direction::NorthWest),
        ]);
        neighbors
    }

    /// Returns the Euclidian remander between rhs and self.
    pub fn rem_euclid(&mut self, rhs: Self) -> Self {
        Self {
            x: (self.x % rhs.x).abs(),
            y: (self.y % rhs.y).abs(),
        }
    }

    #[inline]
    /// Returns the neighbor to the north of this point
    pub fn north(&self) -> Point {
        *self + Self { x: 0, y: -1 }
    }
    #[inline]
    /// Returns the neighbor to the north east of this point
    pub fn north_east(&self) -> Point {
        *self + Self { x: 1, y: -1 }
    }
    #[inline]
    /// Returns the neighbor to the east of this point
    pub fn east(&self) -> Point {
        *self + Self { x: 1, y: 0 }
    }
    #[inline]
    /// Returns the neighbor to the south east of this point
    pub fn south_east(&self) -> Point {
        *self + Self { x: 1, y: 1 }
    }
    #[inline]
    /// Returns the neighbor to the south of this point
    pub fn south(&self) -> Point {
        *self + Self { x: 0, y: 1 }
    }
    #[inline]
    /// Returns the neighbor to the south west of this point
    pub fn south_west(&self) -> Point {
        *self + Self { x: -1, y: 1 }
    }
    #[inline]
    /// Returns the neighbor to the west of this point
    pub fn west(&self) -> Point {
        *self + Self { x: -1, y: 0 }
    }

    #[inline]
    /// Returns the neighbor to the north west of this point
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

/// Create a point from `(x: usize, y: usize)`
impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

/// Create a point from `(x: isize, y: isize)`
impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

/// Create a point from `(x: u64, y: u64)`
impl From<(u64, u64)> for Point {
    fn from(value: (u64, u64)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

/// Create a point from `(x: i64, y: i64)`
impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

/// Create a point from `(x: u32, y: u32)`
impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

/// Create a point from `(x: i32, y: i32)`
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

impl Add<(isize, isize)> for Point {
    type Output = Self;
    fn add(self, (x, y): (isize, isize)) -> Self::Output {
        Self {
            y: self.y + y,
            x: self.x + x,
        }
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

impl Add<isize> for Point {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Add<&isize> for Point {
    type Output = Self;
    fn add(self, rhs: &isize) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<isize> for Point {
    fn add_assign(&mut self, rhs: isize) {
        self.y += rhs;
        self.x += rhs;
    }
}

impl AddAssign<&isize> for Point {
    fn add_assign(&mut self, rhs: &isize) {
        self.y += rhs;
        self.x += rhs;
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

//----------------
impl Sub<i8> for Point {
    type Output = Self;
    fn sub(self, rhs: i8) -> Self {
        Self {
            y: self.y - rhs as isize,
            x: self.x - rhs as isize,
        }
    }
}

impl SubAssign<i8> for Point {
    fn sub_assign(&mut self, rhs: i8) {
        self.y -= rhs as isize;
        self.x -= rhs as isize;
    }
}

impl Sub<u8> for Point {
    type Output = Self;
    fn sub(self, rhs: u8) -> Self {
        Self {
            y: self.y - rhs as isize,
            x: self.x - rhs as isize,
        }
    }
}

impl SubAssign<u8> for Point {
    fn sub_assign(&mut self, rhs: u8) {
        self.y -= rhs as isize;
        self.x -= rhs as isize;
    }
}

impl Sub<i32> for Point {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self {
        Self {
            y: self.y - rhs as isize,
            x: self.x - rhs as isize,
        }
    }
}

impl SubAssign<i32> for Point {
    fn sub_assign(&mut self, rhs: i32) {
        self.y -= rhs as isize;
        self.x -= rhs as isize;
    }
}

impl Sub<u32> for Point {
    type Output = Self;
    fn sub(self, rhs: u32) -> Self {
        Self {
            y: self.y - rhs as isize,
            x: self.x - rhs as isize,
        }
    }
}

impl SubAssign<u32> for Point {
    fn sub_assign(&mut self, rhs: u32) {
        self.y -= rhs as isize;
        self.x -= rhs as isize;
    }
}

impl Sub<i64> for Point {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self {
        Self {
            y: self.y - rhs as isize,
            x: self.x - rhs as isize,
        }
    }
}

impl SubAssign<i64> for Point {
    fn sub_assign(&mut self, rhs: i64) {
        self.y -= rhs as isize;
        self.x -= rhs as isize;
    }
}

impl Sub<u64> for Point {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self {
        Self {
            y: self.y - rhs as isize,
            x: self.x - rhs as isize,
        }
    }
}

impl SubAssign<u64> for Point {
    fn sub_assign(&mut self, rhs: u64) {
        self.y -= rhs as isize;
        self.x -= rhs as isize;
    }
}

impl Sub<i128> for Point {
    type Output = Self;
    fn sub(self, rhs: i128) -> Self {
        Self {
            y: self.y - rhs as isize,
            x: self.x - rhs as isize,
        }
    }
}

impl SubAssign<i128> for Point {
    fn sub_assign(&mut self, rhs: i128) {
        self.y -= rhs as isize;
        self.x -= rhs as isize;
    }
}

impl Sub<u128> for Point {
    type Output = Self;
    fn sub(self, rhs: u128) -> Self {
        Self {
            y: self.y - rhs as isize,
            x: self.x - rhs as isize,
        }
    }
}

impl SubAssign<u128> for Point {
    fn sub_assign(&mut self, rhs: u128) {
        self.y -= rhs as isize;
        self.x -= rhs as isize;
    }
}

impl Sub<isize> for Point {
    type Output = Self;
    fn sub(self, rhs: isize) -> Self {
        Self {
            y: self.y - rhs,
            x: self.x - rhs,
        }
    }
}

impl SubAssign<isize> for Point {
    fn sub_assign(&mut self, rhs: isize) {
        self.y -= rhs;
        self.x -= rhs;
    }
}

impl Sub<usize> for Point {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        Self {
            y: self.y - rhs as isize,
            x: self.x - rhs as isize,
        }
    }
}

impl SubAssign<usize> for Point {
    fn sub_assign(&mut self, rhs: usize) {
        self.y -= rhs as isize;
        self.x -= rhs as isize;
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

impl Mul<i128> for Point {
    type Output = Self;
    fn mul(self, rhs: i128) -> Self::Output {
        Self {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

impl Mul<u128> for Point {
    type Output = Self;
    fn mul(self, rhs: u128) -> Self::Output {
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

impl Rem<isize> for Point {
    type Output = Self;
    fn rem(self, rhs: isize) -> Self::Output {
        Self {
            y: self.y % rhs,
            x: self.x % rhs,
        }
    }
}

impl Rem<&isize> for Point {
    type Output = Self;
    fn rem(self, rhs: &isize) -> Self::Output {
        Self {
            y: self.y % rhs,
            x: self.x % rhs,
        }
    }
}
