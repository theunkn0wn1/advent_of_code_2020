use crate::identities::{SeatIdentity, RowIdentity};
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use crate::parse_row;

    #[test]
    fn test_parse_row() {
        let result = parse_row("FBFBBFF");
        assert_eq!(result, 44);
    }
}

const TOTAL_ROWS: u32 = 127;


mod identities;

fn parse_row(line: &str) -> u32 {
    let mut range_minimum: u32 = 0;
    let mut range_maximum = TOTAL_ROWS;

    let seat_iter = line.chars().map(|x| identities::RowIdentity::new(x));
    for element in seat_iter {
        let remaining_range = range_maximum - range_minimum;
        print!("{:?}\t", element);
            match element {
            RowIdentity::Front => {
                range_maximum = range_minimum + (remaining_range) / 2;
            }
            RowIdentity::Back => {
                range_minimum = range_minimum + (remaining_range) / 2;
            }
        }
        println!("range_minimum := {}\trange_maximum := {}", range_minimum, range_maximum);
    };


    range_maximum
}

fn solve_p1() -> anyhow::Result<()> {
    // let data_iter = day_1::read_lines("input.txt")?
    //     .collect::<Result<Vec<_>, _>>()?;

    let seat = parse_row("FBFBBFF");

    Ok(())
}