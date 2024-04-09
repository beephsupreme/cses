/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

 use criterion::{black_box, Criterion, criterion_group, criterion_main};

 use missing_number::missing_number;
 
 pub fn missing_number_bench(c: &mut Criterion) {
     c.bench_function("missing_number(5, vec![1, 2, 4, 5])", |b| {
         b.iter(|| missing_number(black_box(5), vec![1, 2, 4, 5]).unwrap())
     });
 }
 
 criterion_group!(benches, missing_number_bench);
 criterion_main!(benches);