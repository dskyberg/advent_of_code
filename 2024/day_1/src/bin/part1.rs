use anyhow::Result;
use aoc_utils::*;

use day_1::*;

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();

    let data = read_data(INPUT)?;

    let mut total = 0;
    for i in 0..data.left.len() {
        let left = data.left[i];
        let right = data.right[i];
        let max = left.max(right);
        let min = left.min(right);
        total += max - min;
    }
    aoc.result(total);
    println!("{}", aoc);
    Ok(())
}
