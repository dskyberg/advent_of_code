use anyhow::Result;

use aoc_utils::*;
use day_17::*;

fn main() -> Result<()> {
    let result = 0;
    let mut aoc = Aoc::part1();
    let mut computer = Computer::new([164279024971453, 0, 0], &[
        2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 5, 5, 0, 3, 3, 0,
    ]);
    let output = computer
        .run()
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join(",");

    println!("{}", output);
    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}

// Bad: 417641027
