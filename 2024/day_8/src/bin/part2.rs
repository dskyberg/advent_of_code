use anyhow::Result;
use std::collections::HashSet;

use aoc_utils::*;
use day_8::*;

fn main() -> Result<()> {
    let mut aoc = Aoc::part2();
    let board = read_data(INPUT)?;

    let mut antinodes = HashSet::<Point>::new();

    for locs in board.map.values() {
        // Every pair of antennae are in a straight line.  So, we just need to
        // determine if the distance between 2 antennae can be extended onto the board
        for (i, a1) in locs.iter().enumerate() {
            antinodes.insert(*a1);
            for a2 in locs[i + 1..].iter() {
                antinodes.insert(*a2);
                let distance = *a1 - *a2;
                let mut factor = 0;
                loop {
                    factor += 1;
                    let p = *a1 + distance * factor;
                    if board.valid_point(&p) {
                        antinodes.insert(p);
                    } else {
                        break;
                    }
                }
                factor = 0;
                loop {
                    factor += 1;
                    let p = *a2 - distance * factor;
                    if board.valid_point(&p) {
                        antinodes.insert(p);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    aoc.result(antinodes.len());
    println!("{}", aoc);
    Ok(())
}
