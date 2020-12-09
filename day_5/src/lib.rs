use crate::identities::{RowIdentity, SeatIdentity};
use std::collections::{HashSet};
use once_cell::sync::Lazy;
use itertools::Itertools;
use std::iter::FromIterator;
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
static SEATS: Lazy<HashSet<u32>> = Lazy::new(|| {
    let mut seats = HashSet::new();
    seats.extend((0..971));
    seats
});

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

pub fn solve_p1(data: &Vec<String>) -> anyhow::Result<()> {
    let result = data.iter()
        .map(|row| seat::Seat::new(&row))
        .map(|ticket| ticket.id).max().expect("why is this none?");
    println!("result := {:?}", result);
    Ok(())
}

pub fn solve_p2(data: &Vec<String>) -> anyhow::Result<u32> {
    let ticket_ids = data.iter()
        .map(|row| seat::Seat::new(&row))
        .map(|ticket| ticket.id);

    // compute this up front so we can use it for assertions
    let ticket_ids_len = ticket_ids.len();

    #[cfg(debug_assertions)]
    println!("ticket_ids.len() := {:?}", ticket_ids_len);

    // instantiate a new hashset
    let mut tickets_set: HashSet<u32> = HashSet::from_iter(ticket_ids);

    debug_assert!(tickets_set.len() == ticket_ids_len);


    let mut intersection = SEATS.difference(&tickets_set).collect_vec();
    intersection.sort();
    #[cfg(debug_assertions)]
    println!("difference := {:?}", intersection);

    let mut previous = 0;
    for id in intersection {
        let delta = id - previous;
        if delta != 1 {
            println!("found delta of {} := {}", delta, id)
        }
        previous = *id;
    }

    Ok(0)
}