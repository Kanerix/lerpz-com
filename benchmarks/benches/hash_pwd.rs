use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

use lerpz_pwd::hash_pwd;

fn bench_hash_pwd(c: &mut Criterion) {
    c.bench_function("hash_pwd", |b| {
        b.iter(|| hash_pwd(black_box("#Password123!"), black_box("some_salt")))
    });
}

criterion_group!(benches, bench_hash_pwd);
criterion_main!(benches);
