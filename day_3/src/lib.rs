#[cfg(test)]
mod tests {
    use crate::solve_p1;
    use day_1::read_lines;

    #[test]
    fn test_part_one() {
        let line_reader = read_lines("input.txt").unwrap();
        let lines = line_reader.collect::<Result<Vec<_>, _>>().unwrap();
        println!("length of input rows: {:?}", lines.len());
        let solution1 = solve_p1(&lines, 3, 1);

        assert_eq!(solution1, 167)
    }

    #[test]
    fn test_part_one_b() {
        let line_reader = read_lines("sample.txt").unwrap();
        let lines = line_reader.collect::<Result<Vec<_>, _>>().unwrap();
        println!("length of input rows: {:?}", lines.len());
        let solution1 = solve_p1(&lines, 1, 2);

        assert_eq!(solution1, 2)
    }
}

use anyhow;

pub fn solve_p1(maze: &Vec<String>, dx: usize, dy: usize) -> usize {
    let mut x = 0 + dx;
    // let mut y = 0 + dy;
    let mut collisions: usize = 0;

    let mut row_iter = maze.iter().step_by(dy);

    // The first row needs to be done manually, to prevent off-by-one errors that
    // produce off by a lot errors down the line

    // println!("length of row_iter := {:?}", row_iter.len());
    row_iter.next();
    let row = row_iter.next().unwrap();

    // println!("accessing [{:?}][{:?}]", y, x % row.len());
    if row.chars().nth(x % row.len()).unwrap() == '#' {
        collisions += 1;
    }
    // for the rest, simply loop til EOF.
    while let Some(row) = row_iter.next() {
        x += dx;
        // println!("accessing [{:?}][{:?}]", y, x % row.len());
        if row.chars().nth(x % row.len()).unwrap() == '#' {
            collisions += 1;
        }
    }
    println!("collisions := {:?}", collisions);
    collisions
}

pub fn solve_p2(maze: &Vec<String>) -> usize {
    let curves: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    curves
        .into_iter()
        .map(|(x, y)| crate::solve_p1(maze, x, y))
        .product()
}
