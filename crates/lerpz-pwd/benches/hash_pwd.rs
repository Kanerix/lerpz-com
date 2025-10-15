use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use lerpz_pwd::hash_pwd;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hash_pwd", |b| {
        b.iter(|| hash_pwd(black_box("#Password123!"), black_box("some_salt")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
