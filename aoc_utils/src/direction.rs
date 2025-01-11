use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DirectionError {
    #[error("Error parsing Direction")]
    FromStr,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub enum Direction {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        value as u8
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::North,
            1 => Self::NorthEast,
            2 => Self::East,
            3 => Self::SouthEast,
            4 => Self::South,
            5 => Self::SouthWest,
            6 => Self::West,
            7 => Self::NorthWest,
            _ => panic!("{} is not a Direction", value),
        }
    }
}

impl Direction {
    /// Useful if you want to limit directions to the 4 major points: North, South, East, and West
    pub fn is_major(&self) -> bool {
        *self as u8 % 2 == 0
    }

    /// Turn right to the next major direction (North, South, East, West)
    pub fn right(&self) -> Self {
        ((*self as u8 + 2) % 8).into()
    }

    /// Turn right to the next direction (North, NorthEast, East, SouthEast, South, etc)
    pub fn minor_right(&self) -> Self {
        ((*self as u8 + 1) % 8).into()
    }

    /// Turn left to the next major direction (North, West, South, East)
    pub fn left(&self) -> Self {
        match *self as u8 {
            0 => Self::West,
            n => (n - 2).into(),
        }
    }

    /// Turn left to the next direction (North, NorthWest, West, SouthWest, South, etc)
    pub fn minor_left(&self) -> Self {
        match *self as u8 {
            0 => Self::West,
            n => (n - 1).into(),
        }
    }

    /// Turn 180 degrees to the opposit direction
    pub fn turn_around(&self) -> Self {
        ((*self as u8 + 4) % 8).into()
    }
}

impl std::str::FromStr for Direction {
    type Err = DirectionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "north" | "n" => Ok(Self::North),
            "northeast" | "ne" => Ok(Self::NorthEast),
            "east" | "e" => Ok(Self::East),
            "southeast" | "se" => Ok(Self::SouthEast),
            "south" | "s" => Ok(Self::South),
            "southwest" | "sw" => Ok(Self::SouthWest),
            "west" | "w" => Ok(Self::West),
            "northwest" | "nw" => Ok(Self::NorthWest),
            _ => Err(DirectionError::FromStr),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_from_u8() {
        assert_eq!(Direction::North as u8, 0);
        assert_eq!(Direction::from(0), Direction::North);
    }

    #[test]
    #[should_panic(expected = "10 is not a Direction")]
    fn test_to_from_u8_with_panic() {
        let _will_panic = Direction::from(10);
    }

    #[test]
    fn test_turn_right() {
        assert_eq!(Direction::North.right(), Direction::East);
        assert_eq!(Direction::East.right(), Direction::South);
        assert_eq!(Direction::South.right(), Direction::West);
        assert_eq!(Direction::West.right(), Direction::North);
    }

    #[test]
    fn test_turn_left() {
        assert_eq!(Direction::North.left(), Direction::West);
        assert_eq!(Direction::East.left(), Direction::North);
        assert_eq!(Direction::South.left(), Direction::East);
        assert_eq!(Direction::West.left(), Direction::South);
    }
    #[test]
    fn test_turn_around() {
        assert_eq!(Direction::North.turn_around(), Direction::South);
        assert_eq!(Direction::NorthEast.turn_around(), Direction::SouthWest);
        assert_eq!(Direction::East.turn_around(), Direction::West);
        assert_eq!(Direction::SouthEast.turn_around(), Direction::NorthWest);
        assert_eq!(Direction::South.turn_around(), Direction::North);
        assert_eq!(Direction::SouthWest.turn_around(), Direction::NorthEast);
        assert_eq!(Direction::West.turn_around(), Direction::East);
        assert_eq!(Direction::NorthWest.turn_around(), Direction::SouthEast);
    }

    #[test]
    fn test_from_str() {
        assert_eq!("west".parse(), Ok(Direction::West));
        assert_eq!("w".parse(), Ok(Direction::West));
    }
}
