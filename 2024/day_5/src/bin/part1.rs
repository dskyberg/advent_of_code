use anyhow::Result;
use std::collections::HashMap;

use aoc_utils::*;
use day_5::*;

fn is_correct_order(rules: &[(u32, u32)], updates: &[u32]) -> bool {
    let map: HashMap<u32, usize> = updates
        .iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect();

    rules.iter().all(|(x, y)| match (map.get(x), map.get(y)) {
        (Some(x), Some(y)) => x < y,
        _ => true,
    })
}

fn main() -> Result<()> {
    let mut result = 0;
    let mut aoc = Aoc::part1();
    let (rules, updates) = parse_input(INPUT)?;

    for update in updates {
        if is_correct_order(&rules, &update) {
            result += update[update.len() / 2];
        }
    }

    aoc.result(result as usize);
    println!("{}", aoc);
    Ok(())
}
