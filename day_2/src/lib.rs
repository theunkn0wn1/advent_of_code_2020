use day_1::read_lines;
// convenient line reader abstraction
use nom::{IResult, character};

#[derive(Debug)]
struct Constraint {
    pub subject: char,
    pub minimum: u8,
    pub maximum: u8,
    pub password_body: Vec<char>
}

fn read_digit(input: &str) -> IResult<&str, u8> {
    let (remainder, token) = character::complete::digit0(input)?;
    println!("{:?}", remainder);

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

fn parse_line(input: &str) -> IResult<&str, Constraint> {
    let result = nom::combinator::all_consuming(nom::sequence::tuple((
        read_constraint,
        nom::character::complete::space1,
        read_spec_char,
        nom::character::complete::char(':'),
        nom::multi::many0(nom::character::complete::anychar)
    )))(input)?;

    let (remainder, (constraint_range, _, constraint_char, _, remainder_text)) = result;

    Ok((remainder, Constraint {
        subject: constraint_char,
        minimum: constraint_range[0],
        maximum: constraint_range[1],
        password_body: remainder_text
    }))
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
        let (remainder, result) = read_constraint(raw).expect("fail");
        println!("{:?}", result);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 3);
    }

    #[test]
    fn test_parse_line() {
        let raw = "1-3 b: cdefg";
        let (remainder, result) = parse_line(raw).expect("fail");
        println!("{:?}", result);
    }
}
