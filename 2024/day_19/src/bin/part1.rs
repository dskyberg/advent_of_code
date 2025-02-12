use anyhow::Result;
use std::collections::HashMap;

use aoc_utils::*;
use day_19::*;

fn solve<'a>(problem: &'a str, memo: &mut HashMap<&'a str, bool>, patterns: &[&'a str]) -> bool {
    if problem.is_empty() {
        return true;
    }

    if let Some(&result) = memo.get(problem) {
        return result;
    }

    for pattern in patterns {
        if problem.starts_with(pattern) && solve(&problem[pattern.len()..], memo, patterns) {
            return true;
        }
    }
    memo.insert(problem, false);
    false
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part1();
    let (s1, s2) = INPUT.split_once("\n\n").unwrap();
    let mut memo = HashMap::new();

    let patterns = s1.split(", ").collect::<Vec<_>>();
    let problems = s2.lines().collect::<Vec<_>>();

    let result = problems
        .iter()
        .filter(|p| solve(p, &mut memo, &patterns))
        .count();
    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
