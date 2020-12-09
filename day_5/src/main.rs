use day_5::{solve_p1, solve_p2};

fn main() -> anyhow::Result<()> {
    let data = day_1::read_lines("input.txt")?
        .collect::<Result<Vec<_>, _>>()?;

    solve_p1(&data)?;
    solve_p2(&data)?;
    Ok(())
}