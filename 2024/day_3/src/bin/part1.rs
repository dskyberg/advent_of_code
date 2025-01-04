use anyhow::Result;

use aoc_utils::*;
use day_3::*;

fn main() -> Result<()> {
    let mut result = 0;
    let mut aoc = Aoc::part1();

    let data = read_data(INPUT)?;
    for line in data {
        result += process_line(&line);
    }

    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
