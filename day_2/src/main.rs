use std::collections::HashMap;

use anyhow;

use day_1::read_lines;
use day_2;

fn main() -> anyhow::Result<()> {
    let line_reader = read_lines("input.txt")?;
    let lines = line_reader.collect::<Result<Vec<_>, _>>()?;

    let solution = day_2::solve_p2(lines)?;
    println!("total passing {:?}", solution);
    Ok(())
}

