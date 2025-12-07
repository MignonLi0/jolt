#![cfg_attr(feature = "guest", no_std)]

use jolt::provable;

use jolt_inlines_keccak256 as keccak_inline;
use sha3::{Digest, Keccak256 as Keccak256Ref};

#[provable(
    max_output_size = 4096,
    memory_size = 33554432,
    stack_size = 10485760,
    max_trace_length = 20553600
)]
fn hashbench() -> [u8; 32] {
    // Empty
    test(b"");

    // Short
    test(b"hello");

    // 64 bytes
    test(&[0xABu8; 64]);

    // 135 bytes (rate - 1)
    test(&[0xBCu8; 135]);

    // 136 bytes (= rate, one full block)
    test(&[0xCDu8; 136]);

    // 137 bytes (rate + 1)
    test(&[0xDEu8; 137]);

    // 256 bytes
    test(&[0xEFu8; 256]);

    // 272 bytes (2 * rate)
    test(&[0x12u8; 272]);

    return [0; 32];
}

fn test(input: &[u8]) {
    let ref_result = Keccak256Ref::digest(input);
    let inline_result = keccak_inline::Keccak256::digest(input);
    assert_eq!(ref_result.as_slice(), &inline_result[..]);
}
