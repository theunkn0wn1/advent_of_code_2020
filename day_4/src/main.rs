use day_4::solve_p1;
use anyhow::Result;

fn main() -> Result<()> {
    let validated = solve_p1("input.txt")?;
    println!("part1:= {}", validated);

    Ok(())
}