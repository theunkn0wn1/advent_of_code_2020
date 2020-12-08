use day_4::{solve_p1, solve_p2, read_entries};
use anyhow::Result;

fn main() -> Result<()> {
    let entries = read_entries("input.txt")?;

    let part_1_solution = solve_p1(&entries)?;
    let part_2_solution = solve_p2(&entries)?;
    println!("part1:= {}", part_1_solution);
    println!("part2:= {}", part_2_solution);

    Ok(())
}