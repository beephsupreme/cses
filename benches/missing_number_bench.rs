use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cses::missing_number::missing_number;

pub fn missing_number_bench(c: &mut Criterion) {
    c.bench_function("missing_number(5, vec![1, 2, 4, 5])", |b| {
        b.iter(|| missing_number(black_box(5), vec![1, 2, 4, 5]))
    });
}

criterion_group!(benches, missing_number_bench);
criterion_main!(benches);
