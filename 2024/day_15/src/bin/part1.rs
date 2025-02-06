use anyhow::Result;
use std::fmt::Display;
use std::str::FromStr;

use aoc_utils::*;
use day_15::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Element {
    Wall,
    Box,
    Robot,
    Empty,
}

impl TryFrom<char> for Element {
    type Error = Day15Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            '@' => Ok(Self::Robot),
            '.' => Ok(Self::Empty),
            _ => Err(Day15Error::ElementParse(value)),
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Box => write!(f, "O"),
            Self::Robot => write!(f, "@"),
            Self::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug)]
pub struct Warehouse {
    pub matrix: Matrix<Element>,
    pub moves: Vec<Direction>,
    pub robot: Point,
}

impl Warehouse {
    pub fn step(&mut self, p: &Point, d: Direction) {
        // Get the new point after moving
        let new_p = p.neighbor(d);

        // If there is a wall in the way, return
        if self.matrix.get_unsafe(&new_p) == Element::Wall {
            return;
        }

        // If there is a box in the way, push it
        if self.matrix.get_unsafe(&new_p) == Element::Box {
            self.step(&new_p, d);
        }

        // If the space isn't empty, just bail
        if self.matrix.get_unsafe(&new_p) == Element::Box {
            return;
        }

        // The space is empty.  Swap the elements
        let curr_elem = self.matrix.get_unsafe(p);
        self.matrix.set_unsafe(&new_p, curr_elem);
        self.matrix.set_unsafe(p, Element::Empty);

        if curr_elem == Element::Robot {
            self.robot = new_p;
        }
    }

    pub fn predict(&mut self) {
        for d in self.moves.clone() {
            let robot = self.robot;
            self.step(&robot, d);
        }
    }

    pub fn sum_boxes(&self) -> usize {
        self.matrix.iter().fold(0, |acc, (p, e)| match e {
            Element::Box => acc + p.y as usize * 100 + p.x as usize,
            _ => acc,
        })
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (p, c) in self.matrix.iter() {
            write!(f, "{}", c)?;
            if p.x == self.matrix.width as isize - 1 {
                writeln!(f)?;
            }
        }
        writeln!(f)
    }
}

impl FromStr for Warehouse {
    type Err = Day15Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        // split the input into the map and the moves
        let (map, moves) = input.split_once("\n\n").ok_or(Day15Error::MalformedInput)?;

        // Convert the map to Matrix::<Element>
        let matrix = map.parse::<Matrix<Element>>()?;

        // Since the moves in the puzzle input are multiple lines, just filter out '\n',
        // and map the u8 to Directions.
        let moves = moves
            .as_bytes()
            .iter()
            .filter(|b| **b != b'\n')
            .map(b2d) // Convert the u8 to a Direction
            .collect::<Vec<Direction>>();

        // Find the robot location in the map
        let robot = matrix.find_one(&Element::Robot).unwrap();

        Ok(Warehouse {
            matrix,
            moves,
            robot,
        })
    }
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();
    let mut warehouse = INPUT.parse::<Warehouse>()?;

    warehouse.predict();
    let result = warehouse.sum_boxes();
    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let warehouse = TEST_INPUT.parse::<Warehouse>().expect("Failed");
        dbg!(warehouse);
    }
}
