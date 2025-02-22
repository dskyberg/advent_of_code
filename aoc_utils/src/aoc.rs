use std::fmt::Display;
use std::time::Instant;

pub struct Aoc {
    part: u32,
    pub now: Instant,
    pub result: usize,
}

impl Aoc {
    pub fn new() -> Self {
        Self {
            part: 0,
            now: Instant::now(),
            result: 0,
        }
    }

    pub fn part1() -> Self {
        Self::new().part(1)
    }

    pub fn part2() -> Self {
        Self::new().part(2)
    }

    fn part(self, part: u32) -> Self {
        Self { part, ..self }
    }

    pub fn result(&mut self, result: usize) -> &Self {
        self.result = result;
        self
    }
}

impl Default for Aoc {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Aoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Part {}: {} - {:.2?}",
            self.part,
            self.result,
            self.now.elapsed()
        )
    }
}
