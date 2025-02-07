use anyhow::Result;

use aoc_utils::*;
use day_17::*;

fn main() -> Result<()> {
    let mut aoc = Aoc::part2();
    let program = [2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 5, 5, 0, 3, 3, 0];
    let mut possible_a_registers = Vec::from([0]);

    for i in 1..=program.len() {
        let (_, expected_output) = program.split_at(program.len() - i);
        let mut next_possible_a_registers = Vec::new();
        for possible_a_register in possible_a_registers {
            for next_part_of_a in 0..=7 {
                let a = (possible_a_register << 3) | next_part_of_a;
                let mut computer = Computer::new([a, 0, 0], &program);
                let output = computer.run();
                if output == expected_output {
                    next_possible_a_registers.push(a);
                }
            }
        }
        possible_a_registers = next_possible_a_registers;
    }
    possible_a_registers.sort();
    aoc.result(*possible_a_registers.first().unwrap() as usize);
    println!("{}", aoc);
    Ok(())
}
