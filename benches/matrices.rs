use clrs_study::divide_and_conquer::matrix::Matrix;
use criterion::{criterion_group, criterion_main, Criterion, black_box};

pub fn matrices_multiplication_benchmark(c: &mut Criterion) {
    let new_matrix = || {
        let m = Matrix::<64>::zeros();
        let mut view = m.view();
        for i in 0..m.len() {
            for j in 0..m.len() {
                view.set_data(i, j, rand::random());
            }
        }
        m
    };

    let mat = new_matrix();

    let mut group = c.benchmark_group("matrices_mul");

    group.sample_size(10).bench_function("simple", |b| {
        b.iter(|| {
            let m1 = mat.clone();
            let m2 = mat.clone();
            let _ = black_box(m1).mul(black_box(m2));
        });
    });

    group.sample_size(10).bench_function("dnc_mul", |b| {
        b.iter(|| {
            // TODO: copy matrices for more honest benchmarking with simple multiplication
            let m1 = mat.clone();
            let m2 = mat.clone();
            let _ = black_box(m1).dnc_mul(black_box(&m2));
        });
    });

    group.sample_size(10).bench_function("strassen_mul", |b| {
        b.iter(|| {
            let m1 = mat.clone();
            let m2 = mat.clone();
            let _ = black_box(m1).strassen_mul(black_box(m2));
        });
    });
}

criterion_group!(matrices_benchs, matrices_multiplication_benchmark);
criterion_main!(matrices_benchs);
