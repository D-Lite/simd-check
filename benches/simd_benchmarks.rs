#![feature(portable_simd)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use simd_check::*;

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum");

    group.bench_function("normal", |b| b.iter(|| black_box(normal_sum())));

    group.bench_function("simd", |b| b.iter(|| black_box(simd_sum())));

    group.finish();
}

fn bench_dot_product(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dot Product");

    group.bench_function("normal", |b| b.iter(|| black_box(normal_dot_vector())));

    group.bench_function("simd", |b| b.iter(|| black_box(simd_dot_vector())));

    group.finish();
}

fn bench_array_actions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Array Operations");

    group.bench_function("normal", |b| b.iter(|| black_box(normal_array_actions())));

    group.bench_function("simd", |b| b.iter(|| black_box(simd_array_actions())));

    group.finish();
}

fn bench_max_min(c: &mut Criterion) {
    let mut group = c.benchmark_group("Max Min");

    group.bench_function("normal", |b| b.iter(|| black_box(normal_max_min())));

    group.bench_function("simd", |b| b.iter(|| black_box(simd_max_min())));

    group.finish();
}

fn bench_matrix_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix Multiplication");

    group.bench_function("normal", |b| {
        b.iter(|| black_box(normal_matrix_multiplication()))
    });

    group.bench_function("simd", |b| {
        b.iter(|| black_box(simd_matrix_multiplication()))
    });

    group.finish();
}

fn bench_euclidean_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("Euclidean Distance");

    group.bench_function("normal", |b| {
        b.iter(|| black_box(normal_euclidean_distance()))
    });

    group.bench_function("simd", |b| b.iter(|| black_box(simd_euclidean_distance())));

    group.finish();
}

criterion_group!(
    benches,
    bench_sum,
    bench_dot_product,
    bench_array_actions,
    bench_max_min,
    bench_matrix_multiplication,
    bench_euclidean_distance
);
criterion_main!(benches);
