use std::collections::HashMap;

use anyhow;

use day_1::read_lines;
use day_2;

fn main() -> anyhow::Result<()> {
    let solution = solve()?;
    println!("total passing {:?}", solution);
    Ok(())
}

fn solve() -> anyhow::Result<u32> {
    let lines = read_lines("sample.txt")?;
    let lines = lines.collect::<Result<Vec<_>, _>>()?;
    let mut total_passing: u32 = 0;

    for line in lines.iter() {
        match day_2::parse_line(&line) {
            Ok((_remainder, constraint)) => {
                let mut charmap: HashMap<char, u8> = HashMap::new();
                for letter in constraint.password_body {
                    let hash_v = charmap.entry(letter).or_insert(0);
                    *hash_v += 1;
                }

                if let Some(count) = charmap.get(&constraint.subject) {
                    if count >= &constraint.minimum && count <= &constraint.maximum {
                        total_passing += 1;
                    }
                }
            }
            Err(e) => { anyhow::bail!("{}", e) }
        }
    }

    Ok(total_passing)
}