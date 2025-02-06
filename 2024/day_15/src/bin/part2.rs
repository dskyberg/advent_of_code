use anyhow::Result;
use std::collections::VecDeque;
use std::fmt::Display;
use std::str::FromStr;

use aoc_utils::*;
use day_15::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Element {
    Wall,
    LBox,
    RBox,
    Robot,
    Empty,
}

impl TryFrom<char> for Element {
    type Error = Day15Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            '[' => Ok(Self::LBox),
            ']' => Ok(Self::RBox),
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
            Self::LBox => write!(f, "["),
            Self::RBox => write!(f, "]"),
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
    fn find_robot(&self) -> Point {
        self.matrix
            .find_one(&Element::Robot)
            .expect("Could not locate robot")
    }

    fn move_boxes(&mut self, starting_point: Point, dir: Direction) {
        let mut queue = VecDeque::from([starting_point]);
        let mut seen: Vec<Point> = Vec::from([]);

        while let Some(point) = queue.pop_front() {
            if !seen.contains(&point) {
                seen.push(point);
                let new_p = point.neighbor(dir);

                match self.matrix.get_unsafe(&new_p) {
                    Element::Wall => return,
                    Element::LBox => queue.extend([new_p, new_p.neighbor(Direction::East)]),
                    Element::RBox => queue.extend([new_p.neighbor(Direction::West), new_p]),
                    Element::Empty => continue,
                    _ => unreachable!("Invalid element"),
                }
            }
        }

        match dir {
            Direction::East => seen.sort_by(|lhs, rhs| rhs.x.cmp(&lhs.x)),
            Direction::South => seen.sort_by(|lhs, rhs| rhs.y.cmp(&lhs.y)),
            Direction::West => seen.sort_by(|lhs, rhs| lhs.x.cmp(&rhs.x)),
            Direction::North => seen.sort_by(|lhs, rhs| lhs.y.cmp(&rhs.y)),
            _ => unreachable!("Unsupported Direction"),
        }

        for point in seen {
            let new_p = point.neighbor(dir);
            self.matrix.swap_unsafe(&point, &new_p);
        }
    }

    fn move_robot(&mut self) {
        for dir in self.moves.clone() {
            let robot = self.find_robot();
            let new_p = robot.neighbor(dir);
            match self.matrix.get_unsafe(&new_p) {
                Element::Wall => continue,
                Element::LBox | Element::RBox => self.move_boxes(robot, dir),
                Element::Empty => self.matrix.swap_unsafe(&robot, &new_p),
                Element::Robot => unreachable!("Cannot be more than 1 robot"),
            }
        }
    }

    pub fn sum_boxes(&self) -> usize {
        self.matrix.iter().fold(0, |acc, (p, e)| match e {
            Element::LBox => acc + p.y as usize * 100 + p.x as usize,
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
        let (map_str, moves) = input.split_once("\n\n").ok_or(Day15Error::MalformedInput)?;

        // Handle the map in 2 steps:
        // 1. parse to a grid
        let map = data_to_grid::<char>(map_str).map_err(|_| Day15Error::MalformedInput)?;

        // 2. expand the grid
        let mut matrix: Vec<Vec<Element>> = Vec::new();
        for row in map {
            let mut e_row = Vec::new();
            for c in row {
                match c {
                    '#' => {
                        e_row.push(Element::Wall);
                        e_row.push(Element::Wall)
                    }
                    '.' => {
                        e_row.push(Element::Empty);
                        e_row.push(Element::Empty)
                    }
                    'O' => {
                        e_row.push(Element::LBox);
                        e_row.push(Element::RBox)
                    }
                    '@' => {
                        e_row.push(Element::Robot);
                        e_row.push(Element::Empty)
                    }
                    _ => panic!("Unexpected map char: {}", c),
                }
            }
            matrix.push(e_row);
        }

        // Convert the Vec<Vec<Element>> to Matrix::<Element>
        let matrix = Matrix::try_from(matrix)?;

        let moves = moves
            .as_bytes()
            .iter()
            .filter(|b| **b != b'\n') // convert to a single line
            .map(b2d) // Convert the u8 to a Direction
            .collect::<Vec<Direction>>();

        let robot = matrix.find_one(&Element::Robot).unwrap();

        Ok(Warehouse {
            matrix,
            moves,
            robot,
        })
    }
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part2();
    let mut warehouse = INPUT.parse::<Warehouse>()?;

    warehouse.move_robot();
    let result = warehouse.sum_boxes();
    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
