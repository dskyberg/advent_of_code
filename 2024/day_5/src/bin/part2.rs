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

    rules.iter().all(|(d, p)| match (map.get(d), map.get(p)) {
        (Some(x), Some(y)) => x < y,
        _ => true,
    })
}

fn sort_updates(rules: &[(u32, u32)], updates: &[u32]) -> Vec<u32> {
    let mut result = updates.to_vec().clone();
    result.sort_by(|&x, &y| {
        if rules.contains(&(x, y)) {
            std::cmp::Ordering::Less
        } else if rules.contains(&(y, x)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    result.to_vec()
}

fn main() -> Result<()> {
    let mut result = 0;
    let mut aoc = Aoc::part2();
    let (rules, updates) = parse_input(INPUT)?;

    for update in updates {
        if !is_correct_order(&rules, &update) {
            let sorted = sort_updates(&rules, &update);
            result += sorted[sorted.len() / 2];
        }
    }

    aoc.result(result as usize);
    println!("{}", aoc);
    Ok(())
}
