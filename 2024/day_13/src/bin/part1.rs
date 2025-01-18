use anyhow::Result;
use pathfinding::prelude::dijkstra;

use aoc_utils::*;
use day_13::*;

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();

    let machines: Vec<Machine> = read_data(INPUT)?;

    let result: isize = machines
        .iter()
        .flat_map(|machine| {
            let start_node = Point::from((0, 0));
            let result = dijkstra(
                &start_node,
                |pos| {
                    if pos.x > machine.prize.x || pos.y > machine.prize.y {
                        vec![]
                    } else {
                        vec![(*pos + machine.a, A_COST), (*pos + machine.b, B_COST)]
                    }
                },
                |&p| p == machine.prize,
            );

            result.map(|(_, cost)| cost)
        })
        .sum();

    aoc.result(result as usize);
    println!("{}", aoc);
    Ok(())
}
