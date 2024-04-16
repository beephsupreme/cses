use criterion::{black_box, Criterion, criterion_group, criterion_main};

use cses::increasing_array::increasing_array;

pub fn increasing_array_bench(c: &mut Criterion) {
    c.bench_function("missing_number(5, vec![1, 2, 4, 5])", |b| {
        b.iter(|| increasing_array(black_box(vec![3, 2, 5, 1, 7])))
    });
}

criterion_group!(benches, increasing_array_bench);
criterion_main!(benches);
