use criterion::{criterion_group, criterion_main};
use day_1::read_lines;
use day_2::{solve_p1, solve_p2};
fn criterion_benchmark(c: &mut criterion::Criterion){
    let line_reader = read_lines("input.txt").expect("failed to read input.");
    let lines = line_reader.collect::<Result<Vec<_>, _>>().expect("failed to cast input to vec.");

    c.bench_function("day2 part 1 input.txt", |b| b.iter(|| solve_p1(lines.clone())));
    c.bench_function("day2 part 2 lines.txt", |b| b.iter(|| solve_p2(lines.clone())));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);