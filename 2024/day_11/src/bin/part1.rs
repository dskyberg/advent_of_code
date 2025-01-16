use anyhow::Result;

use aoc_utils::*;
use day_11::*;

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();

    let mut data = line_to_numbers::<usize>(INPUT, ' ')?;
    for _ in 0..25 {
        data = blink(&data)?;
    }

    aoc.result(data.len());
    println!("{}", aoc);
    Ok(())
}
