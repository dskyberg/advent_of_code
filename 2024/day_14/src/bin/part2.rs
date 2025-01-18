use anyhow::Result;

use aoc_utils::*;
use day_14::*;

fn find_cluster(lobby: &mut Lobby) -> usize {
    let mut ticks = 0;

    // Horiz: 63 166
    //verticle:  13 114

    let mut hp = 64;
    let mut vp = 13;

    while ticks < 7893 {
        lobby.second();
        ticks += 1;
        if ticks == hp || ticks == vp {
            println!("After {ticks} seconds");
            println!("{}", lobby);
            if ticks == hp {
                hp += 103;
            } else {
                vp += 101;
            }
        }
    }
    ticks
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part2();

    let mut lobby = Lobby::new(INPUT, 103, 101)?;

    let result = find_cluster(&mut lobby);
    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
