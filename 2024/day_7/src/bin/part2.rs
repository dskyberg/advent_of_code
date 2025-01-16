use anyhow::Result;

use aoc_utils::*;
use day_7::*;

fn main() -> Result<()> {
    let mut aoc = Aoc::part2();

    let equations = read_data(INPUT)?;
    let result = equations
        .iter()
        .map(|eq| match eq.is_valid2() {
            true => eq.result,
            false => 0,
        })
        .sum::<ValueType>();

    aoc.result(result as usize);
    println!("{}", aoc);
    Ok(())
}
