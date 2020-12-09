#[derive(Debug)]
pub enum RowIdentity {
    Front,
    Back,
}

impl RowIdentity {
    pub fn new(raw: char) -> Self {
        match raw
        {
            'F' => RowIdentity::Front,
            'B' => RowIdentity::Back,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub enum SeatIdentity {
    Right,
    Left,
}

impl SeatIdentity {
    pub fn new(raw: char) -> Self {
        match raw
        {
            'L' => SeatIdentity::Left,
            'R' => SeatIdentity::Right,
            _ => unreachable!()
        }
    }
}