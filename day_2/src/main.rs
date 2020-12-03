use day_2;
use day_1::read_lines;
use anyhow;
use day_2::Constraint;
use nom::error::Error;

fn main() -> anyhow::Result<()> {
    let lines = read_lines("sample.txt")?;
    let lines = lines.collect::<Result<Vec<_>, _>>()?;

    for line in lines.iter() {
        match day_2::parse_line(&line) {
            Ok((remainder, constraint)) => {
                println!("{:?}", constraint)
            }
            Err(e) => { anyhow::bail!("{}", e) }
        }
    }

    Ok(())
}