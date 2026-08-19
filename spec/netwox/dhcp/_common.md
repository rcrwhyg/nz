# DHCP 共用字段

需 **sniff+spoof**（`snispo_init_eth`）。filter 固定 `udp and (port 67 or port 68)`。对照 [doc/netwox/proto-misc.md](../../../doc/netwox/proto-misc.md) DHCP 段。

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--device` | `-d` | （缺省网卡） | 嗅探/伪造网卡 |
| `--eth-src` | `-e` | initdefault | 客户端 MAC |
| `--timeout` | `-T` | `30000` | ms（等 OFFER/ACK） |

179 另有 `--ip-src`（`-i`）：INFORM 时客户端已有 IP。

## 171 交互

registry **stdin**：租约持有阶段按 **`q`** 退出并 RELEASE。DISCOVER **不重发**（丢包则等超时）。

## 179 交互

发 INFORM 后等首个 ACK 即结束；**无** stdin 续租循环。

## 非目标

- 171 接受首个 OFFER：实现闸不得改成选最优
- 仅授权实验室/书面授权 LAN
