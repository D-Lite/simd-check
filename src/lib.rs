#![feature(portable_simd)]

use std::simd::num::SimdInt;
use std::simd::*;

type I32x2s = i32x2;
type I32x4s = i32x4;
type I32x8s = i32x8;

pub fn normal_sum() -> i32 {
    let x: i32 = 15;
    let y: i32 = 40;

    x + y
}

pub fn simd_sum() -> I32x4s {
    // because SIMD won't take scalar values, we can convert
    // to an array with splat() or from_array
    let x: I32x4s = I32x4s::splat(15);
    let y: I32x4s = I32x4s::from_array([40; 4]);

    x + y
}

// a ⋅ b = (\(a_{x}b_{x}+a_{y}b_{y}+a_{z}b_{z}\)).
pub fn normal_dot_vector() -> Vec<i32> {
    let a: Vec<i32> = vec![12, 200, 54, 982];
    let b: Vec<i32> = vec![91, 42, 65, 21];

    let mut c: Vec<i32> = vec![0; 4];

    for i in 0..4 {
        c[i] = a[i] + b[i];
    }

    c
}

pub fn simd_dot_vector() -> I32x4s {
    let a: I32x4s = I32x4s::from_array([12, 200, 54, 982]);
    let b: I32x4s = I32x4s::from_array([91, 42, 65, 21]);

    a + b
}

pub fn normal_array_actions() -> Vec<Vec<i32>> {
    let a: Vec<i32> = vec![12, 200, 54, 982];
    let b: Vec<i32> = vec![91, 42, 65, 21];

    let mut add_arr: Vec<i32> = vec![0; 4];
    let mut sub_arr: Vec<i32> = vec![0; 4];
    let mut xly_arr: Vec<i32> = vec![0; 4];

    for i in 0..a.len() {
        add_arr[i] = a[i] + b[i];
        sub_arr[i] = a[i] - b[i];
        xly_arr[i] = a[i] * b[i];
    }

    vec![add_arr, sub_arr, xly_arr]
}

pub fn simd_array_actions() -> [Simd<i32, 4>; 3] {
    let a: I32x4s = I32x4s::from_array([12, 200, 54, 982]);
    let b: I32x4s = I32x4s::from_array([91, 42, 65, 21]);

    let add_arr = a + b;
    let sub_arr = a - b;
    let xly_arr = a * b;

    [add_arr, sub_arr, xly_arr]
}

pub fn normal_max_min() -> [i32; 2] {
    let array: Vec<i32> = vec![30, 291, 1, 20, 40, 202, 329, 0];

    let max = array.iter().max().copied().unwrap();
    let min = array.iter().min().copied().unwrap();

    [max, min]
}

pub fn simd_max_min() -> Simd<i32, 2> {
    let array: I32x8s = I32x8s::from_array([30, 291, 1, 20, 40, 202, 329, 0]);

    let max = array.reduce_max();
    let min = array.reduce_min();

    I32x2s::from_array([max, min])
}

pub fn normal_matrix_multiplication() -> [[i32; 4]; 4] {
    let matrix: [[i32; 4]; 4] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ];

    let matrix2: [[i32; 4]; 4] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ];

    let mut dot_product: [[i32; 4]; 4] = [[0; 4]; 4];

    for i in 0..matrix.len() {
        for j in 0..matrix2.len() {
            let mut sum = 0;
            for k in 0..matrix.len() {
                sum += matrix[i][k] * matrix2[k][j]
            }
            dot_product[i][j] = sum;
        }
    }

    dot_product
}

pub fn simd_matrix_multiplication() -> [Simd<i32, 4>; 4] {
    let matrix = [
        I32x4s::from_array([1, 2, 3, 4]),
        I32x4s::from_array([5, 6, 7, 8]),
        I32x4s::from_array([9, 10, 11, 12]),
        I32x4s::from_array([13, 14, 15, 16]),
    ];

    let matrix2 = [
        I32x4s::from_array([1, 2, 3, 4]),
        I32x4s::from_array([5, 6, 7, 8]),
        I32x4s::from_array([9, 10, 11, 12]),
        I32x4s::from_array([13, 14, 15, 16]),
    ];

    let mut result = [I32x4s::splat(0); 4];

    for i in 0..matrix.len() {
        let row = matrix[i];
        for j in 0..matrix2.len() {
            let col =
                I32x4s::from_array([matrix2[0][j], matrix2[1][j], matrix2[2][j], matrix2[3][j]]);

            let product = row * col;
            result[i][j] = product.reduce_sum();
        }
    }

    result
}

// √[(x₂-x₁)² + (y₂-y₁)² + (z₂-z₁)² + (w₂-w₁)²]
pub fn normal_euclidean_distance() -> i32 {
    let mut distance_sqr: i32 = 0;

    let p1: Vec<i32> = vec![1, 2, 3, 4];
    let p2: Vec<i32> = vec![5, 6, 7, 8];

    for i in 0..p1.len() {
        distance_sqr += (p2[i] - p1[i]).pow(2);
    }

    distance_sqr.isqrt()
}

pub fn simd_euclidean_distance() -> i32 {
    let p1: I32x4s = I32x4s::from_array([1, 2, 3, 4]);
    let p2: I32x4s = I32x4s::from_array([5, 6, 7, 8]);

    let temp_diff: I32x4s = p2 - p1;
    let distance_sqr = temp_diff * temp_diff;

    distance_sqr.reduce_sum().isqrt()
}
