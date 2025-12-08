#![cfg_attr(feature = "guest", no_std)]

use core::hint::black_box;

use jolt::{end_cycle_tracking, start_cycle_tracking};

use jolt_inlines_blake2 as blake2_inline;
use jolt_inlines_blake3 as blake3_inline;
use jolt_inlines_keccak256 as keccak_inline;

#[jolt::provable(
    max_output_size = 4096,
    memory_size = 33554432,
    stack_size = 10485760,
    max_trace_length = 20553600
)]
fn hashbench() -> [u8; 32] {
    // Blake2b
    benchmark_blake2_64();
    benchmark_blake2_128();
    benchmark_blake2_256();
    benchmark_blake2_512();

    // Blake3
    benchmark_blake3_32();
    benchmark_blake3_64();

    // Keccak256
    benchmark_keccak_32();
    benchmark_keccak_64();
    benchmark_keccak_136();
    benchmark_keccak_256();
    benchmark_keccak_512();

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

// ==================== Blake2b ====================

fn benchmark_blake2_64() {
    let mut input = [5u8; 64];
    assign_random_looking_values(&mut input, 20);
    let left: &[u8; 32] = input[..32].try_into().unwrap();
    let right: &[u8; 32] = input[32..].try_into().unwrap();

    start_cycle_tracking("b2_digest_64");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(&input[..])));
    end_cycle_tracking("b2_digest_64");

    start_cycle_tracking("b2_no_copy_64");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("b2_no_copy_64");

    start_cycle_tracking("b2_split_64");
    let r3 = black_box(blake2_inline::Blake2b::digest_64_split(black_box(left), black_box(right)));
    end_cycle_tracking("b2_split_64");

    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
}

fn benchmark_blake2_128() {
    let mut input = [5u8; 128];
    assign_random_looking_values(&mut input, 21);

    start_cycle_tracking("b2_digest_128");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(&input[..])));
    end_cycle_tracking("b2_digest_128");

    start_cycle_tracking("b2_no_copy_128");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("b2_no_copy_128");

    assert_eq!(r1, r2);
}

fn benchmark_blake2_256() {
    let mut input = [5u8; 256];
    assign_random_looking_values(&mut input, 22);

    start_cycle_tracking("b2_digest_256");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(&input[..])));
    end_cycle_tracking("b2_digest_256");

    start_cycle_tracking("b2_no_copy_256");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("b2_no_copy_256");

    assert_eq!(r1, r2);
}

fn benchmark_blake2_512() {
    let mut input = [5u8; 512];
    assign_random_looking_values(&mut input, 23);

    start_cycle_tracking("b2_digest_512");
    let r1 = black_box(blake2_inline::Blake2b::digest(black_box(&input[..])));
    end_cycle_tracking("b2_digest_512");

    start_cycle_tracking("b2_no_copy_512");
    let r2 = black_box(blake2_inline::Blake2b::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("b2_no_copy_512");

    assert_eq!(r1, r2);
}

// ==================== Blake3 ====================

fn benchmark_blake3_32() {
    let mut input = [5u8; 32];
    assign_random_looking_values(&mut input, 30);

    start_cycle_tracking("b3_digest_32");
    let r1 = black_box(blake3_inline::Blake3::digest(black_box(&input[..])));
    end_cycle_tracking("b3_digest_32");

    start_cycle_tracking("b3_no_copy_32");
    let r2 = black_box(blake3_inline::Blake3::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("b3_no_copy_32");

    assert_eq!(r1, r2);
}

fn benchmark_blake3_64() {
    let mut input = [5u8; 64];
    assign_random_looking_values(&mut input, 31);
    let left: &[u8; 32] = input[..32].try_into().unwrap();
    let right: &[u8; 32] = input[32..].try_into().unwrap();

    start_cycle_tracking("b3_digest_64");
    let r1 = black_box(blake3_inline::Blake3::digest(black_box(&input[..])));
    end_cycle_tracking("b3_digest_64");

    start_cycle_tracking("b3_no_copy_64");
    let r2 = black_box(blake3_inline::Blake3::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("b3_no_copy_64");

    start_cycle_tracking("b3_split_64");
    let r3 = black_box(blake3_inline::Blake3::digest_64_split(black_box(left), black_box(right)));
    end_cycle_tracking("b3_split_64");

    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
}

// ==================== Keccak256 ====================

fn benchmark_keccak_32() {
    let mut input = [5u8; 32];
    assign_random_looking_values(&mut input, 40);

    start_cycle_tracking("kec_digest_32");
    let r1 = black_box(keccak_inline::Keccak256::digest(black_box(&input[..])));
    end_cycle_tracking("kec_digest_32");

    start_cycle_tracking("kec_no_copy_32");
    let r2 = black_box(keccak_inline::Keccak256::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("kec_no_copy_32");

    assert_eq!(r1, r2);
}

fn benchmark_keccak_64() {
    let mut input = [5u8; 64];
    assign_random_looking_values(&mut input, 41);
    let left: &[u8; 32] = input[..32].try_into().unwrap();
    let right: &[u8; 32] = input[32..].try_into().unwrap();

    start_cycle_tracking("kec_digest_64");
    let r1 = black_box(keccak_inline::Keccak256::digest(black_box(&input[..])));
    end_cycle_tracking("kec_digest_64");

    start_cycle_tracking("kec_no_copy_64");
    let r2 = black_box(keccak_inline::Keccak256::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("kec_no_copy_64");

    start_cycle_tracking("kec_split_64");
    let r3 = black_box(keccak_inline::Keccak256::digest_64_split(black_box(left), black_box(right)));
    end_cycle_tracking("kec_split_64");

    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
}

fn benchmark_keccak_136() {
    let mut input = [5u8; 136];
    assign_random_looking_values(&mut input, 42);

    start_cycle_tracking("kec_digest_136");
    let r1 = black_box(keccak_inline::Keccak256::digest(black_box(&input[..])));
    end_cycle_tracking("kec_digest_136");

    start_cycle_tracking("kec_no_copy_136");
    let r2 = black_box(keccak_inline::Keccak256::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("kec_no_copy_136");

    assert_eq!(r1, r2);
}

fn benchmark_keccak_256() {
    let mut input = [5u8; 256];
    assign_random_looking_values(&mut input, 43);

    start_cycle_tracking("kec_digest_256");
    let r1 = black_box(keccak_inline::Keccak256::digest(black_box(&input[..])));
    end_cycle_tracking("kec_digest_256");

    start_cycle_tracking("kec_no_copy_256");
    let r2 = black_box(keccak_inline::Keccak256::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("kec_no_copy_256");

    assert_eq!(r1, r2);
}

fn benchmark_keccak_512() {
    let mut input = [5u8; 512];
    assign_random_looking_values(&mut input, 44);

    start_cycle_tracking("kec_digest_512");
    let r1 = black_box(keccak_inline::Keccak256::digest(black_box(&input[..])));
    end_cycle_tracking("kec_digest_512");

    start_cycle_tracking("kec_no_copy_512");
    let r2 = black_box(keccak_inline::Keccak256::digest_no_copy(black_box(&input[..])));
    end_cycle_tracking("kec_no_copy_512");

    assert_eq!(r1, r2);
}
