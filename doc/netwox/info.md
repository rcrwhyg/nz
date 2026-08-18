# 信息 / 本地工具

相位：`spec/_index.md` 第 3 项。金标准：`src/netwox-doc_html/tools/N.html`。
布尔参数普遍是 `-x|+x|--no-x` 三元组；未指定时「显示类」开关常表示**全开**。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名，实现 CLI 闸再钉死。

## 1 — Display network configuration

建议名：`net-conf`
Usage：`nz 1 [-d|+d] [-i|+i] [-a|+a] [-r|+r]`

| 参数 | 含义 |
|------|------|
| `-d/--devices` | 网卡：编号、易记名、Ethernet 或硬件类型、MTU、真实设备名 |
| `-i/--ip` | 地址：所属网卡、IP、掩码、是否 PPP、对端 |
| `-a/--arpcache` | IPv4 ARP / IPv6 neighbor：网卡、Eth、IP |
| `-r/--routes` | 路由：网卡、目的/掩码、源（或 local）、网关、metric |

无开关则四块全打印。完整配置可能需要管理员权限。
例：`nz 1`
GUI：Local_info 字段语义对齐本工具；启动配置走工具 0 `-c`。

## 2 — Display debugging information

建议名：`debug-info`
Usage：`nz 2`（无参数）
打印内部定义值、网络配置如何取得。报 bug 时附上。可能需管理员权限。

## 3 — Display information about an IP address or a hostname

建议名：`host-info`
Usage：`nz 3 -q hostname [-t|+t] [-i|+i] [-h|+h] [-H|+H] [-e|+e] [-a|+a]`

| 参数 | 含义 |
|------|------|
| `-q/--query` | IP 或主机名（必填） |
| `-t/--title` | 每行加标题 |
| `-i/--ip` | IP |
| `-h/--host` | 主主机名 |
| `-H/--hosts` | 全部主机名 |
| `-e/--eth` | Ethernet（解析时可能 sniff/spoof，需权限） |
| `-a/--all` | 对地址列表/网段逐个显示（如 `192.168.0.0/24`） |

ip/host/hosts/eth 全关则全开。
例：`nz 3 -q www.example.com`
GUI：Remote_info。禁止对未授权公网做集成测试；例题主机名只作文档。

## 4 — Display information about an Ethernet address

建议名：`eth-info`
Usage：`nz 4 -e eth [-t|+t] [-i|+i] [-h|+h] [-H|+H]`
由 MAC 反查 IP / 主机名。ip/host/hosts 全关则全开。
例：`nz 4 -e E0:69:95:6F:ED:9A`

## 5 — Obtain Ethernet addresses of computers in an IP list

建议名：`arp-scan`（能力名，不是审计后置工具）
Usage：`nz 5 [-u|+u] -i ips`
对列表/网段发 ARP 或 IPv6 Neighbor Discovery。`--no-unresolved` 隐藏未解析项。需 sniff/spoof 权限。
例：`nz 5 -i 192.168.1.0/24`
仅本机/实验室/书面授权目标。

## 6 — Display how to reach an IP address

建议名：`route-to`
Usage：`nz 6 [-t|+t] [-d|+d] [-i|+i] [-I|+I] [-e|+e] [-E|+E] -q ip`
给出到达该 IP 的网卡、源/目的 IP、源/目的 Ethernet（跨网段时目的 Eth 是网关）。解析 Eth 可能需权限。
例：`nz 6 -q 192.168.100.200`

## 12 — Display which values to use for netwox parameters

建议名：`param-values`
Usage：`nz 12`
列出难猜枚举（如 `recordencode`：hexa、mixed…）。CLI 用户手册；GUI 用控件枚举替代。

## 13 — Obtain DLT type for sniff and spoof for each device

建议名：`dlt-info`
Usage：`nz 13`
是否支持 raw IPv4/IPv6 spoof；每块网卡 sniff/spoof 及 DLT。可能需权限。

## 169 — Display simple network configuration easy to parse

建议名：`net-conf-simple`
Usage：`nz 169`
每行三列：设备名、IP、Ethernet（非以太则为 `notether`）。比工具 1 更好解析。可能需权限。

## 187 — Display date and time

建议名：`datetime`
Usage：`nz 187 [-t uint32] [-r|+r] [-u|+u] [-h|+h] [-s|+s] [-n|+n]`
`--time` 缺省为当前。`--disp-rfc822` 默认开。其它格式：unix date(1)、人类可读、秒、秒+纳秒。

## 213 — Display a list of IP addresses

建议名：`ip-list`
Usage：`nz 213 -i ips [-s data]`
展开范围/列表，用 `--separator` 连接。给其它工具供数。另见工具 24。
例：`--ips 1.2.3.4-1.2.3.6 --separator ","` → `1.2.3.4,1.2.3.5,1.2.3.6`

## 218 — Netwox internal validation suite

建议名：不作为用户工具暴露。
原工具跑 netwox 内部自测。nz 用 `cargo nextest` / 库单测替代，**不复刻为 `nz 218`**，除非实现闸另拍板。清单保留以免「漏工具」误报。

## 共同非目标

- 不在本族实现 sniff/spoof 工具本体（7–11、32+）
- 不对公网跑工具 3/5 的自动化
- 输出排版可现代化，字段必须可映射
