# BLAKE3 Inline Optimization - Final Implementation

## 概述

优化 BLAKE3 inline 指令，减少 virtual cycles 开销，专注于 Merkle Tree 场景。

### 当前指令清单

| 指令 | FUNCT3 | 用途 | flags | Virtual Cycles |
|------|--------|------|-------|----------------|
| `blake3_compress` | 0x00 | 通用压缩 | 可变 | 723 |
| `blake3_parent64_compress` | 0x04 | Merkle 中间节点 | PARENT | 692 |
| `blake3_parent64_root_compress` | 0x05 | Merkle 根节点 | PARENT \| ROOT | 694 |

---

## 性能测试结果

### BLAKE3 Digest

| 输入 | RV64IMAC | Virtual | Virtual/Byte |
|------|----------|---------|--------------|
| 32B aligned | 135 | 878 | 27.4 |
| 32B unaligned | 177 | 1010 | 31.6 |
| 64B aligned | 126 | 869 | 13.6 |
| 64B unaligned | 221 | 1062 | 16.6 |

### 底层指令

| 指令 | RV64IMAC | Virtual | 说明 |
|------|----------|---------|------|
| `blake3_compress` | 10 | 723 | 通用压缩 |
| `blake3_parent64_compress` | 23 | 692 | Merkle 中间节点 |
| `blake3_parent64_root_compress` | 25 | 694 | Merkle 根节点 |

### Merkle Tree 专用指令 vs 通用方式

| 操作 | 通用方式 | 专用指令 | 提升 |
|------|----------|----------|------|
| parent CV | 1143 | 692 | **-39%** |
| root hash | 1141 | 694 | **-39%** |

---

## 性能优化总结

### 配对访存优化

将单独的 LW/SW 改为 LD/SD 配对访问，显著减少 virtual cycles。

| 操作 | 旧方式 | 新方式 | 节省 |
|------|--------|--------|------|
| 消息加载 (16 u32) | 16 × LW (~112) | 8 × LD pair (~24) | -88 |
| 链值加载 (8 u32) | 8 × LW (~56) | 4 × LD pair (~12) | -44 |
| 结果存储 (8 u32) | 8 × SW (~56) | 4 × SD pair (~16) | -40 |
| **总计** | **~224** | **~52** | **-172** |

### 固定 IV 优化

对于 Merkle Tree 场景，IV 固定为 BLAKE3 标准 IV，无需从内存加载：

| 组件 | 通用压缩 | 固定 IV | 节省 |
|------|----------|---------|------|
| v[0..7] 初始化 | 4 × LD pair (~12) | 8 × LUI (~8) | -4 |
| XOR 复制到 state | 8 × XOR (~8) | 0 (直接用) | -8 |
| **小计** | **~20** | **~8** | **-12** |

---

## Merkle Tree 专用指令

### 1. `blake3_parent64_compress` (FUNCT3 = 0x04)

计算 Merkle Tree 中间节点的 CV。

**ABI**:
| 操作数 | 含义 | 对齐要求 |
|--------|------|----------|
| rs1 | 左子 CV 指针 (32B) | 8 字节 |
| rs2 | 右子 CV 指针 (32B) | 8 字节 |
| rd | 输出 CV 指针 (32B) | 8 字节 |

**参数固定值**:
- IV = BLAKE3 标准 IV
- counter = 0
- block_len = 64
- flags = `PARENT` (0x04)

### 2. `blake3_parent64_root_compress` (FUNCT3 = 0x05)

计算 Merkle Tree 根节点的最终输出。

**ABI**: 同 `blake3_parent64_compress`

**参数固定值**:
- IV = BLAKE3 标准 IV
- counter = 0
- block_len = 64
- flags = `PARENT | ROOT` (0x0C)

---

## SDK API

### 高层 API

```rust
impl Blake3 {
    /// 通用 digest (任意长度，flags 包含 ROOT)
    pub fn digest(input: &[u8]) -> [u8; 32];
}
```

### 底层 API

```rust
/// 通用压缩 (需要提供完整参数)
pub unsafe fn blake3_compress(
    chaining_value: *mut u32,  // rs1: 32B, 输入输出
    message: *const u32,       // rs2: 80B (64B msg + counter + len + flags)
);

/// Merkle 中间节点 (flags = PARENT)
pub unsafe fn blake3_parent64_compress(
    left: *const u32,    // rs1: 32B 左子 CV
    right: *const u32,   // rs2: 32B 右子 CV
    output: *mut u32,    // rd: 32B 输出 CV
);

/// Merkle 根节点 (flags = PARENT | ROOT)
pub unsafe fn blake3_parent64_root_compress(
    left: *const u32,    // rs1: 32B 左子 CV
    right: *const u32,   // rs2: 32B 右子 CV
    output: *mut u32,    // rd: 32B 输出 hash
);
```

---

## 技术实现

### 配对读取 (`load_paired_u32`)

```rust
// 旧方式: 每个 u32 单独加载
for i in 0..8 {
    vr[i] = LW(base, i * 4);     // 每个 LW 展开为 ~7 virtual cycles
}
// 总计: 8 × 7 = 56 cycles

// 新方式: 每次 LD 读取 2 个 u32
for i in 0..4 {
    let dword = LD(base, i * 8);              // 1. 加载 64 bits
    vr[i*2] = VirtualZeroExtendWord(dword);   // 2. 提取低 32 bits
    vr[i*2+1] = SRLI(dword, 32);              // 3. 提取高 32 bits
}
// 总计: 4 × 3 = 12 cycles
```

### 配对写入 (`store_paired_u32`)

```rust
// 旧方式: 每个 u32 单独存储
for i in 0..8 {
    SW(base, i * 4, vr[i]);     // 每个 SW 展开为 ~7 virtual cycles
}
// 总计: 8 × 7 = 56 cycles

// 新方式: 每次 SD 写入 2 个 u32
for i in 0..4 {
    let lo = VirtualZeroExtendWord(vr[i*2]);  // 1. 清除高 32 bits
    let hi = SLLI(vr[i*2+1], 32);             // 2. 高位左移 32
    let dword = OR(lo, hi);                    // 3. 合并
    SD(base, i * 8, dword);                    // 4. 存储 64 bits
}
// 总计: 4 × 4 = 16 cycles
```

---

## 文件结构

```
jolt-inlines/blake3/src/
├── lib.rs              # FUNCT3 常量定义
├── sequence_builder.rs # 指令序列生成 (配对访存)
├── host.rs             # 指令注册
├── sdk.rs              # SDK API 实现
└── exec.rs             # 参考实现

examples/hash-bench/guest/src/
├── lib.rs              # 性能测试
└── merkle.rs           # Merkle Tree 实现示例
```

---

## 测试

```bash
# 运行所有 Blake3 测试
cargo test -p jolt-inlines-blake3 --features host

# 运行性能测试
cd examples/hash-bench && RUST_LOG=info cargo run --release
```

---

## 使用示例

### Merkle Tree 构建

```rust
use blake3_inline::{blake3_parent64_compress, blake3_parent64_root_compress};

/// 计算 parent CV (中间节点)
fn parent_cv(left: &[u32; 8], right: &[u32; 8]) -> [u32; 8] {
    let mut output = [0u32; 8];
    unsafe {
        blake3_parent64_compress(left.as_ptr(), right.as_ptr(), output.as_mut_ptr());
    }
    output
}

/// 计算 root hash (根节点)
fn root_output(left: &[u32; 8], right: &[u32; 8]) -> [u8; 32] {
    let mut output = [0u32; 8];
    unsafe {
        blake3_parent64_root_compress(left.as_ptr(), right.as_ptr(), output.as_mut_ptr());
    }
    unsafe { core::mem::transmute(output) }
}
```

### 注意事项

叶子节点处理：
- 输入数据已经是 Blake2b 哈希（32 bytes）
- 直接将 `[u8; 32]` 转换为 `[u32; 8]` 作为 CV
- **不需要**额外的 BLAKE3 压缩指令
