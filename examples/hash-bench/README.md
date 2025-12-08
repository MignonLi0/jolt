# Hash Benchmark

Performance benchmarks for hash implementations in `jolt-inlines`.

## How to Run

```bash
cd examples/hash-bench
RUST_LOG=info cargo run --release
```

---

## Blake2b Results (RV64IMAC Cycles)

| Size | `digest()` | `digest_no_copy()` | `digest_64_split()` | Best | Improvement |
|------|------------|-------------------|---------------------|------|-------------|
| 64B | 603 | 341 | **236** | split | **-61%** |
| 256B | 831 | **326** | - | no_copy | **-61%** |

---

## Blake3 Results (RV64IMAC Cycles)

| Size | `digest()` | `digest_no_copy()` | `digest_64_split()` | Best | Improvement |
|------|------------|-------------------|---------------------|------|-------------|
| 32B | 294 | **122** | - | no_copy | **-59%** |
| 64B | 272 | **147** | 206 | no_copy | **-46%** |

---

## Keccak256 Results (RV64IMAC Cycles)

| Size | `digest()` | `digest_no_copy()` | `digest_64_split()` | Best | Improvement |
|------|------------|-------------------|---------------------|------|-------------|
| 32B | 719 | **361** | - | no_copy | **-50%** |
| 64B | 729 | 385 | **292** | split | **-60%** |

---

## Summary

| Hash | Size | Best Function | Cycles | vs Original |
|------|------|---------------|--------|-------------|
| **Blake2b** | 64B | `digest_64_split()` | **236** | -61% |
| **Blake2b** | 256B | `digest_no_copy()` | **326** | -61% |
| **Blake3** | 32B | `digest_no_copy()` | **122** | -59% |
| **Blake3** | 64B | `digest_no_copy()` | **147** | -46% |
| **Keccak256** | 32B | `digest_no_copy()` | **361** | -50% |
| **Keccak256** | 64B | `digest_64_split()` | **292** | -60% |

---

## Key Findings

1. **Zero-copy wins**: Avoiding internal buffer copy saves 46-61% cycles
2. **Blake2b & Keccak256**: `digest_64_split()` is fastest for 64B (u64 words)
3. **Blake3**: `digest_no_copy()` is fastest (u32 words make split slower)
4. **Consistent gains**: All hash functions achieve ~50-60% improvement

## Recommendations

| Hash | 32B | 64B | >64B |
|------|-----|-----|------|
| Blake2b | `digest_no_copy()` | `digest_64_split()` | `digest_no_copy()` |
| Blake3 | `digest_no_copy()` | `digest_no_copy()` | N/A (max 64B) |
| Keccak256 | `digest_no_copy()` | `digest_64_split()` | `digest_no_copy()` |
