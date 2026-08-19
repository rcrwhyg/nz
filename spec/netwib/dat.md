# 库能力 spec

- 模块：dat 子集（buf、编解码、checksum）
- 对照：`netwib-doc_html/netwib/dat.html` + `dat/{buf,bufenc,bufdec,checksum}.h`
- 状态：draft

## 能力

相位 1 只要工具与 pkt 立刻依赖的部分：

1. **缓冲**：可变字节缓冲，可读可写。
2. **编码写出**（工具 12 / 屏幕 / record 文本）：至少 CLI 表单那组别名——`data`、`hexa`、`mixed`、`base64`、`text`、`nothing`、`synth`、`dump`、`array`、以及 `*_wrap`、`mixedh_wrap`、`lowercase`、`uppercase`。`hexa`=`hexa1`（字节间空格），`mixed`=`mixed1`，`array`=`array8`。
3. **解码读入**：`data`、`hexa`、`mixed`、`base64`。mixed：十六进制无 `0x` 前缀；文本用单引号；`''` 表示一个 `'`。例：`'hello' 09 'bob'`、`'a''b'` → `a'b`。非法 hex/mixed/base64 归参数错误（见 err）。
4. **TCP/IP checksum**：Internet 一补码，结果为**主机序** `u16`（对照 `checksum.h`）。支持整块计算与分段 update/close，供 IP/TCP/UDP 伪头使用。空缓冲对照实现为 `0xFFFF`。

## Rust 形状

缓冲：`Vec<u8>` 或等价（不暴露 C buf 窗）。编码：`enum Encode` + `fn encode(&[u8], Encode) -> String`（或写到 `Vec<u8>`）。解码：`fn decode(&[u8], Decode) -> Result<Vec<u8>>`。checksum：`fn checksum(data: &[u8]) -> u16` 与 hasher 式 API。禁止 `netwib_*`。实现闸再选是否引入小 crate（本闸不加依赖）。

## 非目标

- C 字符串、裸指针、bufpool、fmt/sprintf、uint64 仿真
- ring/hash/array 独立类型（用标准库）
- regexp、TLV：后续 dat spec
- quoted 编解码（C 主要用于参数解析，归 CLI）
- 不在本 spec 实现文件哈希算法（见 [hash.md](hash.md)）

## 验收

- [ ] `buf_roundtrip_bytes`：写入再读出与原字节相等
- [ ] `encode_hexa1_spaces`：`[0x01,0x02]` → `01 02` 一类
- [ ] `decode_mixed_quoted_and_hex`：`'AB' 00` → `b"AB\x00"`；`'a''b'` → `b"a'b"`
- [ ] `decode_bad_hex_is_param_error`：奇数长度或非法字符失败
- [ ] `checksum_empty_is_ffff`：空缓冲结果为 `0xFFFF`
- [ ] `checksum_incremental_matches_oneshot`：分段 update 与一次计算相同

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：（实现时填；wrap/array 若分期，在本 spec 列出未做别名）

## 依赖

[err.md](err.md)
