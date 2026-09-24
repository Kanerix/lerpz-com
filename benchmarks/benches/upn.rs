use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use lerpz_utils::upn::{
    UserInfo, generate_upn, generate_upn_into, generate_upn_with_iteration, is_valid_domain,
    replace_char,
};

fn basic_upn_generation(c: &mut Criterion) {
    let user_info = UserInfo::new("Kasper", vec!["Jønsson"], 2020, "lerpz.com");

    c.bench_function("basic_upn", |b| {
        b.iter(|| generate_upn(black_box(&user_info)))
    });
}

fn upn_with_multiple_surnames(c: &mut Criterion) {
    let user_info = UserInfo::new(
        "Kasper",
        vec!["Sørensen", "Tørkilsen", "Jønsson"],
        2020,
        "lerpz.com",
    );

    c.bench_function("multiple_surnames", |b| {
        b.iter(|| generate_upn(black_box(&user_info)))
    });
}

fn upn_iterations(c: &mut Criterion) {
    let user_info = UserInfo::new(
        "Kasper",
        vec!["Sørensen", "Tørkilsen", "Jønsson"],
        2020,
        "lerpz.com",
    );

    let mut group = c.benchmark_group("iterations");
    for i in 0..6 {
        group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
            b.iter(|| generate_upn_with_iteration(black_box(&user_info), black_box(i)))
        });
    }
    group.finish();
}

fn character_replacement(c: &mut Criterion) {
    let chars = vec!['a', 'å', 'ø', 'æ', 'z', '1', '@', ' '];

    c.bench_function("replace_char", |b| {
        b.iter(|| {
            for &ch in &chars {
                replace_char(black_box(ch));
            }
        })
    });
}

fn varying_name_lengths(c: &mut Criterion) {
    let test_cases: Vec<(&str, Vec<&str>)> = vec![
        ("Bo", vec!["Li"]),
        ("John", vec!["Doe"]),
        ("Katherine", vec!["Smith"]),
        ("Bartholomew", vec!["Montgomery"]),
    ];

    let mut group = c.benchmark_group("name_lengths");
    for (forename, surnames) in test_cases {
        let label = format!("{}-{}", forename, surnames[0]);
        let user_info = UserInfo::new(forename, surnames.clone(), 2020, "example.com");

        group.bench_with_input(
            BenchmarkId::new("generate", &label),
            &user_info,
            |b, info| b.iter(|| generate_upn(black_box(info))),
        );
    }
    group.finish();
}

fn special_characters(c: &mut Criterion) {
    let user_info = UserInfo::new("Søren", vec!["Åström"], 2020, "lerpz.com");

    c.bench_function("special_chars", |b| {
        b.iter(|| generate_upn(black_box(&user_info)))
    });
}

fn domain_validation(c: &mut Criterion) {
    let domains = ["lerpz.com", "sub.lerpz.co.uk", "not a @ domain", ""];

    c.bench_function("is_valid_domain", |b| {
        b.iter(|| {
            for domain in domains {
                is_valid_domain(black_box(domain));
            }
        })
    });
}

fn buffer_reuse(c: &mut Criterion) {
    let user_info = UserInfo::new("Kasper", vec!["Jønsson"], 2020, "lerpz.com");

    let mut group = c.benchmark_group("single_upn");
    group.bench_function("allocating", |b| {
        b.iter(|| generate_upn(black_box(&user_info)))
    });
    group.bench_function("reused_buffer", |b| {
        let mut buf = String::with_capacity(32);
        b.iter(|| {
            buf.clear();
            generate_upn_into(black_box(&user_info), 0, &mut buf)
        })
    });
    group.finish();
}

fn clone_cost(c: &mut Criterion) {
    let user_info = UserInfo::new("Kasper", vec!["Jønsson"], 2020, "lerpz.com");

    let mut group = c.benchmark_group("userinfo");
    group.bench_function("clone", |b| b.iter(|| black_box(user_info.clone())));
    group.bench_function("into_owned", |b| {
        b.iter(|| black_box(user_info.clone()).into_owned())
    });
    group.finish();
}

criterion_group!(
    benches,
    basic_upn_generation,
    upn_with_multiple_surnames,
    upn_iterations,
    character_replacement,
    varying_name_lengths,
    special_characters,
    domain_validation,
    buffer_reuse,
    clone_cost,
);

criterion_main!(benches);
