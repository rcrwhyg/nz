# 清单

来源：`toollist.txt`、`tooltree.h`、netwib 头文件、netwag lessons。后置工具标 **DEFERRED**。

合计：库 6 模块；CLI 工具 0 + 1–223（其中后置 73–86、98、101、130–132 共 20 个）；GUI 功能面 8 块。

## netwib 模块（等价能力，非 C API）

| 模块 | 能力（摘自头文件结构） |
|------|------------------------|
| err | 统一错误码 |
| dat | buf、编解码、fmt、checksum、regexp、ring/hash、TLV、array、uint64 |
| sys | 时间、随机、路径/文件、线程同步、IO、键盘/屏幕、record、init |
| net | device、IP/Eth/port 及集合、conf（设备/IP/ARP/路由）、sock、sniff、spoof |
| pkt | Link、IPv4/6、ARP、UDP、TCP、ICMP4/6、layer/packet、conv、ipfrag |
| shw | 报文展示 |

HTML 入口：`netw-ib-ox-ag-5.39.0/src/netwib-doc_html/`。

## 工具 0（GUI 契约，在范围）

| 号 | 标题 | 说明 |
|----|------|------|
| 0 | Obtain information needed by netwag | `--tools` `-h` `-f` `-r` `-R` `-k` `-e` `-c` `-v`；附带 `-b`/`-u`；非交互用户工具。详见 [netwox/tool0.md](netwox/tool0.md) |

## netwox 1–223

实现相位见 `spec/_index.md`。HTML：`netw-ib-ox-ag-5.39.0/src/netwox-doc_html/tools/N.html`。

### 信息 / 本地

| 号 | 标题 |
|----|------|
| 1 | Display network configuration |
| 2 | Display debugging information |
| 3 | Display information about an IP address or a hostname |
| 4 | Display information about an Ethernet address |
| 5 | Obtain Ethernet addresses of computers in an IP list |
| 6 | Display how to reach an IP address |
| 12 | Display which values to use for netwox parameters |
| 13 | Obtain DLT type for sniff and spoof for each device |
| 169 | Display simple network configuration easy to parse |
| 187 | Display date and time |
| 213 | Display a list of IP addresses |
| 218 | Netwox internal validation suite |

### sniff

| 号 | 标题 |
|----|------|
| 7 | Sniff |
| 8 | Sniff and display open ports |
| 9 | Sniff and display Ethernet addresses |
| 10 | Sniff and display network statistics |
| 11 | Sniff and verify checksums |

### record / pcap

| 号 | 标题 |
|----|------|
| 14 | Spoof a record |
| 15 | Display content of a record |
| 16 | Convert a record |
| 17 | Recompute checksums of packets in a record |
| 18 | Reassemble IP packets of a record, and reorder TCP flow |
| 19 | Extract a range of packets from a record |
| 20 | Search for strings in packets from a record |

### 非网络 / 文件

| 号 | 标题 |
|----|------|
| 21 | Convert a number |
| 22 | Convert a string |
| 23 | Display ASCII table |
| 24 | Convert IP addresses ranges |
| 25 | Test if a directory is secure |
| 26 | Dump a file |
| 27 | Compute MD5 of a file |
| 28 | Convert a binary file to readable and editable file |
| 29 | Convert a readable and editable file to a binary file |
| 30 | Convert a file from unix to dos |
| 31 | Convert a file from dos to unix |
| 127 | Cypher/decypher a file using a xor |
| 128 | Split a file in smaller chunks |
| 129 | Reassemble chunks of a file |
| 186 | Millisecond sleep |
| 190 | Make coffee |
| 191 | Generate a password (English, French, Spanish) |
| 216 | Beep |
| 219 | Compute cryptographic hash of a file (md5, sha, etc.) |
| 220 | Convert a binary file to a base64 encoded file |
| 221 | Convert a base64 encoded file to a binary file |

### spoof

| 号 | 标题 |
|----|------|
| 32 | Spoof Ethernet packet |
| 33 | Spoof EthernetArp packet |
| 34 | Spoof EthernetIp4 packet |
| 35 | Spoof EthernetIp4Udp packet |
| 36 | Spoof EthernetIp4Tcp packet |
| 37 | Spoof EthernetIp4Icmp4 packet |
| 38 | Spoof Ip4 packet |
| 39 | Spoof Ip4Udp packet |
| 40 | Spoof Ip4Tcp packet |
| 41 | Spoof Ip4Icmp4 packet |
| 42–48 | Spoof packet samples：fragment 及 ip4opt noop/rr/lsrr/ts/ipts/ippts |
| 140 | Spoof EthernetIp6 packet |
| 141 | Spoof EthernetIp6Udp packet |
| 142 | Spoof EthernetIp6Tcp packet |
| 143 | Spoof EthernetIp6Icmp6 packet |
| 144 | Spoof Ip6 packet |
| 145 | Spoof Ip6Udp packet |
| 146 | Spoof Ip6Tcp packet |
| 147 | Spoof Ip6Icmp6 packet |
| 192 | Spoof of packet samples : fragment, ip4opt:ssrr |

42–48 展开：42 fragment；43 noop；44 rr；45 lsrr；46 ts；47 ipts；48 ippts。

### ping / traceroute / scan（诊断）

| 号 | 标题 |
|----|------|
| 49–56 | Ping ICMP/TCP/UDP/ARP（及 EthIp spoof 变体） |
| 57–64 | Traceroute ICMP/TCP/UDP/指定 IP 协议（及 spoof 变体） |
| 65–72 | Scan ICMP/TCP/UDP/ARP（及 spoof 变体） |
| 148–149 | Ping ICMP6 Neighbor Discovery（及 spoof） |
| 150–151 | Scan ICMP6 Neighbor Discovery（及 spoof） |
| 214–215 | Traceroute discovery 拓扑图（及 spoof） |

49–56 展开：49 ICMP；50 ICMP spoof；51 TCP；52 TCP spoof；53 UDP；54 UDP spoof；55 ARP；56 ARP spoof。
57–64：57 ICMP；58 ICMP spoof；59 TCP；60 TCP spoof；61 UDP；62 UDP spoof；63 指定协议；64 指定协议 spoof。
65–72：65 ICMP；66 ICMP spoof；67 TCP；68 TCP spoof；69 UDP；70 UDP spoof；71 ARP；72 ARP spoof。

### 通用 client / server / relay / perf / bridge

| 号 | 标题 |
|----|------|
| 87 | TCP client |
| 88 | UDP client |
| 89 | TCP server |
| 90 | UDP server |
| 91 | TCP server multiclients |
| 92 | UDP server multiclients |
| 110 | Ethernet bridge limiting flow |
| 155–158 | 网络性能测量 TCP/UDP server/client |
| 183 | TCP relay |
| 184 | UDP relay |
| 185 | TCP multiclient relay |

155–158：155 TCP server；156 TCP client；157 UDP server；158 UDP client。

### remadm

| 号 | 标题 |
|----|------|
| 93 | TCP remote administration server |
| 94 | TCP remote administration client (exec) |
| 95 | TCP remote administration client (get file) |
| 96 | TCP remote administration client (put file) |
| 126 | HTTP remote administration server |

### 应用协议

DNS：102 Query；103 Bind version；104 固定应答 server；105 sniff 并应答。
SYSLOG：97 client；188 server。
TELNET：99 client；100 执行命令；170 server。
SMTP/邮件：106 发送；177 探测；189 server；223 转发。
NNTP：107 发帖；108 列表；109 下载；172 文章范围；173 overview。
FTP：111–117 列/get/put/del 及递归；168 server；174 get+MD5。
HTTP：118–124 GET/HEAD/POST/PUT/DELETE/TRACE/OPTIONS；125 server；136/175/182 下载与 MD5/size；137–139、210 spider；133–135、211–212、222 URL/HTML。
IRC：152 交互；153 列频道；154 听频道；178 探测。
SNMP：159 Get；160 Walk；161 Trap；162 Trap2；163 Inform；164 Set。
TFTP：165 get；166 put；167 server；176 get+MD5。
DHCP：171 client；179 INFORM。
SNTP：180 client；181 server。
IDENT：193 查会话；194 建会话再查；195 server。
WHOIS：196 client；197 猜 server。
SMB/CIFS：198–209 客户端文件/目录；217 server。

### DEFERRED 审计 / 暴力破解

实现前必须单独批准。详见 `spec/netaudit/_deferred.md`。

| 号 | 标题 |
|----|------|
| 73 | Simulate presence of a/several computer/s (arp and ping) |
| 74 | Flood a host with random fragments |
| 75 | Fill table of a switch using a flood of Ethernet packets |
| 76 | Synflood |
| 77 | Check if seqnum are predictible |
| 78 | Reset every TCP packet |
| 79 | Acknowledge every TCP SYN |
| 80 | Periodically send ARP replies |
| 81 | Send an ICMP4 timestamp |
| 82 | Sniff and send ICMP4/ICMP6 destination unreachable |
| 83 | Sniff and send ICMP4/ICMP6 time exceeded |
| 84 | Sniff and send ICMP4/ICMP6 parameter problem |
| 85 | Sniff and send ICMP4 source quench |
| 86 | Sniff and send ICMP4/ICMP6 redirect |
| 98 | Flood a host with syslog messages |
| 101 | Brute force telnet client |
| 130 | Brute force ftp client |
| 131 | Brute force http client (site password) |
| 132 | Brute force http client (proxy password) |

## netwag 功能面

来源：`lessons.html`。GUI 对齐工作流，不用 Tcl。

| 面 | 要点 |
|----|------|
| Local_info | 本机 devices / IP / ARP / routes（后端即工具 1/169 一类） |
| Remote_info | 查远端主机（工具 3 一类） |
| Clipboard | 多剪贴板、共享小剪贴板 |
| Search | 按树/关键字搜工具（工具 0 `--tools`） |
| Help | 动态帮助（工具 0 `-h`） |
| Form | 参数表单（工具 0 `-f`） |
| Run | 运行/杀进程（`-r` `-R` `-k`） |
| History | 命令历史 |

## 覆盖核对

- 工具号 0 与 1–223 均已出现在上表（42–48、49–72、155–158 为连续号压缩，已注明展开）。
- 后置 20 个：73–86（14）+ 98 + 101 + 130–132（3）= 20。
- 其余 203 个用户工具 + 工具 0 进入主路线。
