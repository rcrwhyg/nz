# transport 共用字段

套接字类工具共用参数长名与 C `NETWOX_SOCK_ARG_*` 一致。各工具 spec 只钉 Usage 与本工具特有语义。decode/encode 枚举与工具 12 对齐，缺省均为 `data`。

## 套接字参数（87–92、155–158、183–185）

| 长名 | 短 | 客户 | 服务端 | 默认 / 备注 |
|------|----|------|--------|-------------|
| `--device` | `-d` | 可选 | 可选 | 缺省网卡；任一 Eth 相关参数会切 VIRT |
| `--src-eth` | `-E` | 可选 | 可选 | HTML 示例 `0:2:3:4:5:6` |
| `--dst-eth` | `-e` | 可选 | — | 仅客户 |
| `--src-ip` | `-I` | 可选 | 可选 | isset 则 REAL full |
| `--dst-ip` | `-i` | **必填** | — | 仅客户 |
| `--src-port` | `-P` | 可选 | **必填** | 客户默认 `0`（系统分配） |
| `--dst-port` | `-p` | **必填** | — | 仅客户 |
| `--ip4opts` | `-o` | 可选 | 可选 | mixed |
| `--ip6exts` | `-O` | 可选 | 可选 | 首字节为扩展协议号 |
| `--iptype` | `-t` | — | 可选 | 未给 `--src-ip` 时用；缺省 `ip4` |

初始化路径见 [net-sock.md](../../netwib/net-sock.md)。

## telnet 类（87–90）

键盘 → 网络：`--decode`（`-k`，缺省 `data`）。网络 → 屏幕：`--encode`（`-n`，缺省 `data`）。双向 `io_kbd_loop`（stdin + 套接字）。**registry stdin：87–90**（工具 0 `--tools`）。

89 配 87、90 配 88。同义词 nc/netcat。

## 多客户 echo（91–92）

`--showscreen`（`-s`）打到屏幕，**默认关**。`--echoback`（`-b`）回显给客户，**C 默认 `"1"`（开）**。每连接 `io_showecho`。

## perf 协议（155–158）

专用二进制消息（对照 `perf.c` / `perf.h`），version 仅支持 1。

| 类型 | 值 | 大小（字节） | 体 |
|------|-----|-------------|-----|
| HELLO | 1 | 20 | version, durationms, datasize |
| DATA | 2 | ≥24 | sendtime(sec,nsec), recvtime(sec,nsec) |
| BYE | 3 | 12 | numdatasent |
| BYEACK | 4 | 8 | （空） |

每条消息前 8 字节：`msgsize`（含自身）、`perfmsgtype`。

**客户端**（156/158）：发 HELLO（version=1；durationms = `--duration` + 200；datasize = `--chunksize`）→ 收 DATA 至 BYE → 回 BYEACK。约每秒打印 `Bytes/sec`、`[=~= kbit/sec]`（公式 `19*bps/2120`）、`jitter`（RFC 1889 式，微秒）。无包时提示检查服务端；UDP 丢 BYE 时 numdatasent 可能 unknown。

**服务端**（155/157）：收 HELLO 取 duration/datasize → 在 duration 内循环发 DATA → 发 BYE → 等 BYEACK（10s 超时）。

客户端 `--chunksize` 默认 500；`--duration` 毫秒默认 5000。同义词 iperf。

## relay（183–185）

必填：`--src-port`、`--server-ip`、`--server-port`。`--allowed-clients`（`-c`）缺省 `all`（C `OPTIPS_SU` 未给即 NULL=全允许）；不在列表则打印 `IP address … is not allowed` 并断开。

- **183/184**：`--server-ip` 单个 IP。
- **185**：`--server-ip` 为 **IP 列表**（逗号分隔）；每连接随机选一后端（负载均衡）；可同时多路（`tcp_mulser` + 每连接 `relay_loop`）。

TCP relay：接受后查 allowed → 连后端 → 双向转发至 DATAEND。UDP relay：首包后查 allowed。连后端失败打印 `Could not connect to …` 并结束该会话。

## bridge（110）

与套接字族无关。双网卡 sniff+spoof 桥接；`--max12` / `--max21` 字节/秒，`0`=不限；超限入 ring 按速率放出。学习各侧以太网源地址，来自对侧 LAN 的包才转发。`--verbose` **默认开**（SNIFF/SPOOF 行）。

## 非目标

- CI 不对真网卡跑 bridge 或长 duration perf
- relay 默认不得对公网开放（`--allowed-clients` 须实现）
- 不为 perf 另造与 netwox 不兼容的协议
