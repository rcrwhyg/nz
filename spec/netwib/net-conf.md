# 库能力 spec

- 模块：net / 本机配置
- 对照：`netwib-doc_html/netwib/net.html` + `net/{conf,confdev,confip,confarp,confrout}.h`
- 状态：implemented

## 能力

读本机网络配置（工具 1、6、169、工具 0 `--conf`）：

1. **网卡**：同 [net-device.md](net-device.md)。
2. **IP**：所属网卡编号、地址、掩码、是否 PPP、PPP 对端。
3. **ARP / neighbor**：网卡编号、Ethernet、IP（IPv4 ARP 与 IPv6 neighbor 同一表）。
4. **路由**：网卡编号、目的/掩码、源（或 `local`）、网关、metric。
5. **到达**：给定目的 IP，得出出接口、源 IP；若还要 Ethernet，再给出源/目的 MAC（跨网段时目的 MAC 是网关）。找不到路由或非以太时，对应字段标为未解析（工具 6）。

测试用假表，不要求 CI 有真网卡或 root。

## Rust 形状

`struct IpOnDevice` / `ArpEntry` / `Route` + `fn list_*() -> Result<Vec<_>>` + `fn reach(dst: IpAddr) -> Result<Reach>`。禁止 `netwib_*`。真系统读取可 `cfg`，默认测试走假后端。

## 非目标

- 不写配置、不改路由表
- 不在本 spec 做 sniff/spoof IO（工具 13 探测 DLT 另用假通道）
- 不对公网

## 验收

- [ ] `conf_fake_lists_four_tables`：假后端至少一块 ether、一条 IP、一条 ARP、一条路由，字段齐全
- [ ] `conf_reach_local_subnet`：目的在本网段时给出设备与源 IP
- [ ] `conf_reach_via_gw_sets_dst_eth`：跨网段时目的 Eth 为网关 MAC
- [ ] `conf_reach_missing_is_unresolved`：无路由时 reach 标明未解析，不当成崩溃
- [ ] `conf_real_ignored_in_ci`：真读取用 feature 或忽略；CI 不依赖 root

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：真 conf 仅在 feature `system-inventory` 下提供桩，CI 只计假后端。

## 依赖

[err.md](err.md)、[net-device.md](net-device.md)、[net-addr.md](net-addr.md)
