use day_6::part_1::solve_p1;
use day_6::part_2::solve_p2;
fn main() {
    let lines = day_1::read_lines("input.txt")
        .expect("failed to read lines")
        .collect::<Result<Vec<_>, _>>()
        .expect("bad read.");

    let part_1_solution = solve_p1(&lines);
    println!("part 1 := {}", part_1_solution);
    let part_2_solution = solve_p2(&lines);
    println!("part 2 := {}", part_2_solution);
}