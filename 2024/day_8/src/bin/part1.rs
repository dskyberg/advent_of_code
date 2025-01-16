use anyhow::Result;
use std::collections::HashSet;

use aoc_utils::*;
use day_8::*;

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();

    let board = read_data(INPUT)?;
    let mut antinodes = HashSet::<Point>::new();

    for locs in board.map.values() {
        // Every pair of antennae are in a straight line.  So, we just need to
        // determine if the distance between 2 antennae can be extended onto the board
        for (i, a1) in locs.iter().enumerate() {
            for a2 in locs[i + 1..].iter() {
                let distance = *a1 - *a2;

                let p1 = *a1 + distance;
                if board.valid_point(&p1) {
                    antinodes.insert(p1);
                }

                let p2 = *a2 - distance;
                if board.valid_point(&p2) {
                    antinodes.insert(p2);
                }
            }
        }
    }

    aoc.result(antinodes.len());
    println!("{}", aoc);
    Ok(())
}
