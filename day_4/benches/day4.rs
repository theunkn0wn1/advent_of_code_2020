use criterion::{criterion_group, criterion_main};
use day_4::{solve_p1, solve_p2, read_entries};

fn criterion_benchmark(c: &mut criterion::Criterion) {
    let data = read_entries("input.txt").expect("IO failure.");
    c.bench_function("day4 part 1 input.txt", |b| {
        b.iter(|| solve_p1(&data).expect("test fail."))
    });
    c.bench_function("day4 part 2 input.txt", |b| b.iter(|| solve_p2(&data).expect("test fail.")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
