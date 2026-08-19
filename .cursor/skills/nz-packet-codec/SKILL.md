---
name: nz-packet-codec
description: >-
  Encodes and decodes Ethernet, IPv4/IPv6, ARP, UDP, TCP, ICMP with checksum
  auto-compute vs literal advanced fields. Use when implementing pkt, spoof
  tools 32-48/140-147, or checksum recompute (tool 17/11).
---

# 报文编解码

对照：`doc/netwib/modules.md` pkt/shw、`doc/netwox/spoof.md`。实现进 `nz-net`，工具只填字段。

## 层

Link（以太等）→ IPv4/IPv6（含 options / ext）→ ARP → UDP/TCP（含 TCP options）→ ICMPv4/ICMPv6。
展示用 shw：先解码再格式化，允许现代化排版。

## 校验和

- 未指定 ihl / totlen / 各层 checksum / TCP doff：组包时**自动计算**。
- 用户一旦指定这些「高级」字段：按字面写入、不再自动算，并警告非法包。
- `ip4-id`、`tcp-seqnum`：未指定则随机。
- 工具 17：重算失败则**原包原样写出**。

## spoofip（IP 发送路径）

缺省 `best`（=`linkraw`）。取值与别名见 `doc/netwox/spoof.md`。链路工具要求 DLT=`ether`。仅授权目标。

## 测试

用构造字节 + 已知向量，不打真网。覆盖率目标见库 spec（≥ 95%）。
