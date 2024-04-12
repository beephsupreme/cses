use criterion::{black_box, Criterion, criterion_group, criterion_main};

use cses::weird_algorithm::weird_algorithm;

pub fn weird_algorithm_bench(c: &mut Criterion) {
    c.bench_function("weird_algorithm::solve(1000000)", |b| {
        b.iter(|| weird_algorithm(black_box(1000000)))
    });
}

criterion_group!(benches, weird_algorithm_bench);
criterion_main!(benches);
