# 库能力 spec

- 模块：net / sock
- 对照：`netwib-doc_html/` socket 段 + `net/sock*.h`；netwox `modules/sock.c`
- 状态：draft

## 能力

打开 TCP/UDP 客户、单客户服、多客户服通道。三条初始化路径（对照 `netwox_sockinit`）：

1. **REALEASY**（默认）：系统套接字，不填 `--src-ip` 时服务端用 `--iptype`（缺省 `ip4`）+ `*_easy` 绑定。
2. **REAL**：显式 `--src-ip` 和/或 `--ip4opts` / `--ip6exts` 时用 `*_full`。
3. **VIRT**：客户侧给出 `--device` / `--src-eth` / `--dst-eth` 任一，或服务端给出 `--device` / `--src-eth` 时，走 sniff+spoof 虚拟套接字（`sockv_*`）。

客户 `--src-port` 为 `0` 时系统分配；虚拟客户为 `0` 时在 1024–65535 随机。TCP 多客户服（`TCP_MULSER`）不走 `sock_init`，单独 `tcp_mulser` / `sockv_tcp_mulser`。

默认测试用**假套接字**：记录 connect/bind/read/write/close，不打真网。真路径 `privileged-tests`；CI 不对公网、不依赖 root。

## Rust 形状

`SockInfo`（类型、init 路径、地址、opts）+ `open_client` / `open_server` / `accept_mulser` + `Io` trait（read/write/wait）。禁止 `netwib_*`。本闸不加运行时依赖。

## 非目标

- 不在本 spec 定义 perf 消息格式（见 [transport/_common.md](../netwox/transport/_common.md)）
- 不为每个工具复制 keyboard 循环
- 不对公网连未授权目标

## 验收

- [ ] `sock_tcp_cli_fake_connects`：假通道记下目的 IP/端口
- [ ] `sock_server_realeasy_ip4`：未给 `--src-ip` + 缺省 `--iptype` 走 ip4 easy
- [ ] `sock_virt_when_device_set`：客户给 `--device` 时 init 路径为 VIRT
- [ ] `sock_src_port_zero_system`：REAL 客户 `--src-port 0` 由假后端分配
- [ ] `sock_real_ignored_in_ci`：真套接字路径 feature/忽略

## 覆盖率

库代码目标 ≥ 95%。缺口：真 OS 套接字在 CI 可能测不到，缺口写「仅假后端计入覆盖」。

## 依赖

[err.md](err.md)、[net-addr.md](net-addr.md)、[net-device.md](net-device.md)、[net-sniff.md](net-sniff.md)、[net-spoof.md](net-spoof.md)、[pkt.md](pkt.md)
