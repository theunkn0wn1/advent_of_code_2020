use day_6::part_1::solve_p1;

fn main() {
    let lines = day_1::read_lines("input.txt")
        .expect("failed to read lines")
        .collect::<Result<Vec<_>, _>>()
        .expect("bad read.");

    let part_1_solution = day_6::part_1::solve_p1(&lines);
    println!("part 1 := {}", part_1_solution);
}