use anyhow::Result;
use std::collections::HashSet;

use aoc_utils::*;

use day_6::*;

#[derive(Debug, Clone)]
struct Map {
    max_row: isize,
    max_col: isize,
    matrix: Matrix<u8>,
    visited: HashSet<Point>,
    curr_pos: Point,
    curr_dir: Direction,
}

impl Map {
    fn new(data: &str) -> Result<Self> {
        let (matrix, start_pos) = read_data(data)?;
        let mut visited = HashSet::new();
        let curr_pos = start_pos;
        visited.insert(curr_pos);

        Ok(Map {
            max_row: matrix.height as isize - 1,
            max_col: matrix.width as isize - 1,
            matrix,
            visited,
            curr_pos,
            curr_dir: Direction::default(),
        })
    }

    #[inline]
    fn turn(&mut self) {
        self.curr_dir = self.curr_dir.right();
    }

    #[inline]
    fn on_edge(&self) -> bool {
        self.curr_pos.y == 0
            || self.curr_pos.y == self.max_row
            || self.curr_pos.x == 0
            || self.curr_pos.x == self.max_col
    }

    fn try_step(&self) -> Option<Point> {
        match (&self.curr_dir, (self.curr_pos.y, self.curr_pos.x)) {
            (Direction::North, (row, col)) => {
                if row > 0 {
                    Some(Point { y: row - 1, x: col })
                } else {
                    None
                }
            }
            (Direction::South, (row, col)) => {
                if row < self.max_row {
                    Some(Point { x: col, y: row + 1 })
                } else {
                    None
                }
            }
            (Direction::East, (row, col)) => {
                if col < self.max_col {
                    Some(Point { y: row, x: col + 1 })
                } else {
                    None
                }
            }
            (Direction::West, (row, col)) => {
                if col > 0 {
                    Some(Point { y: row, x: col - 1 })
                } else {
                    None
                }
            }
            (d, _) => panic!("{} is not a valid direction in this context", d),
        }
    }

    /// Returns true if there are more steps.
    /// Returns false when an edge has been reached.
    fn step(&mut self) -> bool {
        // If this fails, it's because we're on an edge, and missed it.
        let Some(pos) = self.try_step() else {
            panic!("Trying to step when already on an edge!");
        };

        self.visited.insert(pos);
        self.curr_pos = pos;

        // Is this the end?
        if self.on_edge() {
            return false;
        }

        // See if we need to turn
        let Some(pos) = self.try_step() else {
            panic!("Trying to step when already on an edge!");
        };
        if self.matrix.get_unsafe(&pos) == BLOCKED {
            self.turn()
        }
        true
    }

    fn run_simulation(&mut self) -> usize {
        while self.step() {}
        self.visited.len()
    }
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();

    let mut map = Map::new(INPUT)?;
    let result = map.run_simulation();

    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
