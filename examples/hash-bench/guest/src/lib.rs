#![cfg_attr(feature = "guest", no_std)]

use core::hint::black_box;

use jolt::{end_cycle_tracking, start_cycle_tracking};

use blake2 as blake2_reference;
use blake3 as blake3_reference;
use jolt_inlines_blake2 as blake2_inline;
use jolt_inlines_blake3 as blake3_inline;
use jolt_inlines_keccak256 as keccak_inline;
use jolt_inlines_sha2 as sha2_inline;
use sha2::{self as sha2_reference, Digest};
use sha3 as keccak_reference;

const INPUT_SIZE_64: usize = 64;
const INPUT_SIZE_256: usize = 256;
const INPUT_SIZE_2048: usize = 2048;

#[jolt::provable(
    max_output_size = 4096,
    memory_size = 33554432,
    stack_size = 10485760,
    max_trace_length = 20553600
)]
fn hashbench() -> [u8; 32] {
    // 64 bytes
    benchmark_blake2_reference_64();
    benchmark_blake2_inline_64();

    // 256 bytes
    benchmark_blake2_reference_256();
    benchmark_blake2_inline_256();

    // 2048 bytes
    benchmark_blake2_reference_2048();
    benchmark_blake2_inline_2048();

    return [0; 32];
}

/// Assigns deterministic random-looking values to array
/// Uses a simple Linear Congruential Generator (LCG) algorithm to fill the array
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

#[allow(dead_code)]
fn benchmark_sha2_reference() {
    let mut input = [5u8; INPUT_SIZE_2048];
    assign_random_looking_values(&mut input, 40);
    start_cycle_tracking("sha2_reference");
    let result = black_box(sha2_reference::Sha256::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("sha2_reference");
}

#[allow(dead_code)]
fn benchmark_sha2_inline() {
    let mut input = [5u8; INPUT_SIZE_2048];
    assign_random_looking_values(&mut input, 40);
    start_cycle_tracking("sha2_inline");
    let result = black_box(sha2_inline::Sha256::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("sha2_inline");
}

#[allow(dead_code)]
fn benchmark_keccak_reference() {
    let mut input = [5u8; INPUT_SIZE_2048];
    assign_random_looking_values(&mut input, 30);
    start_cycle_tracking("keccak_reference");
    let result = black_box(keccak_reference::Keccak256::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("keccak_reference");
}

#[allow(dead_code)]
fn benchmark_keccak_inline() {
    let mut input = [5u8; INPUT_SIZE_2048];
    assign_random_looking_values(&mut input, 30);
    start_cycle_tracking("keccak_inline");
    let result = black_box(keccak_inline::Keccak256::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("keccak_inline");
}

fn benchmark_blake2_reference_64() {
    let mut input = [5u8; INPUT_SIZE_64];
    assign_random_looking_values(&mut input, 20);
    start_cycle_tracking("blake2_reference_64");
    let result = black_box(blake2_reference::Blake2b512::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("blake2_reference_64");
}

fn benchmark_blake2_reference_256() {
    let mut input = [5u8; INPUT_SIZE_256];
    assign_random_looking_values(&mut input, 20);
    start_cycle_tracking("blake2_reference_256");
    let result = black_box(blake2_reference::Blake2b512::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("blake2_reference_256");
}

fn benchmark_blake2_reference_2048() {
    let mut input = [5u8; INPUT_SIZE_2048];
    assign_random_looking_values(&mut input, 20);
    start_cycle_tracking("blake2_reference_2048");
    let result = black_box(blake2_reference::Blake2b512::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("blake2_reference_2048");
}

fn benchmark_blake2_inline_64() {
    let mut input = [5u8; INPUT_SIZE_64];
    assign_random_looking_values(&mut input, 20);
    start_cycle_tracking("blake2_inline_64");
    let result = black_box(blake2_inline::Blake2b::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("blake2_inline_64");
}

fn benchmark_blake2_inline_256() {
    let mut input = [5u8; INPUT_SIZE_256];
    assign_random_looking_values(&mut input, 20);
    start_cycle_tracking("blake2_inline_256");
    let result = black_box(blake2_inline::Blake2b::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("blake2_inline_256");
}

fn benchmark_blake2_inline_2048() {
    let mut input = [5u8; INPUT_SIZE_2048];
    assign_random_looking_values(&mut input, 20);
    start_cycle_tracking("blake2_inline_2048");
    let result = black_box(blake2_inline::Blake2b::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("blake2_inline_2048");
}

#[allow(dead_code)]
fn benchmark_blake3_reference() {
    let mut input = [5u8; INPUT_SIZE_64];
    assign_random_looking_values(&mut input, 10);
    start_cycle_tracking("blake3_reference");
    let result = black_box(blake3_reference::hash(black_box(&input)));
    black_box(result);
    end_cycle_tracking("blake3_reference");
}

#[allow(dead_code)]
fn benchmark_blake3_inline() {
    let mut input = [5u8; INPUT_SIZE_64];
    assign_random_looking_values(&mut input, 10);
    start_cycle_tracking("blake3_inline");
    let result: [u8; 32] = black_box(blake3_inline::Blake3::digest(black_box(&input)));
    black_box(result);
    end_cycle_tracking("blake3_inline");
}
