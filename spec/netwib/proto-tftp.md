# 库能力 spec

- 模块：net / tftp
- 对照：`modules/tftp/`；工具 165–167、176
- 状态：draft

## 能力

UDP TFTP（RFC 1350 子集，对齐 netwox）：

1. **客户端**：UDP 69；`--mode` 缺省 `octet`（或 `netascii`）；`--timeout` 缺省 **10000 ms**（**176 为 60000**）；`--retry` 缺省 **3**。
2. **get/put**：RRQ/WRQ + DATA/ACK 块传输；176 get 后比 MD5。
3. **服务端**（167）：UDP mulser 69；`--allow-get`/`--allow-put` **C 默认开**；`--allowed-clients` 缺省 all；`--rootdir` 可选。

默认测试用**假 TFTP 通道**。

## Rust 形状

`TftpClient` + `get/put` + `TftpServer::serve_mulser` + `mode Octet|Netascii`。

## 非目标

- 不做 TFTP 选项扩展（blocksize 等）除非 C 已有
- 不对公网未授权 TFTP

## 验收

- [ ] `tftp_octet_mode_default`
- [ ] `tftp_retry_default_3`
- [ ] `tftp_get_roundtrip_fake`
- [ ] `tftp_server_allow_get_put_default_on`
- [ ] `tftp_get_md5_176_timeout_60000`

## 覆盖率

库代码目标 ≥ 95%。

## 依赖

[err.md](err.md)、[net-sock.md](net-sock.md)、[hash.md](hash.md)
