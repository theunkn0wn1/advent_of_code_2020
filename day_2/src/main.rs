use day_2;
use day_1::read_lines;
use anyhow;
use day_2::Constraint;
use std::collections::HashMap;

fn main() -> anyhow::Result<()>{
    let solution = solve()?;
    println!("{:?}", solution);
    Ok(())
}

fn solve() -> anyhow::Result<bool> {
    let lines = read_lines("sample.txt")?;
    let lines = lines.collect::<Result<Vec<_>, _>>()?;
    let mut total_passing:u32 = 0;

    for line in lines.iter() {
        match day_2::parse_line(&line) {
            Ok((_remainder, constraint)) => {
                let mut charmap: HashMap<char, u8> = HashMap::new();
                for letter in constraint.password_body {
                    let hash_v = charmap.entry(letter).or_insert(0);
                    *hash_v += 1;
                }

                if let Some(count) = charmap.get(&constraint.subject) {
                    return Ok(count >= &constraint.minimum && count <= &constraint.maximum);
                } else {
                    return Ok(false);
                }
            }
            Err(e) => { anyhow::bail!("{}", e) }
        }
    }

    Ok(false)
}