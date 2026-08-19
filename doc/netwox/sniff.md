# sniff 工具

相位：`spec/_index.md` 第 6 项。金标准：`src/netwox-doc_html/tools/N.html`，默认值与语义以 `000NNN.c` 为准。
布尔仍是 `-x|+x|--no-x`。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名，实现 CLI 闸再钉死。

实时抓包默认走 pcap/record 回放或用户态假接口验收；CI 不依赖 root、不对公网。仅本机/实验室/书面授权目标。

## 共用：设备与 BPF 过滤

`-d/--device` 选网卡（Windows 等系统上部分网卡不可 sniff）。`-f/--filter` 是 BPF/pcap 过滤。

常用原子：`host`、`net`（含 mask 与 `/prefix`）、`port`、`dst host`、`src port`、`ether host` / `ether src`（`ether a:b:c:d:e:f` **无效**）、`ip`、`arp`、`rarp`、`tcp`、`icmp`、`udp`。

例：`"host 1.2.3.4"`、`"net 192.168 and icmp"`、`"(udp or tcp) and not host 1.2.3.4"`。

record 格式与 DLT 见 [record.md](record.md)。写出 record 时 DLT：`--rawip` 则为 `raw`，否则为工具 13 给出的 sniff DLT。

## 7 — Sniff

建议名：`sniff`
Usage：`nz 7 [-d device] [-f filter] [-p|+p] [-H encode] [-D encode] [-r|+r] [-x|+x] [-i|+i] [-t|+t] [-s|+s] [-o file] [-R recordencode] [-c uint32] [-C uint32] [-Q|+Q]`

抓包：显示和/或写入 record。需 sniff 权限。工具 0 把它标为需要 stdin（`--pause` 读键）。

| 参数 | 含义 |
|------|------|
| `-d/--device` | 网卡 |
| `-f/--filter` | BPF/pcap |
| `-p/--pause` | 可暂停。`P` 切换屏幕显示（落盘不停），`Q` 退出。默认关 |
| `-H/--hdrencode` | 屏幕上头展示，缺省 `array` |
| `-D/--dataencode` | 屏幕上载荷展示，缺省 `dump` |
| `-r/--rawip` | 忽略链路层，从 IP 头开始。默认关 |
| `-x/--extended` | 尝试解码 DNS/DHCP 等；**默认开** |
| `-i/--ipreas` | IP 重组（可能丢包）。默认关 |
| `-t/--tcpreord` | TCP 按序号重排（可能丢包）。默认关 |
| `-s/--screen` | 打到屏幕；**默认开** |
| `-o/--outfile` | 写入 record；未给则不落盘 |
| `-R/--recordencode` | 落盘编码，缺省 `bin` |
| `-c/--split-size` | 单文件最大 KB；`0` = 不按大小切。默认 0 |
| `-C/--split-age` | 单文件最长秒；`0` = 不按时间切。默认 0 |
| `-Q/--losepriv` | Linux 降到 nobody；默认关 |

`--split-size` 或 `--split-age` 非 0 时，文件名在 basename 后加 `.YYYYMMDD_HHMMSS.N`（N 从 1 起）。两者都为 0 则写到 `--outfile` 原名。
例：`nz 7`

## 8 — Sniff and display open ports

建议名：`sniff-ports`
Usage：`nz 8 [-d device] [-f filter]`

从流量里抽端口。UDP：解码出即打印目的 IP/端口，不验证是否伪造。TCP：**只认 SYN-ACK**（忽略扫描），打印源 IP/源端口。实现上走 IP 层 sniff 并做 IP 重组。需 sniff 权限。

输出形态：`UDP\t{ip}\t{port}` / `TCP\t{ip}\t{port}`。
例：`nz 8`

## 9 — Sniff and display Ethernet addresses

建议名：`sniff-eth`
Usage：`nz 9 [-d device] [-f filter]`

从 ARP / RARP / IP 推 Ethernet↔IP。跨以太网 LAN 会不准。网卡 DLT 必须是 `ether`，否则打印不支持并退出。需 sniff 权限。

输出：`{eth}\t{ip}`。跳过全 0 IP、以太网广播目的、以及 `33:33:ff:*` 组播目的（IPv6 solicited-node 一类）。ARP 应答再打目的一侧；RARP 应答同理。
例：`nz 9`

## 10 — Sniff and display network statistics

建议名：`sniff-stats`
Usage：`nz 10 [-d device] [-f filter]`

七组计数：Ethernet/link、ARP、IP4、IP6、UDP、TCP、ICMP。每组：包数 `count`、字节和 `size`、包数占比 `c%`、字节占比 `s%`。最多每秒刷新一次。需 sniff 权限。
例：`nz 10`

## 11 — Sniff and verify checksums

建议名：`sniff-csum`
Usage：`nz 11 [-d device] [-f filter] [-r|+r] [-i|+i]`

校验 IP/TCP/UDP 等校验和；坏的连同期望值一起显示。`--rawip`、`--ipreas` 语义同工具 7（默认皆关）。需 sniff 权限。
例：`nz 11`

## 非目标

- 不在 CI 上打真网卡或公网
- 8 不是端口扫描器（扫描族另见 49–72）
- 9 不保证 WAN 上的 MAC/IP 对应正确
- 7/11 的重组路径允许丢包，与工具 18 同一口径
