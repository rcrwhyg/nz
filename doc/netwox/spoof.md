# spoof 工具

相位：`spec/_index.md` 第 7 项。金标准：`src/netwox-doc_html/tools/N.html`，默认值与「非法头字段」行为以 `000NNN.c` 为准。
布尔仍是 `-x|+x|--no-x`。字段短字母在不同工具间会平移，**以长名为准**。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名，实现 CLI 闸再钉死。

全部需要 spoof 权限。仅本机/实验室/书面授权目标。验收用假接口或离线组包对照，CI 不对公网发送。

## 两条发送路径

**链路（Ethernet）** 32–37、140–143：`--device` 选网卡，DLT 必须是 `ether`，否则失败。

**IP** 38–41、144–147、样本 42–48/192：`--spoofip` 决定链路如何生成，缺省 **`best`**。常用：`best`、`link`、`raw`。全表（工具 12）：

| 值 | 含义 |
|----|------|
| `raw` | 走系统 IP 栈；防火墙或某些系统可能失败 |
| `linkf` | 链路层（现仅 Ethernet），尝试填源 MAC |
| `linkb` | 链路层，源 MAC 留空 `0:0:0:0:0:0` |
| `linkfb` | 能填则填，否则空白 |
| `rawlinkf` / `rawlinkb` / `rawlinkfb` | 先 raw 再对应 link |
| `linkfraw` / `linkbraw` / `linkfbraw` | 先 link 再 raw |
| `link` | = `linkfb` |
| `rawlink` | = `rawlinkfb` |
| `linkraw` | = `linkfbraw` |
| `best` | = `linkraw` |

源 IP 被伪造时 `linkf` 可能填不出 MAC，改用 `linkb` / `linkfb`。

## 组包规则

头字段未给时用库的 `initdefault`。`ip4-id`、`tcp-seqnum`：**未指定则随机**。
ihl / totlen / 各层 checksum / TCP doff 等「高级」字段：未指定则自动计算；一旦用户指定，按字面写入、**不再自动算**，并警告「非法包，不要信 sniffer 漂亮显示」。

载荷是 mixed（可编辑十六进制混排）。IPv4 options / IPv6 extensions / TCP options 走对应 mixed 缓冲。

实现时各层编解码走库，禁止每个工具复制一套。

## 分层工具（32–41、140–147）

| 号 | 标题 | 建议名 | 层 |
|----|------|--------|----|
| 32 | Spoof Ethernet packet | `spoof-eth` | Eth + data |
| 33 | Spoof EthernetArp packet | `spoof-eth-arp` | Eth + ARP/RARP |
| 34 | Spoof EthernetIp4 packet | `spoof-eth-ip4` | Eth + IPv4 + data |
| 35 | Spoof EthernetIp4Udp packet | `spoof-eth-ip4-udp` | Eth + IPv4 + UDP |
| 36 | Spoof EthernetIp4Tcp packet | `spoof-eth-ip4-tcp` | Eth + IPv4 + TCP |
| 37 | Spoof EthernetIp4Icmp4 packet | `spoof-eth-ip4-icmp4` | Eth + IPv4 + ICMPv4 |
| 38 | Spoof Ip4 packet | `spoof-ip4` | IPv4 + data + `--spoofip` |
| 39 | Spoof Ip4Udp packet | `spoof-ip4-udp` | IPv4 + UDP + `--spoofip` |
| 40 | Spoof Ip4Tcp packet | `spoof-ip4-tcp` | IPv4 + TCP + `--spoofip` |
| 41 | Spoof Ip4Icmp4 packet | `spoof-ip4-icmp4` | IPv4 + ICMPv4 + `--spoofip` |
| 140 | Spoof EthernetIp6 packet | `spoof-eth-ip6` | Eth + IPv6 + data |
| 141 | Spoof EthernetIp6Udp packet | `spoof-eth-ip6-udp` | Eth + IPv6 + UDP |
| 142 | Spoof EthernetIp6Tcp packet | `spoof-eth-ip6-tcp` | Eth + IPv6 + TCP |
| 143 | Spoof EthernetIp6Icmp6 packet | `spoof-eth-ip6-icmp6` | Eth + IPv6 + ICMPv6 |
| 144 | Spoof Ip6 packet | `spoof-ip6` | IPv6 + data + `--spoofip` |
| 145 | Spoof Ip6Udp packet | `spoof-ip6-udp` | IPv6 + UDP + `--spoofip` |
| 146 | Spoof Ip6Tcp packet | `spoof-ip6-tcp` | IPv6 + TCP + `--spoofip` |
| 147 | Spoof Ip6Icmp6 packet | `spoof-ip6-icmp6` | IPv6 + ICMPv6 + `--spoofip` |

Usage 以各工具 HTML 为准；下面按**长名**列字段（短字母会变）。

### Ethernet

`--device`、`--eth-src`、`--eth-dst`、`--eth-type`（纯 Eth 工具 32 还有 `--eth-data`）。
ARP 时 `--eth-type`：ARP=2054，RARP=32821。

### ARP（仅 33）

`--arp-op`：1=ARPREQ，2=ARPREP，3=RARPREQ，4=RARPREP。
`--arp-ethsrc` / `--arp-ipsrc` / `--arp-ethdst` / `--arp-ipdst`。

### IPv4

`--ip4-tos`、`--ip4-id`（未设则随机）、`--ip4-reserved`、`--ip4-dontfrag`、`--ip4-morefrag`、`--ip4-offsetfrag`、`--ip4-ttl`、`--ip4-protocol`、`--ip4-src`、`--ip4-dst`、`--ip4-opt`、`--ip4-data`（无上层时）。
高级：`--ip4-ihl`、`--ip4-totlen`、`--ip4-checksum`。

### IPv6

`--ip6-trafficclass`、`--ip6-flowlabel`、`--ip6-protocol`（next header）、`--ip6-ttl`（hop limit）、`--ip6-src`、`--ip6-dst`、`--ip6-exts`、`--ip6-data`（无上层时）、`--ip6-payloadlength`（高级）。

### UDP

`--udp-src`、`--udp-dst`、`--udp-data`；高级 `--udp-len`、`--udp-checksum`。

### TCP

`--tcp-src`、`--tcp-dst`、`--tcp-seqnum`（未设则随机）、`--tcp-acknum`、reserved1–4、cwr、ece、urg、ack、psh、rst、syn、fin、`--tcp-window`、`--tcp-urgptr`、`--tcp-opt`、`--tcp-data`。
高级：`--tcp-doff`、`--tcp-checksum`。

### ICMP

`--icmp-type`、`--icmp-code`；高级 `--icmp-checksum`。v4 与 v6 工具都用这两名；载荷细节实现闸对照 pkt 模块。

例：`nz 32`、`nz 38 --ip4-dst 192.0.2.1`（文档地址，勿对公网发）。

## 样本工具（42–48、192）

硬编码样本：`--sample` 1=`udp_syslog`，2=`tcp_syn`，3=`tcpsynack`，4=`tcpack`，5=`ping`。缺省 1。
共用：`--ip4-src`、`--ip4-dst`、`--tcp-src`、`--tcp-dst`、`--fragsize`（0=不分片）、`--display`、`--spoofip`。

| 号 | 标题后缀 | 建议名 | 额外 |
|----|----------|--------|------|
| 42 | fragment | `spoof-sample` | 可分片 |
| 43 | fragment, ip4opt:noop | `spoof-sample-noop` | IPv4 NoOperation 选项 |
| 44 | fragment, ip4opt:rr | `spoof-sample-rr` | Record Route |
| 45 | fragment, ip4opt:lsrr | `spoof-sample-lsrr` | Loose Source Route；`--ip4opt-ips` |
| 46 | fragment, ip4opt:ts | `spoof-sample-ts` | Timestamp |
| 47 | fragment, ip4opt:ipts | `spoof-sample-ipts` | IP Timestamp |
| 48 | fragment, ip4opt:ippts | `spoof-sample-ippts` | Prespecified Timestamp；`--ip4opt-ip` |
| 192 | fragment, ip4opt:ssrr | `spoof-sample-ssrr` | Strict Source Route；`--ip4opt-ips` |

例：`nz 42 --sample 2 --fragsize 8`

## 非目标

- 不对未授权网络发送
- 不为每个分层工具复制编解码；非法头字段是可选能力，不是默认路径
- 样本 1–5 的字节级内容实现闸对照源码钉死，本摘录只钉样本号与选项种类
