use anyhow::Result;
use std::collections::{BTreeMap, HashMap, HashSet};

use aoc_utils::*;
use day_20::*;

/// This solution runs faster than using Race::solve
fn solve(race: &Race) -> Result<usize> {
    let mut cheats = BTreeMap::<usize, usize>::new();
    let mut visited = HashSet::new();
    // Convert to a hash map for lookups
    let mut hashed_path = HashMap::<Point, usize>::new();
    for (i, p) in race.track.iter().enumerate() {
        hashed_path.insert(*p, i);
    }

    // 2 steps in each direction
    let steps: [[(isize, isize); 2]; 4] =
        [[(0, -1), (0, -2)], [(0, 1), (0, 2)], [(-1, 0), (-2, 0)], [
            (1, 0),
            (2, 0),
        ]];

    for (idx, point) in race.track.iter().enumerate() {
        for step in steps {
            let step0 = *point + step[0];

            if visited.contains(&step0) {
                continue;
            }
            visited.insert(step0);

            // If the first step is not a wall, ignore
            if race.matrix.compare(&step0, b'#') != Some(true) {
                continue;
            }

            let step1 = *point + step[1];
            // If the second step is not on the racetrack, ignore
            let Some(&cost) = hashed_path.get(&step1) else {
                continue;
            };

            // calculate the savings and add to the list
            let diff = cost.abs_diff(idx + 2);
            if diff >= 100 {
                *cheats.entry(diff).or_default() += 1;
            }
        }
    }

    let mut result = 0;
    for value in cheats.values() {
        result += *value;
    }
    Ok(result)
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();

    let race = INPUT.parse::<Race>()?;
    // println!("{}", &race);
    let result = solve(&race)?;

    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
