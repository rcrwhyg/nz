---
name: nz-netwib-map
description: >-
  Maps netwib err/dat/sys/net/pkt/shw capabilities onto nz-net without cloning
  the C API. Use when designing library types, porting a netwib module, or when
  a tool would otherwise grow a private codec.
---

# netwib → nz-net

对照：`doc/netwib/modules.md`。库 spec 用 `spec/TEMPLATE-lib.md`。

## 模块

| 原模块 | 能力（不是函数名） |
|--------|-------------------|
| err | 统一错误；分区含义见 modules.md |
| dat | buf、编解码、checksum、regexp、TLV |
| sys | 时间、路径/文件、IO、record |
| net | 设备、地址集合、conf、sock、sniff、spoof |
| pkt | 各层头、layer/packet、conv、分片重组 |
| shw | 结构化展示，不是嗅探 GUI |

HTML 入口常写成 5 节；`err` 仍要覆盖。

## Rust 形状

- 公共 API 用惯用 Rust 类型（`IpAddr`、自有 newtype），禁止 `netwib_*` 符号与 C ABI。
- 线程/锁用 `std`；不要把原 mutex 模型暴露出去。
- `FILE*`、Windows/Unix 特化不进公共语义，用 `cfg`。

## 禁止

- 为单个工具复制编解码；工具只组包/调 IO。
- 把对照源 C 贴进 crate。
- 未到第 4 闸就加运行时依赖（本闸写 skill/spec 即可）。
