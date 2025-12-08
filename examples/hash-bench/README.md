# Hash Benchmark

Performance benchmarks for hash implementations in `jolt-inlines`.

## How to Run

```bash
cd examples/hash-bench
RUST_LOG=info cargo run --release
```

---

## Blake2b Results (RV64IMAC Cycles)

| Size | `digest()` | `digest_no_copy()` | `digest_64_split()` | Improvement |
|------|------------|-------------------|---------------------|-------------|
| 64B | 603 | 341 | **237** | **-61%** |
| 128B | 570 | **192** | - | **-66%** |
| 256B | 830 | **327** | - | **-61%** |
| 512B | 1,346 | **590** | - | **-56%** |

---

## Blake3 Results (RV64IMAC Cycles)

*Note: Blake3 only supports up to 64 bytes input*

| Size | `digest()` | `digest_no_copy()` | `digest_64_split()` | Improvement |
|------|------------|-------------------|---------------------|-------------|
| 32B | 301 | **122** | - | **-59%** |
| 64B | 267 | **147** | 205 | **-45%** |

---

## Keccak256 Results (RV64IMAC Cycles)

| Size | `digest()` | `digest_no_copy()` | `digest_64_split()` | Improvement |
|------|------------|-------------------|---------------------|-------------|
| 32B | 697 | **360** | - | **-48%** |
| 64B | 704 | 384 | **292** | **-59%** |
| 136B | 848 | **377** | - | **-56%** |
| 256B | 935 | **499** | - | **-47%** |
| 512B | 1,323 | **639** | - | **-52%** |

---

## Summary

| Hash | Size | Best Function | Cycles | vs Original |
|------|------|---------------|--------|-------------|
| **Blake2b** | 64B | `digest_64_split()` | **237** | -61% |
| **Blake2b** | 128B | `digest_no_copy()` | **192** | -66% |
| **Blake2b** | 256B | `digest_no_copy()` | **327** | -61% |
| **Blake2b** | 512B | `digest_no_copy()` | **590** | -56% |
| **Blake3** | 32B | `digest_no_copy()` | **122** | -59% |
| **Blake3** | 64B | `digest_no_copy()` | **147** | -45% |
| **Keccak256** | 32B | `digest_no_copy()` | **360** | -48% |
| **Keccak256** | 64B | `digest_64_split()` | **292** | -59% |
| **Keccak256** | 136B | `digest_no_copy()` | **377** | -56% |
| **Keccak256** | 256B | `digest_no_copy()` | **499** | -47% |
| **Keccak256** | 512B | `digest_no_copy()` | **639** | -52% |

---

## Key Findings

1. **Consistent 45-66% improvement** across all hash functions and sizes
2. **Blake2b 128B is most efficient**: 192 cycles (-66%)
3. **Blake3 32B is fastest overall**: 122 cycles
4. **Split function best for 64B** on Blake2b and Keccak256 (u64 words)

## Recommendations

| Hash | 32B | 64B | 128B+ |
|------|-----|-----|-------|
| Blake2b | `no_copy` | `split` | `no_copy` |
| Blake3 | `no_copy` | `no_copy` | N/A (max 64B) |
| Keccak256 | `no_copy` | `split` | `no_copy` |
