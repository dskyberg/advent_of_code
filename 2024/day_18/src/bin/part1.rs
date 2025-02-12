use anyhow::Result;

use aoc_utils::*;
use day_18::*;
use pathfinding::prelude::dijkstra;

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();
    let matrix = parse_input(INPUT, 71, 1024)?;
    let start = Point::from((0, 0));
    let goal = Point::from((70, 70));

    let successors = |p: &Point| {
        p.neighbors()
            .into_iter()
            .filter(|p| matrix.valid_point(p) && matrix.get_unsafe(p) != b'#')
            .map(|p| (p, 1))
            .collect::<Vec<(Point, usize)>>()
    };
    let (_path, cost) = dijkstra(&start, successors, |p| *p == goal).ok_or(Error::NoRoute)?;

    aoc.result(cost);
    println!("{}", aoc);
    Ok(())
}

/*
...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#....*/
