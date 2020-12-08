// convenient line reader abstraction
use nom::{IResult, character};
use std::collections::HashMap;
#[derive(Debug)]
pub struct Constraint {
    pub subject: char,
    pub minimum: u8,
    pub maximum: u8,
    pub password_body: Vec<char>
}

fn read_digit(input: &str) -> IResult<&str, u8> {
    let (remainder, token) = character::complete::digit0(input)?;

    Ok((remainder, token.parse().unwrap()))
}

fn read_constraint(input: &str) -> IResult<&str, Vec<u8>> {
    nom::multi::separated_list0(
        nom::character::complete::char('-'),
        read_digit,
    )(input)
}

fn read_spec_char(input: &str) -> IResult<&str, char> {
    nom::character::complete::anychar(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Constraint> {
    let result = nom::combinator::all_consuming(nom::sequence::tuple((
        read_constraint,
        nom::character::complete::space1,
        read_spec_char,
        nom::character::complete::char(':'),
        nom::character::complete::space1,
        nom::multi::many1(nom::character::complete::anychar)
    )))(input)?;

    let (remainder, (constraint_range, _, constraint_char, _, _, remainder_text)) = result;

    Ok((remainder, Constraint {
        subject: constraint_char,
        minimum: constraint_range[0],
        maximum: constraint_range[1],
        password_body: remainder_text
    }))
}
pub fn solve_p1(lines: Vec<String>) -> anyhow::Result<u32> {

    let mut total_passing: u32 = 0;

    for line in lines.iter() {
        match parse_line(&line) {
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

pub fn solve_p2(lines: Vec<String>) -> anyhow::Result<u32> {

    let mut total_passing: u32 = 0;

    for line in lines.iter() {
        match parse_line(&line) {
            Ok((_remainder, constraint)) => {
                let char_maximum: char = constraint.password_body[(constraint.maximum-1) as usize];
                let char_minimum: char = constraint.password_body[(constraint.minimum-1) as usize];
                if (char_maximum == constraint.subject.into()) ^ (char_minimum == constraint.subject.into()){
                    total_passing += 1;
                }

            }
            Err(e) => { anyhow::bail!("{}", e) }
        }
    }

    Ok(total_passing)
}
// fn read_constraint(input: &str) -> IResult<&str, Constraint> {
// }
#[cfg(test)]
mod tests {
    use crate::{read_digit, read_constraint, parse_line};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_read_digit() {
        let raw = "2-3";
        let result = read_digit(raw).expect("fail");

        println!("{:?}", result);
    }

    #[test]
    fn test_read_constraint() {
        let raw = "1-3 b: cdefg";
        let (_, result) = read_constraint(raw).expect("fail");
        println!("{:?}", result);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 3);
    }

    #[test]
    fn test_parse_line() {
        let raw = "1-3 b: cdefg";
        let (_, result) = parse_line(raw).expect("fail");
        println!("{:?}", result);

        assert_eq!(result.maximum, 3);
        assert_eq!(result.minimum,1);
        assert_eq!(result.subject, 'b');
    }
}
