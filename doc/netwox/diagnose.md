# ping / traceroute / scan

相位：`spec/_index.md` 第 8 项。金标准：`src/netwox-doc_html/tools/N.html`，默认值以 `000NNN.c` 为准。
布尔仍是 `-x|+x|--no-x`。`--spoofip` 取值见 [spoof.md](spoof.md)，缺省 `best`。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名，实现 CLI 闸再钉死。

全部需要 sniff+spoof 权限。仅本机/实验室/书面授权目标。CI 用回放/假接口，不对公网。

偶数号（50、52…）是 **EthIP spoof**：显式伪造 `--device`、`--src-eth`、`--dst-eth`（扫描变体常无目的 MAC）、`--src-ip`，不再用 `--spoofip`。

## ping（单主机）

探测一包、等应答。共用：`--dst-ip`（必填）、`--max-count`（缺省 4294967295）、`--max-ms`（缺省 1000）、`--beep`（通了响铃，默认关）、`--display01`（只打 `0`/`1`，默认关）。

| 号 | 标题 | 建议名 | 探针 |
|----|------|--------|------|
| 49 | Ping ICMP | `ping-icmp` | ICMP Echo Request → Echo Reply |
| 50 | Ping ICMP (EthIP spoof) | `ping-icmp-eth` | 同上 + 伪造 Eth/IP |
| 51 | Ping TCP | `ping-tcp` | TCP SYN；开→SYN-ACK，关→RST。`--port` |
| 52 | Ping TCP (EthIp spoof) | `ping-tcp-eth` | 同上 |
| 53 | Ping UDP | `ping-udp` | UDP；**关**口才回 ICMP 错，开则无回。`--port` |
| 54 | Ping UDP (EthIp spoof) | `ping-udp-eth` | 同上 |
| 55 | Ping ARP | `ping-arp` | ARP Request → Reply（无 `--spoofip`） |
| 56 | Ping ARP (EthIp spoof) | `ping-arp-eth` | 同上 |
| 148 | Ping ICMP6 Neighbor Discovery | `ping-nsolic` | IPv6 Neighbor Solicitation → Advertisement |
| 149 | Ping ICMP6 Neighbor Discovery (EthIp spoof) | `ping-nsolic-eth` | 同上 |

148/149 的 `--dst-ip` 是 IPv6。例：`nz 49 --dst-ip 192.0.2.1`（文档地址）。

## traceroute

TTL 从 `--min-ttl`（缺省 1）增到 `--max-ttl`（缺省 30）；路由器 TTL=0 回 ICMP Time Exceeded。共用：`--dst-ip`、`--max-ms`（1000）、`--resolve`（反查主机名，**默认关**）。

| 号 | 标题 | 建议名 | 探针 |
|----|------|--------|------|
| 57 | Traceroute ICMP | `trace-icmp` | ICMP Echo |
| 58 | Traceroute ICMP (EthIP spoof) | `trace-icmp-eth` | 同上 |
| 59 | Traceroute TCP | `trace-tcp` | TCP；`--port` |
| 60 | Traceroute TCP (EthIp spoof) | `trace-tcp-eth` | 同上 |
| 61 | Traceroute UDP | `trace-udp` | UDP；`--port` |
| 62 | Traceroute UDP (EthIp spoof) | `trace-udp-eth` | 同上 |
| 63 | Traceroute on a specified IP protocol | `trace-ipproto` | 仅 IP；`--protocol`、`--data`（mixed）。**无法自动判断结束，需用户中断** |
| 64 | Traceroute on a specified IP protocol (EthIp spoof) | `trace-ipproto-eth` | 同上 |

## scan（多目标）

`--ips` 列表/网段（必填）。TCP/UDP 另需 `--ports`。共用：`--min-ms`（包间隔，缺省 0）、`--max-ms`（等应答，缺省 5000）、`--disp-useful`（只打有用行，默认关）、`--numtargets`（并发，缺省 20）。

例外：**72 与 151 没有 `--numtargets`**。EthIP 扫描变体一般无 `--dst-eth`。

| 号 | 标题 | 建议名 | 探针 |
|----|------|--------|------|
| 65 | Scan ICMP | `scan-icmp` | Echo |
| 66 | Scan ICMP (EthIP spoof) | `scan-icmp-eth` | 同上 |
| 67 | Scan TCP | `scan-tcp` | SYN；开/关判定同 ping TCP |
| 68 | Scan TCP (EthIp spoof) | `scan-tcp-eth` | 同上 |
| 69 | Scan UDP | `scan-udp` | 同 ping UDP（开口可能无回包） |
| 70 | Scan UDP (EthIp spoof) | `scan-udp-eth` | 同上 |
| 71 | Scan ARP | `scan-arp` | ARP（无 `--spoofip`） |
| 72 | Scan ARP (EthIp spoof) | `scan-arp-eth` | 同上，无并发参数 |
| 150 | Scan ICMP6 Neighbor Discovery | `scan-nsolic` | Neighbor Solicitation |
| 151 | Scan ICMP6 Neighbor Discovery (EthIp spoof) | `scan-nsolic-eth` | 同上，无并发参数 |

例：`nz 65 --ips 192.0.2.0/24`

## 214 / 215 — traceroute 拓扑图

建议名：`trace-map` / `trace-map-eth`
Usage：`nz 214 -i ips [-p ports] [-P ports] [-c|+c] [-s spoofip] [-T uint32] [-t uint32] [-m uint32] [-r|+r] [-v|+v]`
215 用 EthIP 字段替换 `--spoofip`。

对网段做 traceroute，画远端拓扑。正确设 min/max TTL 能加快。`--icmp` **默认开**。`--verbose` 打中间结果，默认关。

| 参数 | 含义 |
|------|------|
| `-i/--ips` | 目标列表/范围（例文档用 `all`） |
| `-p/--tcpports` | 认为**开**的 TCP 口 |
| `-P/--udpports` | 认为**关**的 UDP 口（才能收到 ICMP 错） |
| `-c/--icmp` | 是否也扫 ICMP；默认开 |
| `-T/--min-ttl` | 缺省 1 |
| `-t/--max-ttl` | 缺省 30 |
| `-m/--max-ms` | 缺省 1000 |
| `-r/--resolve` | 反查主机名 |
| `-v/--verbose` | 中间结果 |

TCP 口缺省例：21,22,23,25,53,79,80,88,110,113,119,139,143,389,443,445,1080,2401,6000。
UDP 口缺省例：1,53,67,68,123,137,138,161,162,177,514。

## 非目标

- 不把本族做成未授权扫描器；后置审计工具另见 `spec/netaudit/_deferred.md`
- UDP「开」可能无应答，不是实现 bug
- 63/64 必须能被中断结束
- 输出排版可现代化，0/1、有用行、拓扑图信息面必须可映射
