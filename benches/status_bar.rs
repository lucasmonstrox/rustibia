use criterion::{criterion_group, criterion_main, Criterion};
use rustibia::client::status_bar::StatusBar;
use rustibia::utils::image::load_image;

fn benchmark_get_hp_percentage_100(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/100_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_100", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

fn benchmark_get_hp_percentage_90(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/90_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_90", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

fn benchmark_get_hp_percentage_80(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/80_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_80", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

fn benchmark_get_hp_percentage_70(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/70_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_70", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

fn benchmark_get_hp_percentage_60(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/60_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_60", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

fn benchmark_get_hp_percentage_50(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/50_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_50", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

fn benchmark_get_hp_percentage_40(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/40_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_40", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

fn benchmark_get_hp_percentage_30(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/30_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_30", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

fn benchmark_get_hp_percentage_20(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/20_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_20", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

fn benchmark_get_hp_percentage_10(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/10_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_10", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

fn benchmark_get_hp_percentage_1(c: &mut Criterion) {
    let content = load_image("./assets/examples/client/status_bar/1_percent.png").unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage_1", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

criterion_group!(
    benches,
    benchmark_get_hp_percentage_100,
    benchmark_get_hp_percentage_90,
    benchmark_get_hp_percentage_80,
    benchmark_get_hp_percentage_70,
    benchmark_get_hp_percentage_60,
    benchmark_get_hp_percentage_50,
    benchmark_get_hp_percentage_40,
    benchmark_get_hp_percentage_30,
    benchmark_get_hp_percentage_20,
    benchmark_get_hp_percentage_10,
    benchmark_get_hp_percentage_1,
);
criterion_main!(benches);
