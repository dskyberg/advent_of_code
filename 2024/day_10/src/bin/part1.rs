use anyhow::Result;
use std::collections::HashSet;

use aoc_utils::*;
use day_10::*;

fn walk_path(stop: u8, from: &Point, max: u8, maxes: &mut HashSet<Point>, dag: &DAG) -> Result<()> {
    for node in (*dag).get(&stop).ok_or(Day10Error::IncompleteDAG)? {
        if node.touches(from) {
            if stop == max {
                maxes.insert(*node);
            } else {
                walk_path(stop + 1, node, max, maxes, dag)?;
            }
        }
    }
    Ok(())
}

/// Count how many different maxes can be reached from the trail head.
/// This would be safer if we didn't assum the min (trail head) and max
fn count_trails(dag: &DAG) -> Result<usize> {
    let mut result = 0;

    // Get all the zeros
    let trail_heads = (*dag).get(&TRAIL_HEAD_HEIGHT).unwrap();

    for trail_head in trail_heads {
        let mut maxes = HashSet::new();
        walk_path(1, trail_head, TRAIL_MAX_HEIGHT, &mut maxes, dag)?;
        result += maxes.len();
    }
    Ok(result)
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();

    let matrix = data_to_digits::<u8>(INPUT)?;
    let dag = DAG::from(&matrix);

    let result = count_trails(&dag)?;

    aoc.result(result as usize);
    println!("{}", aoc);
    Ok(())
}
