use criterion::{criterion_group, criterion_main, Criterion};
use ndarray::ArrayView3;
use rustibia::client::battle_list::BattleList;

fn benchmark_get_creatures_count(c: &mut Criterion) {
    let content_vec = image::open("content.png").unwrap().as_bytes().to_vec();
    let content = ArrayView3::from_shape((998, 156, 4), &content_vec).unwrap();
    let battle_list = BattleList { content };

    c.bench_function("get_creatures_count", |b| {
        b.iter(|| battle_list.get_creatures_count())
    });
}

fn benchmark_get_max_creatures_count(c: &mut Criterion) {
    let content_vec = image::open("content.png").unwrap().as_bytes().to_vec();
    let content = ArrayView3::from_shape((998, 156, 4), &content_vec).unwrap();
    let battle_list = BattleList { content };

    c.bench_function("get_max_creatures_count", |b| {
        b.iter(|| battle_list.get_max_creatures_count())
    });
}

fn benchmark_has_creature_in_target(c: &mut Criterion) {
    let content_vec = image::open("content.png").unwrap().as_bytes().to_vec();
    let content = ArrayView3::from_shape((998, 156, 4), &content_vec).unwrap();
    let battle_list = BattleList { content };

    c.bench_function("has_creature_in_target", |b| {
        b.iter(|| battle_list.has_creature_in_target())
    });
}

criterion_group!(
    benches,
    benchmark_get_creatures_count,
    benchmark_get_max_creatures_count,
    benchmark_has_creature_in_target
);
criterion_main!(benches);
