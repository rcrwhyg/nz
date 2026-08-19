# 库能力 spec

- 模块：pkt（分层编解码）
- 对照：`netwib-doc_html/netwib/pkt.html` + `pkt/*.h`
- 状态：draft

## 能力

对字节缓冲做分层构造与解码（工具 spoof/sniff/record 共用，禁止每工具一套）。

层：

- Link：至少 Ethernet（src/dst/type）；其它 DLT 能当不透明载荷或后续扩展
- IPv4：tos、id、标志、片偏移、ttl、protocol、src/dst、options；ihl/totlen/checksum 默认自动
- IPv6：traffic class、flow label、next header、hop limit、src/dst、extensions；payload length 默认自动
- ARP/RARP：op 与各方 Eth/IP
- UDP：sport/dport/len/checksum（默认自动，含伪头）
- TCP：端口、seq/ack、标志、window、urgent、options、doff/checksum 默认自动
- ICMPv4 / ICMPv6（含 Neighbor Solicitation/Advertisement 等邻居报文）

另：

- **layer / packet**：按 DLT 从链路或 IP 起解。组包时自动填：以太网 type、IP protocol、IPv4 ihl/totlen/checksum、IPv6 payload length、UDP len/checksum、TCP doff/checksum、ICMP checksum。
- **conv**：DLT 之间转换（工具 16）。缺字段策略对照 `newfield`：blank（不填）/ fill（计算）/ fillblank（能算则算，否则空白）；默认 fillblank。
- **ipfrag**：IPv4/IPv6 **分片**（对照 `ipfrag.h`）。片载荷大小向上取整到 8 字节；首片可因 options/ext 更大；0 表示不限制。**重组**在对照库不在 `ipfrag` 里，但工具 7 `--ipreas` 与工具 18 需要，放进本 spec 以免每工具一套；重组允许丢包。

校验和策略见 `nz-packet-codec`：未指定高级字段则算；指定则字面写入（不必先组包再改字节）。`ip4-id`、TCP seq 未指定则随机（随机可注入 RNG 以便测）。

IPv4 options / IPv6 ext / TCP options：长度须 4 字节对齐；IPv4/TCP option 块过大归参数错误。

## Rust 形状

每层一个结构 + `encode`/`decode`；packet 是有序层或枚举树。输入输出 `&[u8]` / `Vec<u8>`。禁止 `netwib_*`。本闸不加入 pcap crate。

## 非目标

- 不是 Wireshark 式协议树；DNS/DHCP 等「extended」解码可后置到 shw/应用 spec
- 不在本 spec 做 IO（sniff/spoof）
- 不对公网发包

## 验收

- [ ] `eth_ip4_udp_roundtrip`：最小 Ethernet+IPv4+UDP 编解码字节一致（自动 checksum）
- [ ] `ipv4_literal_checksum_not_recomputed`：用户给定 checksum 则输出中保持该值
- [ ] `tcp_syn_flags`：只置 SYN 时标志位正确
- [ ] `arp_request_fields`：op=1 与四方地址
- [ ] `icmp6_neighbor_solicit_decode`：能认出邻居请求类报文（不要求实现全部 ICMP 子类型）
- [ ] `conv_ether_to_raw_strips_l2`：ether→raw 去掉以太网头
- [ ] `ipfrag_split_rounds_to_8`：请求片大小 1 时按 8 字节片切
- [ ] `ipfrag_reassemble_two_frags`：两片能拼回；缺片时行为有文档（丢或错误）

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：稀有 DLT、全部 ICMP 码点可分期，须在本 spec 列出未做类型。

## 依赖

[err.md](err.md)、[dat.md](dat.md)（checksum 与 mixed 载荷）、[net-addr.md](net-addr.md)
