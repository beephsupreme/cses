use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cses::number_spiral::number_spiral;

pub fn number_spiral_bench(c: &mut Criterion) {
    c.bench_function("number_spiral(3, vec![(2, 3), (1, 1), (4, 2)])", |b| {
        b.iter(|| number_spiral(black_box(3), vec![(2, 3), (1, 1), (4, 2)]))
    });
}
criterion_group!(benches, number_spiral_bench);
criterion_main!(benches);
