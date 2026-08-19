# 通用 client / server / relay / perf / bridge

相位：`spec/_index.md` 第 9 项。金标准：`src/netwox-doc_html/tools/N.html`。
布尔仍是 `-x|+x|--no-x`。decode/encode 全表见工具 12；缺省都是 `data`。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名，实现 CLI 闸再钉死。

套接字类工具共用可选：`--device`、`--src-eth`、`--dst-eth`（仅客户）、`--src-ip`、`--ip4opts`、`--ip6exts`。未给 `--src-ip` 时 `--iptype` 选 `ip4`（缺省）或 `ip6`。`--src-port` 客户侧 0 表示系统分配。

87–90 需要 stdin（工具 0）。无特权即可跑套接字路径；伪造 Eth/IP 或 bridge 才要 sniff/spoof。仅授权目标。

## 客户 / 单客户服务（类 telnet）

| 号 | 标题 | 建议名 | 必填 |
|----|------|--------|------|
| 87 | TCP client | `tcp-client` | `--dst-ip` `--dst-port` |
| 88 | UDP client | `udp-client` | `--dst-ip` `--dst-port` |
| 89 | TCP server | `tcp-server` | `--src-port` |
| 90 | UDP server | `udp-server` | `--src-port` |

键盘入网：`--decode`（缺省 `data`）。网上显示：`--encode`（缺省 `data`）。
89 配 87、90 配 88。

## 多客户 echo 服务

| 号 | 标题 | 建议名 |
|----|------|--------|
| 91 | TCP server multiclients | `tcp-mulser` |
| 92 | UDP server multiclients | `udp-mulser` |

必填 `--src-port`。`--showscreen` 打到屏幕（默认关）。`--echoback` 回显给客户（**默认开**）。

## 性能测量

一对机器：服务端 155/157，客户端 156/158。测吞吐与 jitter。

| 号 | 标题 | 建议名 |
|----|------|--------|
| 155 | Network performance measurement : TCP server | `perf-tcp-server` |
| 156 | Network performance measurement : TCP client | `perf-tcp-client` |
| 157 | Network performance measurement : UDP server | `perf-udp-server` |
| 158 | Network performance measurement : UDP client | `perf-udp-client` |

服务端必填 `--src-port`。客户端必填 `--dst-ip` `--dst-port`；`--chunksize` 缺省 500；`--duration` 毫秒，缺省 5000。CI 用短 duration + 假套接字，不要真跑很久。

## relay

客户连本机，本机再连真正服务；双方都只看见中继地址。`--allowed-clients` 缺省 `all`。

| 号 | 标题 | 建议名 | 差异 |
|----|------|--------|------|
| 183 | TCP relay | `tcp-relay` | `--server-ip` 单个 IP |
| 184 | UDP relay | `udp-relay` | 同上 |
| 185 | TCP multiclient relay | `tcp-mulrelay` | `--server-ip` 为 **IP 列表**；可同时多路，当负载均衡 |

必填：`--src-port`、`--server-ip`、`--server-port`。

## 110 — Ethernet bridge limiting flow

建议名：`eth-bridge`
Usage：`nz 110 -d device -D device [-m uint32] [-M uint32] [-v|+v]`

双网卡机器切开网络，限速以模拟慢链路。需 sniff+spoof。

| 参数 | 含义 |
|------|------|
| `-d/--device1` | 网卡 1（必填） |
| `-D/--device2` | 网卡 2（必填） |
| `-m/--max12` | 1→2 最大字节/秒；`0` = 不限 |
| `-M/--max21` | 2→1 最大字节/秒；`0` = 不限 |
| `-v/--verbose` | **默认开** |

## 非目标

- 不在 CI 打真网卡做 bridge
- 不把 relay 默认成对公网开放的代理（`--allowed-clients` 要实现）
- 编解码枚举与工具 12 对齐，不另造一套
