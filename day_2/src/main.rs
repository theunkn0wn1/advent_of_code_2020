use day_2;
use day_1::read_lines;
use anyhow;

fn main() -> anyhow::Result<()> {
    let lines = read_lines("sample.txt")?;

    for line in lines {
        let actual_line = line?;
        let (_, _constraint) = day_2::parse_line(&actual_line)?;
    }

    Ok(())
}