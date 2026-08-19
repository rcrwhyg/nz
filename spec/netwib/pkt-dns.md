# 库能力 spec

- 模块：pkt / dns
- 对照：`netwib-doc_html/` DNS 段 + `modules/dns/`；工具 102–105
- 状态：draft

## 能力

DNS 报文编解码与查询/固定应答，供 102–105 共用：

1. **question**：组 question 段（`dnshdr` + name/type/class）；PTR 且 `--name` 可解析为 IP 时自动转 `in-addr.arpa` / `ip6.int`（`dns_inaddrarpa`）。
2. **query**：UDP/TCP 发 question、等应答（按 DNS ID 匹配）；超时返回 `NETWOX_ERR_TIMEOUT`；应答 array+dump 显示（工具 102）或回调解析（103）。
3. **fixed answer**（`dnspktex_answer`）：对 IN class、QUERY opcode 的 A/NS/PTR/MX 用固定 hostname/IP/authns 构造应答；A 用 `--hostnameip`、PTR 用 `--hostname`、MX preference 固定 10 且 exchange=`--hostname`；其余 type 或 class 非 IN 回 NOTIMP（104）或 ignore（105，`ignoreunimplemented=true`）。
4. **display**：DNS 包 array+dump（与工具 12 encode 无关）。

默认测试用**假 IO**：注入/捕获 DNS 字节，不打真 resolver。真路径 `privileged-tests`；CI 不对公网。

## Rust 形状

`DnsQuestion` + `encode_question` / `decode_packet` + `query_udp` / `query_tcp` + `build_fixed_answer`。禁止 `netwib_*`。本闸不加运行时依赖。

## 非目标

- 不做 DNSSEC（进 `spec/modern/`）
- 不做完整递归 resolver
- 不为每个工具复制编解码

## 验收

- [ ] `dns_ptr_inaddrarpa_from_ip4`
- [ ] `dns_query_matches_id_udp`
- [ ] `dns_fixed_answer_a_uses_hostnameip`
- [ ] `dns_fixed_answer_ptr_uses_hostname`
- [ ] `dns_fixed_answer_mx_pref_10`
- [ ] `dns_unimplemented_class_ch_not_in_fixed`

## 覆盖率

库代码目标 ≥ 95%。缺口：真 UDP/TCP 53 在 CI 不测，仅假 IO 计入。

## 依赖

[err.md](err.md)、[dat.md](dat.md)、[pkt.md](pkt.md)、[net-addr.md](net-addr.md)
