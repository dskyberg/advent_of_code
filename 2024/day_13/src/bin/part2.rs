use anyhow::Result;

use aoc_utils::*;
use day_13::*;

fn main() -> Result<()> {
    let mut aoc = Aoc::part2();

    let machines: Vec<Machine> = read_data(INPUT)?
        .iter()
        .map(|m| Machine {
            a: m.a,
            b: m.b,
            prize: Point::from((
                m.prize.x + 10_000_000_000_000,
                m.prize.y + 10_000_000_000_000,
            )),
        })
        .collect();
    let result: isize = machines.iter().flat_map(|m| m.solve2()).sum();

    aoc.result(result as usize);
    println!("{}", aoc);
    Ok(())
}
