use anyhow::Result;

use aoc_utils::*;
use day_2::*;

fn is_safe(row: &[u16]) -> bool {
    if row.len() < 2 {
        return true;
    }

    let ord = row[0].cmp(&row[1]);

    for i in 0..row.len() - 1 {
        let Ok(o) = safe_diff(row[i], row[i + 1]) else {
            return false;
        };
        if o != ord {
            return false;
        }
    }

    true
}

fn walk_to_safe(row: &[u16]) -> bool {
    for i in 0..row.len() {
        let mut row = row.to_vec();
        row.remove(i);
        if is_safe(&row) {
            return true;
        }
    }
    false
}
fn main() -> Result<()> {
    let mut result = 0;
    let mut aoc = Aoc::part2();

    let data = read_data(INPUT)?;

    for row in data {
        if is_safe(&row) || walk_to_safe(&row) {
            result += 1;
        }
    }

    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
