#[cfg(test)]
mod tests {
    use day_1::read_lines;
    use crate::solve_p1;

    #[test]
    fn test_part_one() {
        let line_reader = read_lines("input.txt").unwrap();
        let lines = line_reader.collect::<Result<Vec<_>, _>>().unwrap();
        println!("length of input rows: {:?}", lines.len());
        let solution1 = solve_p1(lines.clone(), 3, 1).unwrap();


        assert_eq!(solution1, 167)
    }
}

use anyhow;


pub fn solve_p1(maze: Vec<String>, dx: usize, dy: usize) -> anyhow::Result<usize> {
    let mut x = 0 + dx;
    let mut y = 0 + dy;
    let mut collisions: usize = 0;

    let mut row_iter = maze.iter();

    // The first row needs to be done manually, to prevent off-by-one errors that
    // produce off by a lot errors down the line

    // println!("length of row_iter := {:?}", row_iter.len());
    let row = row_iter.nth(dy).unwrap();
    // println!("accessing [{:?}][{:?}]", y, x % row.len());
    if row.chars().nth(x % row.len()).unwrap() == '#' {
        collisions += 1;
    }
    // for the rest, simply loop til EOF.
    while let Some(row) = row_iter.nth(dy - 1) {
        x += dx;
        y += dy;
        // println!("accessing [{:?}][{:?}]", y, x % row.len());
        if row.chars().nth(x % row.len()).unwrap() == '#' {
            collisions += 1;
        }
    }

    Ok(collisions)
}