use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cses::increasing_array::increasing_array;

pub fn increasing_array_bench(c: &mut Criterion) {
    c.bench_function("increasing_array(vec![3, 2, 5, 1, 7])", |b| {
        b.iter(|| increasing_array(black_box(vec![3, 2, 5, 1, 7])))
    });
}

criterion_group!(benches, increasing_array_bench);
criterion_main!(benches);
