use anyhow::Result;

use aoc_utils::*;
use day_12::*;

#[derive(Debug)]
struct Garden {
    pub regions: Vec<Region>,
}

impl Garden {
    fn cost(&self) -> usize {
        self.regions
            .iter()
            .fold(0, |acc, r| acc + r.area() * r.perimeter())
    }
}

impl From<&str> for Garden {
    fn from(value: &str) -> Self {
        let (_, regions) = read_data(value);
        Self { regions }
    }
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();

    let garden = Garden::from(INPUT);
    let result = garden.cost();

    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
