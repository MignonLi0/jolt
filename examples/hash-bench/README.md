# Blake2b Hash Benchmark

Performance benchmarks for Blake2b-512 digest implementations in `jolt-inlines-blake2`.

## How to Run

```bash
cd examples/hash-bench
RUST_LOG=info cargo run --release
```

## Implementations

| Function | Description |
|----------|-------------|
| `digest()` | Original implementation with internal buffer copy |
| `digest_no_copy()` | Zero-copy, processes blocks directly from input |
| `digest_64()` | Optimized for ≤64 bytes with explicit 8-byte reads |
| `digest_64_split()` | Takes two `&[u8; 32]` arrays, fastest for 64 bytes |

---

## Results (RV64IMAC Cycles)

### Aligned Input

| Size | `digest()` | `digest_no_copy()` | `digest_64()` | `split()` | Best | Speedup |
|------|------------|-------------------|---------------|-----------|------|---------|
| 64 B | 609 | 344 | 345 | **241** | split | **-60%** |
| 256 B | 846 | **336** | - | - | no_copy | **-60%** |
| 1024 B | 2,400 | **1,128** | - | - | no_copy | **-53%** |
| 2048 B | 4,473 | **2,184** | - | - | no_copy | **-51%** |

### Unaligned Input

| Size | `digest()` | `digest_no_copy()` | `digest_64()` | Best | Speedup |
|------|------------|-------------------|---------------|------|---------|
| 64 B | 689 | 439 | **434** | d64 | **-37%** |
| 256 B | 1,298 | **638** | - | no_copy | **-51%** |
| 1024 B | 4,665 | **2,336** | - | no_copy | **-50%** |
| 2048 B | 9,152 | **4,598** | - | no_copy | **-50%** |

---

## Analysis

### Aligned vs Unaligned Penalty

| Size | Aligned | Unaligned | Penalty |
|------|---------|-----------|---------|
| 64 B (no_copy) | 344 | 439 | +28% |
| 256 B (no_copy) | 336 | 638 | +90% |
| 1024 B (no_copy) | 1,128 | 2,336 | +107% |
| 2048 B (no_copy) | 2,184 | 4,598 | +111% |

### Throughput (Aligned, Best Implementation)

| Size | Cycles | Cycles/Byte |
|------|--------|-------------|
| 64 B | 241 | 3.77 |
| 256 B | 336 | 1.31 |
| 1024 B | 1,128 | 1.10 |
| 2048 B | 2,184 | 1.07 |

---

## Key Findings

1. **`digest_no_copy()` vs `digest()`**: 50-60% faster across all sizes
2. **`digest_64_split()` is fastest for 64B**: 241 cycles (-60% vs original)
3. **Alignment matters**: Unaligned penalty ranges from +28% to +111%
4. **Larger inputs amortize overhead**: Per-byte cost drops from 3.77 to 1.07

---

## Recommendations

| Use Case | Function | Cycles |
|----------|----------|--------|
| 64 bytes, split available | `digest_64_split(left, right)` | **241** |
| 64 bytes, single slice | `digest_no_copy()` or `digest_64()` | ~344 |
| 64 bytes, unaligned | `digest_64()` | 434 |
| >64 bytes | `digest_no_copy()` | varies |

---

## Example

```rust
use jolt_inlines_blake2::Blake2b;

// 64 bytes - fastest
let left: &[u8; 32] = data[..32].try_into().unwrap();
let right: &[u8; 32] = data[32..].try_into().unwrap();
let hash = Blake2b::digest_64_split(left, right);  // 241 cycles

// Large input
let hash = Blake2b::digest_no_copy(&large_data);
```
