use anyhow::Result;

use aoc_utils::*;
use day_20::*;

fn main() -> Result<()> {
    let race = INPUT.parse::<Race>()?;
    let mut aoc = Aoc::part2();

    // println!("{}", &race);
    let result = race.solve(20, 100)?;
    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
