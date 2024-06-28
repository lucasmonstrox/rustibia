use criterion::{criterion_group, criterion_main, Criterion};
use rustibia::client::battle_list::BattleList;
use rustibia::utils::image::load_image;

fn benchmark_get_creatures_count_empty(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/empty_battle_list.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function("get_creatures_count_empty", |b| {
        b.iter(|| battle_list.get_creatures_count())
    });
}

fn benchmark_get_creatures_count_full(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/full_battle_list.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function("get_creatures_count_full", |b| {
        b.iter(|| battle_list.get_creatures_count())
    });
}

fn benchmark_get_max_creatures_count(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/full_battle_list.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function("get_max_creatures_count", |b| {
        b.iter(|| battle_list.get_max_creatures_count())
    });
}

fn benchmark_has_creature_in_target_empty(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/empty_battle_list.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function("has_creature_in_target_first", |b| {
        b.iter(|| battle_list.has_creature_in_target())
    });
}

fn benchmark_has_creature_in_target_first(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/target_first.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function("has_creature_in_target_first", |b| {
        b.iter(|| battle_list.has_creature_in_target())
    });
}

fn benchmark_has_creature_in_target_last(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/target_last.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function("has_creature_in_target_last", |b| {
        b.iter(|| battle_list.has_creature_in_target())
    });
}

criterion_group!(
    benches,
    benchmark_get_creatures_count_empty,
    benchmark_get_creatures_count_full,
    benchmark_get_max_creatures_count,
    benchmark_has_creature_in_target_empty,
    benchmark_has_creature_in_target_first,
    benchmark_has_creature_in_target_last
);
criterion_main!(benches);
