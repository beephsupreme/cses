/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

 use criterion::{black_box, Criterion, criterion_group, criterion_main};

 use weird_algorithm::weird_algorithm;
 
 pub fn bench(c: &mut Criterion) {
     c.bench_function("weird_algorithm::solve(7)", |b| {
         b.iter(|| weird_algorithm(black_box(7)))
     });
 }
 
 criterion_group!(benches, bench);
 criterion_main!(benches);