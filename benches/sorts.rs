use clrs_study::sorts::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn insertion_sorts_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("insertion sort");

    // TODO: somehow eluminate array copying without sort rewriting
    for input in [[6, 5, 4, 3, 2, 1], [1, 2, 3, 4, 5, 6]].iter() {
        group.bench_with_input(
            BenchmarkId::new("insert_sort", format!("{:?}", input)),
            input,
            |b, i| {
                b.iter(|| {
                    let mut i = i.clone();
                    insert_sort(i.as_mut());
                });
            },
        );

        // TODO: investigate why this bench slower than original
        group.bench_with_input(
            BenchmarkId::new("insertion_sort2", format!("{:?}", input)),
            input,
            |b, i| {
                b.iter(|| {
                    let mut i = i.clone();
                    insertion_sort2(i.as_mut());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("selection_sort", format!("{:?}", input)),
            input,
            |b, i| {
                b.iter(|| {
                    let mut i = i.clone();
                    selection_sort(i.as_mut());
                });
            },
        );
    }
}

criterion_group!(benches, insertion_sorts_benchmark);
criterion_main!(benches);
