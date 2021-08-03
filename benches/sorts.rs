use clrs_study::{exercises::Inversions, sorts::*};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

static LARGE_TEST_SAMPLE: once_cell::sync::Lazy<Vec<i32>> = once_cell::sync::Lazy::new(|| {
    (0..10_000)
        .into_iter()
        .map(|_| rand::random::<i32>())
        .collect()
});

static LARGE_TEST_SAMPLE_2: once_cell::sync::Lazy<Vec<i32>> = once_cell::sync::Lazy::new(|| {
    (0..1_000_000)
        .into_iter()
        .map(|_| rand::random::<i32>())
        .collect()
});

static LARGE_TEST_SAMPLE_3: once_cell::sync::Lazy<Vec<i32>> = once_cell::sync::Lazy::new(|| {
    (0..100_000)
        .into_iter()
        .map(|_| rand::random::<i32>())
        .collect()
});

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
    }
}

pub fn merge_sorts_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge sort");

    // TODO: somehow eluminate array copying without sort rewriting
    for input in [[6, 5, 4, 3, 2, 1], [1, 2, 3, 4, 5, 6]].iter() {
        group.bench_with_input(
            BenchmarkId::new("merge_sort", format!("{:?}", input)),
            input,
            |b, i| {
                b.iter(|| {
                    let mut i = i.clone();
                    MergeSort::merge_sort(i.as_mut());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("merge_sort2", format!("{:?}", input)),
            input,
            |b, i| {
                b.iter(|| {
                    let mut i = i.clone();
                    MergeSort2::merge_sort(i.as_mut());
                });
            },
        );
    }
}

pub fn large_test_sample_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_sample_sorts");

    group.bench_function("merge_sort", |b| {
        b.iter(|| {
            let mut l = LARGE_TEST_SAMPLE.clone();
            MergeSort::merge_sort(l.as_mut());
        });
    });

    group.bench_function("merge_sort2", |b| {
        b.iter(|| {
            let mut l = LARGE_TEST_SAMPLE.clone();
            MergeSort2::merge_sort(l.as_mut());
        });
    });

    group.bench_function("insert_sort", |b| {
        b.iter(|| {
            let mut l = LARGE_TEST_SAMPLE.clone();
            insert_sort(l.as_mut());
        });
    });

    group.bench_function("insert_sort2", |b| {
        b.iter(|| {
            let mut l = LARGE_TEST_SAMPLE.clone();
            insertion_sort2(l.as_mut());
        });
    });
}

pub fn large_test_sample_2_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_sample_sorts2");

    group.bench_function("merge_sort", |b| {
        b.iter(|| {
            let mut l = LARGE_TEST_SAMPLE_2.clone();
            MergeSort::merge_sort(l.as_mut());
        });
    });

    group.bench_function("merge_sort2", |b| {
        b.iter(|| {
            let mut l = LARGE_TEST_SAMPLE_2.clone();
            MergeSort2::merge_sort(l.as_mut());
        });
    });

    group.bench_function("merge_insert_sort", |b| {
        b.iter(|| {
            let mut l = LARGE_TEST_SAMPLE_2.clone();
            MergeInsertSort {
                insert_sort_size: 512,
            }
            .merge_sort(l.as_mut());
        });
    });

    group.bench_function("inversions count", |b| {
        b.iter(|| {
            let mut l = LARGE_TEST_SAMPLE_2.clone();
            Inversions::merge_sort(l.as_mut());
        });
    });
}

pub fn large_test_sample_2_merge_insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_sample_sorts_merge_inserts");

    for i in 8..16 {
        group.bench_with_input(BenchmarkId::new("merge_insert_sort", &i), &i, |b, i| {
            b.iter(|| {
                let mut l = LARGE_TEST_SAMPLE_3.clone();
                MergeInsertSort {
                    insert_sort_size: 2usize.pow(*i as u32),
                }
                .merge_sort(l.as_mut());
            });
        });
    }
}

pub fn selection_sort_bench(c: &mut Criterion) {
    c.bench_function("selection_sort", |b| {
        b.iter(|| {
            let mut a = vec![6, 5, 4, 3, 2, 1];
            selection_sort(&mut a);
        })
    });
}

criterion_group!(insertion_sorts, insertion_sorts_benchmark);
criterion_group!(selection_sorts, selection_sort_bench);
criterion_group!(merge_sorts, merge_sorts_benchmark);
criterion_group!(
    large_sample_sorts,
    large_test_sample_benchmark,
    large_test_sample_2_benchmark,
    large_test_sample_2_merge_insert_benchmark
);
criterion_main!(
    insertion_sorts,
    selection_sorts,
    merge_sorts,
    large_sample_sorts
);
