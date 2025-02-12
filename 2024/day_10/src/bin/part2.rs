use anyhow::Result;

use aoc_utils::*;
use day_10::*;

fn walk_path(stop: u8, from: &Point, max: u8, dag: &DAG) -> Result<u32> {
    let mut result = 0;
    for node in (*dag).get(&stop).ok_or(Day10Error::IncompleteDAG)? {
        if node.touches(from) {
            if stop == max {
                result += 1;
            } else {
                result += walk_path(stop + 1, node, max, dag)?;
            }
        }
    }
    Ok(result)
}

/// Count how many different ways max can be reach from each trail head
fn count_trails(dag: &DAG) -> Result<u32> {
    let mut result = 0;

    // Get all the zeros
    let trail_heads = (*dag).get(&TRAIL_HEAD_HEIGHT).unwrap();

    for trail_head in trail_heads {
        result += walk_path(1, trail_head, TRAIL_MAX_HEIGHT, dag)?;
    }
    Ok(result)
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part2();

    let matrix = data_to_digits::<u8>(INPUT)?;
    let dag = DAG::from(&matrix);

    let result = count_trails(&dag)?;
    aoc.result(result as usize);
    println!("{}", aoc);
    Ok(())
}
