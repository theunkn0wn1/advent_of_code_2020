use day_1::{do_read, numbers2, numbers3};
use criterion::{criterion_group, criterion_main};

fn criterion_benchmark(c: &mut criterion::Criterion){
    let input = do_read().expect("failed to read test data");
    c.bench_function("numbers2 input.txt", |b| b.iter(|| numbers2(input.clone())));
    c.bench_function("numbers3 input.txt", |b| b.iter(|| numbers3(input.clone())));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);