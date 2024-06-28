use criterion::{criterion_group, criterion_main, Criterion};
use rustibia::client::battle_list::BattleList;
use rustibia::utils::image::load_image;

fn benchmark_get_creatures_count_when_no_creatures(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/no_creatures.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function("get_creatures_count_when_no_creatures", |b| {
        b.iter(|| battle_list.get_creatures_count())
    });
}

fn benchmark_get_creatures_count_when_full_of_creatures(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/full_of_creatures.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function("get_creatures_count_when_full_of_creatures", |b| {
        b.iter(|| battle_list.get_creatures_count())
    });
}

fn benchmark_get_max_creatures_count(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/full_of_creatures.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function("get_max_creatures_count", |b| {
        b.iter(|| battle_list.get_max_creatures_count())
    });
}

fn benchmark_has_creature_in_target_when_there_are_no_creatures(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/no_creatures.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function("has_creature_in_target_when_there_are_no_creatures", |b| {
        b.iter(|| battle_list.has_creature_in_target())
    });
}

fn benchmark_has_creature_in_target_when_has_no_creature_in_target(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/battle_list/full_of_creatures.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function(
        "has_creature_in_target_when_has_no_creature_in_target",
        |b| b.iter(|| battle_list.has_creature_in_target()),
    );
}

fn benchmark_has_creature_in_target_when_has_creature_target_in_first_position(c: &mut Criterion) {
    let content =
        load_image("./assets/examples/client/battle_list/target_in_first_position.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function(
        "has_creature_in_target_when_has_creature_target_in_first_position",
        |b| b.iter(|| battle_list.has_creature_in_target()),
    );
}

fn benchmark_has_creature_in_target_when_has_creature_target_in_last_position(c: &mut Criterion) {
    let content =
        load_image("./assets/examples/client/battle_list/target_in_first_position.png").unwrap();
    let battle_list = BattleList { content };

    c.bench_function(
        "has_creature_in_target_when_has_creature_target_in_last_position",
        |b| b.iter(|| battle_list.has_creature_in_target()),
    );
}

criterion_group!(
    benches,
    benchmark_get_creatures_count_when_no_creatures,
    benchmark_get_creatures_count_when_full_of_creatures,
    benchmark_get_max_creatures_count,
    benchmark_has_creature_in_target_when_there_are_no_creatures,
    benchmark_has_creature_in_target_when_has_no_creature_in_target,
    benchmark_has_creature_in_target_when_has_creature_target_in_first_position,
    benchmark_has_creature_in_target_when_has_creature_target_in_last_position
);
criterion_main!(benches);
