use day_4::{solve_p1, solve_p2};
use anyhow::Result;

fn main() -> Result<()> {
    let part_1_solution = solve_p1("input.txt")?;
    let part_2_solution = solve_p2("input.txt")?;
    println!("part1:= {}", part_1_solution);
    println!("part2:= {}", part_2_solution);

    Ok(())
}