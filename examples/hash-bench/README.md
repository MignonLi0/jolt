# Hash Benchmark

Performance benchmarks for Blake2b and Blake3 digest implementations in `jolt-inlines`.

## How to Run

```bash
cd examples/hash-bench
RUST_LOG=info cargo run --release
```

---

## Blake2b Results (RV64IMAC Cycles)

### Implementations

| Function | Description |
|----------|-------------|
| `digest()` | Original with internal buffer copy |
| `digest_no_copy()` | Zero-copy, processes blocks directly |
| `digest_64_split()` | Two `&[u8;32]` inputs, fastest for 64B |

### 64 Bytes

| Implementation | Aligned | Unaligned | Improvement |
|---------------|---------|-----------|-------------|
| `digest()` | 605 | 684 | baseline |
| `digest_no_copy()` | 343 | 431 | -43% |
| **`digest_64_split()`** | **239** | N/A | **-60%** |

### 256 Bytes

| Implementation | Aligned | Unaligned | Improvement |
|---------------|---------|-----------|-------------|
| `digest()` | 832 | 1,285 | baseline |
| **`digest_no_copy()`** | **329** | 630 | **-60%** |

---

## Blake3 Results (RV64IMAC Cycles)

### Implementations

| Function | Description |
|----------|-------------|
| `digest()` | Original with internal buffer copy |
| `digest_no_copy()` | Zero-copy, direct processing |
| `digest_64_split()` | Two `&[u8;32]` inputs |

### 64 Bytes

| Implementation | Aligned | Unaligned | Improvement |
|---------------|---------|-----------|-------------|
| `digest()` | 272 | 363 | baseline |
| **`digest_no_copy()`** | **149** | 236 | **-45%** |
| `digest_64_split()` | 208 | N/A | -24% |

### 32 Bytes

| Implementation | Aligned | Improvement |
|---------------|---------|-------------|
| `digest()` | 294 | baseline |
| **`digest_no_copy()`** | **122** | **-59%** |

---

## Summary

| Hash | Size | Best Function | Cycles | vs Original |
|------|------|---------------|--------|-------------|
| Blake2b | 64B | `digest_64_split()` | **239** | -60% |
| Blake2b | 256B | `digest_no_copy()` | **329** | -60% |
| Blake3 | 32B | `digest_no_copy()` | **122** | -59% |
| Blake3 | 64B | `digest_no_copy()` | **149** | -45% |

## Key Findings

1. **Blake2b**: `digest_64_split()` is fastest for 64B (-60%), uses u64 words
2. **Blake3**: `digest_no_copy()` is fastest (-45% to -59%), uses u32 words
3. **Unaligned penalty**: +30% to +90% depending on size
4. **Zero-copy wins**: Avoiding internal buffer copy saves 40-60% cycles

## Recommendations

| Use Case | Blake2b | Blake3 |
|----------|---------|--------|
| 64 bytes (aligned) | `digest_64_split()` | `digest_no_copy()` |
| 64 bytes (unaligned) | `digest_no_copy()` | `digest_no_copy()` |
| >64 bytes | `digest_no_copy()` | N/A (max 64B) |
