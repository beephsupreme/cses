use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cses::permutations::permutations;

pub fn permutations_bench(c: &mut Criterion) {
    c.bench_function("permutations(1000000)", |b| {
        b.iter(|| permutations(black_box(1000000)))
    });
}

criterion_group!(benches, permutations_bench);
criterion_main!(benches);
