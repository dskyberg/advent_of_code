use anyhow::Result;

use aoc_utils::*;
use day_14::*;

fn sort_quads(lobby: &Lobby) -> [usize; 4] {
    let mut quads = [0_usize; 4];

    for robot in &lobby.robots {
        let x = robot.pos.x.cmp(&(lobby.max.x / 2));
        let y = robot.pos.y.cmp(&(lobby.max.y / 2));

        if let Some(q) = match (x, y) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Some(0),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Some(1),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Some(2),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Some(3),
            _ => None,
        } {
            quads[q] += 1;
        }
    }
    quads
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();

    let mut lobby = Lobby::new(INPUT, 103, 101)?;

    for _ in 0..100 {
        lobby.second();
    }

    // Calc quadrants
    let quads = sort_quads(&lobby);
    let result: usize = quads.iter().product();

    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
