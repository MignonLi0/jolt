#![cfg_attr(feature = "guest", no_std)]

use core::hint::black_box;

use jolt::{end_cycle_tracking, start_cycle_tracking};

use jolt_inlines_blake2 as blake2_inline;

#[jolt::provable(
    max_output_size = 4096,
    memory_size = 33554432,
    stack_size = 10485760,
    max_trace_length = 20553600
)]
fn hashbench() -> [u8; 32] {
    // 64 bytes
    benchmark_64();
    benchmark_64_unaligned();

    // 256 bytes
    benchmark_256();
    benchmark_256_unaligned();

    // 1024 bytes
    benchmark_1024();
    benchmark_1024_unaligned();

    // 2048 bytes
    benchmark_2048();
    benchmark_2048_unaligned();

    return [0; 32];
}

fn assign_random_looking_values(array: &mut [u8], seed: u32) {
    const A: u32 = 1664525;
    const C: u32 = 1013904223;
    let mut state = seed;
    for item in array {
        state = state.wrapping_mul(A).wrapping_add(C);
        let value = (state ^ (state >> 16)) as u8;
        *item = value;
    }
}

// ==================== 64 bytes ====================

fn benchmark_64() {
    let mut input = [5u8; 64];
    assign_random_looking_values(&mut input, 20);

    let left: &[u8; 32] = input[..32].try_into().unwrap();
    let right: &[u8; 32] = input[32..].try_into().unwrap();

    start_cycle_tracking("digest_64");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(&input[..])));
    end_cycle_tracking("digest_64");

    start_cycle_tracking("no_copy_64");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("no_copy_64");

    start_cycle_tracking("d64_64");
    let r3 = black_box(blake2_inline::Blake2b::digest_64(black_box(&input[..])));
    end_cycle_tracking("d64_64");

    start_cycle_tracking("split_64");
    let r4 = black_box(blake2_inline::Blake2b::digest_64_split(black_box(left), black_box(right)));
    end_cycle_tracking("split_64");

    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
    assert_eq!(r3, r4);
}

fn benchmark_64_unaligned() {
    let mut buffer = [5u8; 65];
    assign_random_looking_values(&mut buffer, 20);
    let input = &buffer[1..];

    start_cycle_tracking("digest_64_un");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(input)));
    end_cycle_tracking("digest_64_un");

    start_cycle_tracking("no_copy_64_un");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(input)));
    end_cycle_tracking("no_copy_64_un");

    start_cycle_tracking("d64_64_un");
    let _ = black_box(blake2_inline::Blake2b::digest_64(black_box(input)));
    end_cycle_tracking("d64_64_un");

    assert_eq!(r1, r2);
}

// ==================== 256 bytes ====================

fn benchmark_256() {
    let mut input = [5u8; 256];
    assign_random_looking_values(&mut input, 20);

    start_cycle_tracking("digest_256");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(&input[..])));
    end_cycle_tracking("digest_256");

    start_cycle_tracking("no_copy_256");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("no_copy_256");

    assert_eq!(r1, r2);
}

fn benchmark_256_unaligned() {
    let mut buffer = [5u8; 257];
    assign_random_looking_values(&mut buffer, 20);
    let input = &buffer[1..];

    start_cycle_tracking("digest_256_un");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(input)));
    end_cycle_tracking("digest_256_un");

    start_cycle_tracking("no_copy_256_un");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(input)));
    end_cycle_tracking("no_copy_256_un");

    assert_eq!(r1, r2);
}

// ==================== 1024 bytes ====================

fn benchmark_1024() {
    let mut input = [5u8; 1024];
    assign_random_looking_values(&mut input, 20);

    start_cycle_tracking("digest_1024");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(&input[..])));
    end_cycle_tracking("digest_1024");

    start_cycle_tracking("no_copy_1024");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("no_copy_1024");

    assert_eq!(r1, r2);
}

fn benchmark_1024_unaligned() {
    let mut buffer = [5u8; 1025];
    assign_random_looking_values(&mut buffer, 20);
    let input = &buffer[1..];

    start_cycle_tracking("digest_1024_un");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(input)));
    end_cycle_tracking("digest_1024_un");

    start_cycle_tracking("no_copy_1024_un");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(input)));
    end_cycle_tracking("no_copy_1024_un");

    assert_eq!(r1, r2);
}

// ==================== 2048 bytes ====================

fn benchmark_2048() {
    let mut input = [5u8; 2048];
    assign_random_looking_values(&mut input, 20);

    start_cycle_tracking("digest_2048");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(&input[..])));
    end_cycle_tracking("digest_2048");

    start_cycle_tracking("no_copy_2048");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("no_copy_2048");

    assert_eq!(r1, r2);
}

fn benchmark_2048_unaligned() {
    let mut buffer = [5u8; 2049];
    assign_random_looking_values(&mut buffer, 20);
    let input = &buffer[1..];

    start_cycle_tracking("digest_2048_un");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(input)));
    end_cycle_tracking("digest_2048_un");

    start_cycle_tracking("no_copy_2048_un");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(input)));
    end_cycle_tracking("no_copy_2048_un");

    assert_eq!(r1, r2);
}
