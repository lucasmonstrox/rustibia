use criterion::{criterion_group, criterion_main, Criterion};
use ndarray::ArrayView3;
use rustibia::client::status_bar::StatusBar;

fn benchmark_get_hp_percentage(c: &mut Criterion) {
    let content_vec = image::open("status_bar.png").unwrap().as_bytes().to_vec();
    let content = ArrayView3::from_shape((24, 108, 4), &content_vec).unwrap();
    let status_bar = StatusBar { content };

    c.bench_function("get_hp_percentage", |b| {
        b.iter(|| status_bar.get_hp_percentage())
    });
}

criterion_group!(benches, benchmark_get_hp_percentage,);
criterion_main!(benches);
