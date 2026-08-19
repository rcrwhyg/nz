# 库能力 spec

- 模块：pkt / dhcp4
- 对照：`modules/dhcp4/`；工具 171、179
- 状态：draft

## 能力

DHCPv4 over Ethernet/UDP（67/68）编解码与客户端状态机：

1. **报文**：BOOTREQUEST/REPLY、`dhcp4hdr` + options（msgtype、clientid、parameter request list 等）；array+dump 显示。
2. **sniff+spoof 通道**：filter `udp and (port 67 or port 68)`；`snispo_init_eth`。
3. **171 DISCOVER 流程**：DISCOVER（广播）→ **首个匹配 OFFER 即接受**（不选最优）→ REQUEST → ACK → 租约续期 REQUEST（间隔 `ipLeaseTime/3`，缺省 30s）→ 用户按 `q` RELEASE。
4. **179 INFORM**：带已有 `--ip-src` 发 INFORM → 等 ACK（无租约续期、无 stdin 循环）。
5. **clientid**：type 1，id 为 client MAC。

默认测试用**假 sniff/spoof 通道**，注入 OFFER/ACK，不对公网。

## Rust 形状

`Dhcp4Packet` encode/decode + `DhcpClientDiscover` + `DhcpClientInform`。

## 非目标

- 不做 DHCPv6
- 171 不改成「选最优 OFFER」
- 不对未授权 LAN 发 DISCOVER

## 验收

- [ ] `dhcp4_discover_encode_broadcast`
- [ ] `dhcp4_first_offer_accepted`
- [ ] `dhcp4_inform_requires_client_ip`
- [ ] `dhcp4_extend_lease_third_of_leasetime`
- [ ] `dhcp4_release_on_q_key`

## 覆盖率

库代码目标 ≥ 95%。缺口：真网卡 sniff/spoof 在 CI 不测。

## 依赖

[err.md](err.md)、[pkt.md](pkt.md)、[net-sniff.md](net-sniff.md)、[net-spoof.md](net-spoof.md)、[net-addr.md](net-addr.md)
