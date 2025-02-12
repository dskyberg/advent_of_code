use anyhow::Result;
use std::collections::HashMap;

use aoc_utils::*;
use day_19::*;

fn solve<'a>(problem: &'a str, memo: &mut HashMap<&'a str, usize>, patterns: &[&'a str]) -> usize {
    if problem.is_empty() {
        return 1;
    }

    if let Some(&count) = memo.get(problem) {
        return count;
    }

    let mut count = 0;
    for pattern in patterns {
        if let Some(end) = problem.strip_prefix(pattern) {
            count += solve(end, memo, patterns);
        }
    }
    memo.insert(problem, count);
    count
}

fn main() -> Result<()> {
    let mut aoc = Aoc::part2();
    let (s1, s2) = INPUT.split_once("\n\n").unwrap();
    let mut memo = HashMap::new();

    let patterns = s1.split(", ").collect::<Vec<_>>();
    let problems = s2.lines().collect::<Vec<_>>();

    let result = problems
        .iter()
        .map(|p| solve(p, &mut memo, &patterns))
        .sum();
    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
