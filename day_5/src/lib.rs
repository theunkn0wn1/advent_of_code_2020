use crate::identities::{RowIdentity, SeatIdentity};
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use crate::{parse_col, parse_row};

    #[test]
    fn test_parse_row() {
        assert_eq!(parse_row("FBFBBFF"), 44);
        assert_eq!(parse_row("BFFFBBF"), 70);
        assert_eq!(parse_row("FFFBBBF"), 14);
        assert_eq!(parse_row("BBFFBBF"), 102);
    }

    #[test]
    fn test_parse_col() {
        assert_eq!(parse_col("RRR"), 7);
        assert_eq!(parse_col("RLL"), 4);
        assert_eq!(parse_col("RLR"), 5);
    }
}

const TOTAL_ROWS: u32 = 127;
const TOTAL_COLS: u32 = 7;

mod identities;
mod seat;

fn parse_row(line: &str) -> u32 {
    let mut range_minimum: u32 = 0;
    let mut range_maximum = TOTAL_ROWS;

    let seat_iter = line.chars().map(|x| identities::RowIdentity::new(x));
    for element in seat_iter {
        let remaining_range = range_maximum - range_minimum;
        // print!("{:?}\t", element);
        match element {
            RowIdentity::Front => {
                range_maximum = range_minimum + (remaining_range) / 2;
            }
            RowIdentity::Back => {
                range_minimum = range_minimum + (remaining_range) / 2;
            }
        }
        // println!("range_minimum := {}\trange_maximum := {}", range_minimum, range_maximum);
    };


    range_maximum
}

fn parse_col(line: &str) -> u32 {
    let mut range_minimum: u32 = 0;
    let mut range_maximum = TOTAL_COLS;

    let seat_iter = line.chars().map(|x| identities::SeatIdentity::new(x));
    for element in seat_iter {
        let remaining_range = range_maximum - range_minimum;
        // print!("{:?}\t", element);
        match element {
            SeatIdentity::Left => {
                range_maximum = range_minimum + (remaining_range) / 2;
            }
            SeatIdentity::Right => {
                range_minimum = range_minimum + (remaining_range) / 2;
            }
        }
        // println!("range_minimum := {}\trange_maximum := {}", range_minimum, range_maximum);
    };


    range_maximum
}

pub fn solve_p1(data: &Vec<String>) -> Option<u32> {
    let result = data.iter()
        .map(|row| seat::Seat::new(&row))
        .map(|ticket| ticket.id).max().expect("why is this none?");
    Some(result)
}

pub fn solve_p2(data: &Vec<String>) -> Option<u32> {
    let ticket_ids = data.iter()
        .map(|row| seat::Seat::new(&row))
        .map(|ticket| ticket.id);
    // collect the ticket IDs into a sorted vector.
    let ids = ticket_ids.sorted().collect_vec();
    #[cfg(debug_assertions)]
    println!("difference := {:?}", ids);

    let mut previous = 0;
    for id in ids {
        let delta = id - previous;
        if delta == 2 {
            return Some(previous + 1);
        }
        previous = id;
    }

    None
}