# 库能力 spec

- 模块：dat / 密码学哈希
- 对照：`netwox-src/src/modules/crypto/`（工具 22/27/219）
- 状态：draft

## 能力

对字节或文件计算摘要（只算不能反解）：md2、md4、md5、ripemd128、ripemd160、sha1、sha224、sha256、sha384、sha512。输出小写十六进制。弱算法仍要能对照原实现向量（工具 22/27/219；工具 127 口令派生也用 MD5）。实现可用 Rust 生态 crate（实现闸再选，本闸不加依赖）。

## Rust 形状

`enum HashAlg` + `fn hash(alg, &[u8]) -> String` + 流式读文件。禁止 `netwox_*`。

## 非目标

- 不在本 spec 做 HMAC/DES（除非日后 SMB 需要再开 spec）
- 不把弱哈希宣传为安全

## 验收

- [ ] `hash_md5_empty`：空输入 MD5 为 `d41d8cd98f00b204e9800998ecf8427e`
- [ ] `hash_sha256_abc`：`abc` 的 SHA-256 符合 FIPS 向量
- [ ] `hash_md2_and_ripemd_nonempty`：md2 与 ripemd160 对固定输入得到非空且稳定摘要
- [ ] `hash_file_matches_bytes`：文件内容与内存字节同一算法结果相同

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：（实现时填；若某算法仅封装 crate，测封装层）

## 依赖

[err.md](err.md)、[dat.md](dat.md)
