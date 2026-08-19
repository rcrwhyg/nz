# ping / traceroute / scan 共用

本族需 sniff+spoof。仅本机 / 实验室 / 书面授权。CI 假通道 + 注入应答，不对公网。`--spoofip` 取值见 [spoof/_common.md](../spoof/_common.md)，缺省 `best`。短选项以各工具 Usage 为准；测试用长名。非 EthIP 工具用本机路由得出网卡与源 IP（[net-conf.md](../../netwib/net-conf.md)）。

偶数号是 **EthIP spoof**：不用 `--spoofip`。HTML 示例 `0:a:a:a:a:a` / `1.2.3.4` **不是自动填入**。运行时若未显式给出则失败（`device/src-eth/src-ip must be set`；traceroute 与 ping ICMP/TCP/UDP 还要 `dst-eth`）。ARP / ICMPv6 邻居发现 EthIP **ping/scan 无 `--dst-eth`**。

## ping（单主机）

`--dst-ip` 必填。循环：发探针 → 等最多 `--max-ms`（默认 1000）→ 匹配则普通模式打印 `Ok`（`--beep` 则响铃）。`--max-count` 默认 `4294967295`；**CLI `0` 为无限**（对照 ping.h）。`--display01` 开时强制只发 1 次，结束打 `1` 或 `0`。Ctrl-C 可中断。

| 探针 | 成功匹配 |
|------|----------|
| ICMP Echo | Echo Reply |
| TCP SYN | SYN+ACK（开）或 RST+ACK（关，也算 reached）。EthIP 收到 SYN+ACK 后再发 RST，避免 SYN 洪泛 |
| UDP | 仅 **端口不可达** ICMP（开口通常无回包，不是实现 bug） |
| ARP | ARP Reply |
| ICMP6 NS | Neighbor Advertisement |

TCP/UDP `--dst-port` 默认 80。55/148 的 count/ms/beep/display01 是普通选项；49/51/53 等带 `--spoofip` 的为 Advanced。

## traceroute

TTL 从 `--min-ttl`（1）增到 `--max-ttl`（30）。路由器 TTL=0 回 Time Exceeded。`--max-ms` 1000。`--resolve` 反查主机名，**默认关**。63/64 只发指定 IP 协议（`--protocol` 默认 1=ICMPv4）+ `--ip4-data`，**无法自动判断结束**，必须能被中断。

## scan（多目标）

`--ips` 必填（语法见 [net-addr.md](../../netwib/net-addr.md)）。TCP/UDP 另需 `--ports`。`--min-ms` 包间隔默认 0；`--max-ms` 等应答默认 5000；`--numtargets` 并发默认 20，**大于 200 则钳到 200**。72 与 151 **无** `--numtargets` CLI，内部仍用 20。`--disp-useful` 默认关：开则只打「有用」行。

`--disp-useful` 开时仍打印：ICMP `reached`；TCP `open`；UDP `timeout (perhaps open)`；ARP/NS `{ip} : {eth}`。不打印：TCP/ICMP/ARP 的 closed/unreached/timeout（UDP closed 也不打）。

## 214 / 215

对 `--ips` 做 traceroute 拓扑。`--tcpports` 视为开的 TCP 口；`--udpports` 视为关的 UDP 口。`--icmp` **默认开**。`--verbose` 默认关。`--ips` 示例 `all`。
