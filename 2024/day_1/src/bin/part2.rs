use anyhow::Result;

use aoc_utils::*;
use day_1::*;

fn main() -> Result<()> {
    let mut aoc = Aoc::part2();
    let data = read_data(INPUT)?;

    let mut total = 0;
    for val in data.left {
        let len = data
            .right
            .iter()
            .filter(|&&x| x == val)
            .collect::<Vec<&usize>>()
            .len();
        total += val * len;
    }
    aoc.result(total);
    println!("{}", aoc);

    Ok(())
}
