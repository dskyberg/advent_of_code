use anyhow::Result;

use aoc_utils::*;
use day_3::*;

static DO: &str = "do()";
static DONT: &str = "don't()";

fn main() -> Result<()> {
    let mut result = 0;
    let mut aoc = Aoc::part2();
    let data = read_data(INPUT)?;
    let mut data = data.join("");

    // We need to partition along do() and don't().
    loop {
        // Look for the next don't()
        match data.find(DONT) {
            Some(idx) => {
                // Pull the slice for processing
                let slice = &data[0..(idx + DONT.len())];
                result += process_line(slice);
                // Throw away the slice
                data = data[(idx + DONT.len() + 1)..].to_string();
            }
            None => {
                // Looks like we reached the last slice
                result += process_line(&data);
                break;
            }
        }

        // Throw away until the next do()
        if let Some(idx) = data.find(DO) {
            data = data[idx..].to_string();
        };
    }

    aoc.result(result);
    println!("{}", aoc);
    Ok(())
}
