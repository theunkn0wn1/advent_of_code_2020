use anyhow;

use day_1::read_lines;
use day_3;

fn main() -> anyhow::Result<()> {
    let line_reader = read_lines("input.txt")?;
    let lines = line_reader.collect::<Result<Vec<_>, _>>()?;

    let solution1 = day_3::solve_p1(lines.clone())?;
    // let solution2 = day_3::solve_p2(lines.clone())?;
    println!("total passing p1 := {:?}", solution1);
    // println!("total passing p2 := {:?}", solution2);
    Ok(())
}
