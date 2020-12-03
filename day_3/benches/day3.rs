use criterion::{criterion_group, criterion_main};
use day_1::read_lines;
use day_3::{solve_p1, solve_p2};

fn criterion_benchmark(c: &mut criterion::Criterion) {
    let line_reader = read_lines("input.txt").expect("failed to read input.");
    let lines = line_reader
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to cast input to vec.");

    c.bench_function("day3 part 1 input.txt", |b| {
        b.iter(|| solve_p1(&lines, 3, 1))
    });
    c.bench_function("day3 part 2 lines.txt", |b| b.iter(|| solve_p2(&lines)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
