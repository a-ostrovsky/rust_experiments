use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_experiments::quick_select::quick_select;
use rust_experiments::utils::create_random_distinct_vec;

fn benchmark_quick_select(c: &mut Criterion) {
    let mut group = c.benchmark_group("quick_select");
    for size in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(format!("size: {}", size), size, |b, &size| {
            b.iter_batched_ref(
                || create_random_distinct_vec(size),
                |arr| {
                    let _ = quick_select(arr, black_box(size / 2));
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_quick_select);
criterion_main!(benches);
