use benchmart_rust::{sort_algo_1, sort_algo_2};

use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers: Vec<i32> = vec![
        1, 2, 3, 6, 5, 4, 8, 52, 2, 1, 5, 4, 4, 5, 8, 54, 2, 0, 55, 5, 2, 0, 5, 5, 5, 21,
    ];

    // This creates a benchmark
    c.bench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_1(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);