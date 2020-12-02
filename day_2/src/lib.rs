use day_1::read_lines; // convenient line reader abstraction
use nom::{IResult,sequence::tuple, character};
struct Constraint {
    pub subject: char,
    pub minimum: u8,
    pub maximum: u8,
}

fn read_digit(input: &str) -> IResult<&str, u8> {
    let (remainder, token) = character::complete::digit0(input)?;
    println!("{:?}", remainder);

     Ok((remainder, token.parse().unwrap()))

}

fn read_constraint(input: &str) -> IResult<&str, Vec<u8>> {
    nom::multi::separated_list0(
        nom::character::complete::char('-'),
        read_digit
    )(input)

}

fn read_spec_char(input: &str) -> IResult<&str, char> {
    nom::character::complete::anychar(input)
}

// fn read_constraint(input: &str) -> IResult<&str, Constraint> {
// }
#[cfg(test)]
mod tests {
    use crate::{read_digit, read_constraint};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_read_digit(){
        let raw = "2-3";
        let result = read_digit(raw).expect("fail");

        println!("{:?}", result);
    }
    #[test]
    fn test_read_constraint() {
        let raw = "1-3 b: cdefg";
        let (remainder, result) = read_constraint(raw).expect("fail");
        println!("{:?}", result);

    }
}
