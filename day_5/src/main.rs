use day_5::{solve_p1, solve_p2};

fn main() -> anyhow::Result<()> {
    let data = day_1::read_lines("input.txt")?
        .collect::<Result<Vec<_>, _>>()?;

    solve_p1(&data)?;
    if let Some(solution) = solve_p2(&data) {
        println!("Part 2 := {}", solution);
    } else {
        println!("Part 2 : No solution.")
    }
    Ok(())
}