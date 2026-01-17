use simd_check::*;

fn main() {
    println!("Normal sum: {}", normal_sum());
    println!("SIMD sum: {:?}", simd_sum());

    println!("Normal dot product: {:?}", normal_dot_vector());
    println!("SIMD dot product: {:?}", simd_dot_vector());

    println!("Normal array manipulation: {:?}", normal_array_actions());
    println!("SIMD array manipulation: {:?}", simd_array_actions());

    println!("Normal min max: {:?}", normal_max_min());
    println!("SIMD min max: {:?}", simd_max_min());

    println!(
        "Normal matrix multiplication: {:?}",
        normal_matrix_multiplication()
    );
    println!(
        "SIMD matrix multiplication: {:?}",
        simd_matrix_multiplication()
    );

    println!(
        "Normal euclidean distance: {:?}",
        normal_euclidean_distance()
    );
    println!("SIMD euclidean distance: {:?}", simd_euclidean_distance());
}

// RGB to grayscale conversion - Practical image processing example with interesting data layout challenges
// Array filtering/counting (e.g., count elements > threshold) - Shows how to handle conditional logic with SIMD masks
// Moving average or convolution - Demonstrates sliding window operations and memory access patterns
// String operations (find character, compare, count bytes) - Surprisingly effective with SIMD, different from numeric operations
