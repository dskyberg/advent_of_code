use anyhow::Result;
use std::collections::HashSet;
use thiserror::Error;

use aoc_utils::*;

use day_6::*;

#[derive(Debug, Error)]
enum AppError {
    #[error("In a time loop")]
    TimeLoop,
}

#[derive(Debug, Clone)]
struct Map {
    max_row: usize,
    max_col: usize,
    matrix: Matrix<u8>,
    visited: HashSet<(Point, Direction)>,
    start_pos: Point,
    last_changed: Option<Point>,
    curr_pos: Point,
    curr_dir: Direction,
}
impl Map {
    fn new(data: &str) -> Result<Self> {
        let (matrix, start_pos) = read_data(data)?;
        let mut visited = HashSet::new();
        let curr_pos = start_pos;
        let last_changed = None;
        visited.insert((curr_pos, Direction::North));

        Ok(Map {
            max_row: matrix.len() - 1,
            max_col: matrix[0].len() - 1,
            matrix,
            visited,
            start_pos,
            curr_pos,
            last_changed,
            curr_dir: Direction::default(),
        })
    }

    fn reset(&mut self) {
        self.visited.clear();
        self.curr_pos = self.start_pos;
        self.curr_dir = Direction::default();
        if let Some(pos) = &self.last_changed {
            self.matrix[pos.y as usize][pos.x as usize] = OPEN;
        }
        self.last_changed = None;
    }

    #[inline]
    fn turn(&mut self) {
        self.curr_dir = self.curr_dir.right();
    }

    #[inline]
    /// Returns true of the current position is an edge.
    fn on_edge(&self) -> bool {
        self.curr_pos.y == 0
            || self.curr_pos.y as usize == self.max_row
            || self.curr_pos.x as usize == 0
            || self.curr_pos.x as usize == self.max_col
    }

    /// Adds a block, `'#'`, at the given position.
    fn add_block(&mut self, pos: &Point) -> bool {
        if *pos == self.start_pos {
            return false;
        }
        if self.matrix.get_unsafe(pos) == OPEN {
            self.matrix.set_unsafe(pos, BLOCKED);
            self.last_changed = Some(*pos);
            return true;
        }
        false
    }

    /// Returns the next position if moving forward is valid.  Else None.
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
                if row < self.max_row as isize {
                    Some(Point { x: col, y: row + 1 })
                } else {
                    None
                }
            }
            (Direction::East, (row, col)) => {
                if col < self.max_col as isize {
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
            (d, _) => panic!("{} is not valid in this context.", d),
        }
    }

    /// Returns true if there are more steps.
    /// Returns false when an edge has been reached.
    /// Returns an error if it looks like we're in a loop.
    fn step(&mut self) -> Result<bool> {
        // If this fails, it's because we're on an edge, and missed it.
        let Some(pos) = self.try_step() else {
            panic!("Trying to step when already on an edge!");
        };

        if !self.visited.insert((pos, self.curr_dir)) {
            // Looks like we're in a time loop!
            return Err(AppError::TimeLoop.into());
        }

        self.curr_pos = pos;

        // Is this the end?
        if self.on_edge() {
            return Ok(false);
        }

        // See if we need to turn
        let Some(pos) = self.try_step() else {
            panic!("Trying to step when already on an edge!");
        };

        if self.matrix.get_unsafe(&pos) == BLOCKED {
            self.turn();
            // Recheck if this is now a loop.  So we can bail early.
            if self.visited.contains(&(self.curr_pos, self.curr_dir)) {
                // oops!  Looks like we enterd a time loop!
                return Err(AppError::TimeLoop.into());
            }
        }
        Ok(true)
    }

    fn is_a_loop(&mut self) -> bool {
        loop {
            let result = self.step();
            if result.is_err() {
                return true;
            }
            if !result.unwrap() {
                return false;
            }
        }
    }
}

fn main() -> Result<()> {
    let mut result = 0;
    let mut aoc = Aoc::part2();

    let mut map = Map::new(INPUT)?;
    for row in 0..=map.max_row {
        for col in 0..=map.max_col {
            let pos = Point::from((col, row));
            if map.add_block(&pos) {
                if map.is_a_loop() {
                    result += 1;
                }
                map.reset();
            }
        }
    }

    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
