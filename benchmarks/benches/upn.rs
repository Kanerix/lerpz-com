use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;

use lerpz_utils::upn::{generate_upn, generate_upn_with_iteration, replace_char, UserInfo};

fn basic_upn_generation(c: &mut Criterion) {
    let user_info = UserInfo::new(
        "Kasper",
        vec!["Jønsson".into()],
        "Engineering",
        chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
        "lerpz.com",
    );

    c.bench_function("basic_upn", |b| {
        b.iter(|| generate_upn(black_box(user_info.clone())))
    });
}

fn upn_with_multiple_surnames(c: &mut Criterion) {
    let user_info = UserInfo::new(
        "Kasper",
        vec!["Sørensen".into(), "Tørkilsen".into(), "Jønsson".into()],
        "Engineering",
        chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
        "lerpz.com",
    );

    c.bench_function("multiple_surnames", |b| {
        b.iter(|| generate_upn(black_box(user_info.clone())))
    });
}

fn upn_iterations(c: &mut Criterion) {
    let user_info = UserInfo::new(
        "Kasper",
        vec!["Sørensen".into(), "Tørkilsen".into(), "Jønsson".into()],
        "Engineering",
        chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
        "lerpz.com",
    );

    let mut group = c.benchmark_group("iterations");
    for i in 0..6 {
        group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
            b.iter(|| generate_upn_with_iteration(black_box(user_info.clone()), black_box(i)))
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
    let test_cases: Vec<(Arc<str>, Vec<Arc<str>>, Arc<str>)> = vec![
        (Arc::from("Bo"), vec![Arc::from("Li")], Arc::from("IT")),
        (Arc::from("John"), vec![Arc::from("Doe")], Arc::from("Sales")),
        (Arc::from("Katherine"), vec![Arc::from("Smith")], Arc::from("Engineering")),
        (Arc::from("Bartholomew"), vec![Arc::from("Montgomery")], Arc::from("Management")),
    ];

    let mut group = c.benchmark_group("name_lengths");
    for (forename, surnames, dept) in test_cases {
        let label = format!("{}-{}", forename, surnames[0].as_ref());
        let user_info = UserInfo::new(
            forename,
            surnames.clone(),
            dept,
            chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
            "example.com",
        );

        group.bench_with_input(
            BenchmarkId::new("generate", &label),
            &user_info,
            |b, info| b.iter(|| generate_upn(black_box(info.clone()))),
        );
    }
    group.finish();
}

fn special_characters(c: &mut Criterion) {
    let user_info = UserInfo::new(
        "Søren",
        vec!["Åström".into()],
        "Ødegård",
        chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
        "lerpz.com",
    );

    c.bench_function("special_chars", |b| {
        b.iter(|| generate_upn(black_box(user_info.clone())))
    });
}

fn clone_cost(c: &mut Criterion) {
    let user_info = UserInfo::new(
        "Kasper",
        vec!["Jønsson".into()],
        "Engineering",
        chrono::NaiveDate::from_ymd_opt(2020, 10, 15).unwrap(),
        "lerpz.com",
    );

    c.bench_function("userinfo_clone", |b| {
        b.iter(|| black_box(user_info.clone()))
    });
}

criterion_group!(
    benches,
    basic_upn_generation,
    upn_with_multiple_surnames,
    upn_iterations,
    character_replacement,
    varying_name_lengths,
    special_characters,
    clone_cost,
);

criterion_main!(benches);
