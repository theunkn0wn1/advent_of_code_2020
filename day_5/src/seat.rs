use crate::{parse_row, parse_col};
#[cfg(test)]
mod tests {
    use crate::seat::Seat;

    #[test]
    fn test_seat() {
        let seat = Seat::new("BFFFBBFRRR");
        assert_eq!(seat.row, 70);
        assert_eq!(seat.col, 7);
        assert_eq!(seat.id, 567);

        let seat = Seat::new("FFFBBBFRRR");
        assert_eq!(seat.row, 14);
        assert_eq!(seat.col, 7);
        assert_eq!(seat.id, 119);

        let seat = Seat::new("BBFFBBFRLL");
        assert_eq!(seat.row, 102);
        assert_eq!(seat.col, 4);
        assert_eq!(seat.id, 820);

    }
}
#[derive(Debug)]
pub struct Seat {
    pub row: u32,
    pub col: u32,
    pub id: u32,
}

impl Seat {
    pub fn new(raw: &str) -> Self {
        let (row_part, col_part) = raw.split_at(7);
        let row = parse_row(row_part);
        let col = parse_col(col_part);
        let id = row * 8 + col;

        Self {
            row,
            col,
            id,
        }
    }
}