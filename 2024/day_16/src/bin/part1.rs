use anyhow::Result;

use aoc_utils::*;
use day_16::*;

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();
    let mut maze = INPUT.parse::<Maze>()?;
    let (result, _) = maze.solve();
    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
