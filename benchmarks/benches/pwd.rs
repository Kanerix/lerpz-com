use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

use lerpz_pwd::{hash_pwd, validate_pwd};

fn bench_hash_pwd(c: &mut Criterion) {
    c.bench_function("hash_pwd", |b| {
        b.iter(|| hash_pwd(black_box("#Password123!"), black_box("some_salt")))
    });
}

fn bench_validate_pwd(c: &mut Criterion) {
    c.bench_function("validate_pwd", |b| {
        b.iter(|| {
            validate_pwd(
                black_box("#Password123!"),
                black_box("some_salt"),
                black_box("#01#$argon2id$v=19$m=19456,t=2,p=1$c29tZV9zYWx0$ghjsFNe2ss8a58awwK3hDF3pxQW85H5ko9flPA41JSU")
            )
        })
    });
}

criterion_group!(benches, bench_hash_pwd, bench_validate_pwd);
criterion_main!(benches);
