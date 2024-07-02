use criterion::{criterion_group, criterion_main, Criterion};
use rustibia::client::action_bars::Action;
use rustibia::utils::image::load_image;

fn benchmark_action_when_is_unequipped(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/action_bars/action/empty.png").unwrap();
    let action = Action { content };
    c.bench_function("is_unequipped", |b| b.iter(|| action.is_equipped()));
}

fn benchmark_action_when_is_equipped(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/action_bars/action/equipped.png").unwrap();
    let action = Action { content };
    c.bench_function("is_equipped", |b| b.iter(|| action.is_equipped()));
}

criterion_group!(
    benches,
    benchmark_action_when_is_unequipped,
    benchmark_action_when_is_equipped
);
criterion_main!(benches);
