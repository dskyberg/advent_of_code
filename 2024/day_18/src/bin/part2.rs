use anyhow::Result;

use aoc_utils::*;
use day_18::*;
use pathfinding::prelude::dijkstra;

fn main() -> Result<()> {
    let result = 0;
    let mut aoc = Aoc::part2();
    let dim = 7;
    let max_drops = 12;

    let matrix = parse_input(TEST_INPUT, dim, max_drops)?;
    let start = Point::from((0, 0));
    let goal = Point::from((dim - 1, dim - 1));

    let successors = |p: &Point| {
        p.neighbors_hv()
            .into_iter()
            .filter(|p| matrix.valid_point(p) && matrix.get_unsafe(p) != b'#')
            .map(|p| (p, 1))
            .collect::<Vec<(Point, usize)>>()
    };
    let (_, points) = points(TEST_INPUT)?;
    for point in points[12..].iter() {
        if dijkstra(&start, successors, |p| *p == goal).is_none() {
            // Winner winner!
            println!("First is {}", point);
        }
    }

    println!("{}", aoc);
    Ok(())
}
