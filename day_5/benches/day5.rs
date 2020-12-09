use criterion::{criterion_group, criterion_main};
use day_5::{solve_p1, solve_p2};

fn criterion_benchmark(c: &mut criterion::Criterion) {
    let data = day_1::read_lines("input.txt").unwrap()
        .collect::<Result<Vec<_>, _>>().unwrap();

    c.bench_function("day5 part 1 input.txt", |b| {
        b.iter(|| solve_p1(&data).expect("test fail."))
    });
    c.bench_function("day5 part 2 input.txt", |b| b.iter(|| solve_p2(&data).expect("test fail.")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
