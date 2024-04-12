use criterion::{black_box, Criterion, criterion_group, criterion_main};

use cses::repetitions::repetitions;

pub fn repetitions_bench(c: &mut Criterion) {
    c.bench_function("repetitions(AAABBGGGGTT)", |b| {
        b.iter(|| repetitions(black_box("AAABBGGGGTT".to_string())))
    });
}

criterion_group!(benches, repetitions_bench);
criterion_main!(benches);
