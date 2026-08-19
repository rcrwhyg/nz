# 通用 client / server / relay / perf / bridge 工具 spec

相位：`spec/_index.md` 第 9 项。对照 [doc/netwox/transport.md](../../../doc/netwox/transport.md)。共用：[\_common.md](_common.md)。库：[net-sock.md](../../netwib/net-sock.md)、[net-sniff.md](../../netwib/net-sniff.md)、[net-spoof.md](../../netwib/net-spoof.md)。

CI 假套接字/假通道，不对公网。87–90 registry stdin。

| 文件 | 号 | 建议名 |
|------|----|--------|
| [087.md](087.md) | 87 | `tcp-client` |
| [088.md](088.md) | 88 | `udp-client` |
| [089.md](089.md) | 89 | `tcp-server` |
| [090.md](090.md) | 90 | `udp-server` |
| [091.md](091.md) | 91 | `tcp-mulser` |
| [092.md](092.md) | 92 | `udp-mulser` |
| [110.md](110.md) | 110 | `eth-bridge` |
| [155.md](155.md) | 155 | `perf-tcp-server` |
| [156.md](156.md) | 156 | `perf-tcp-client` |
| [157.md](157.md) | 157 | `perf-udp-server` |
| [158.md](158.md) | 158 | `perf-udp-client` |
| [183.md](183.md) | 183 | `tcp-relay` |
| [184.md](184.md) | 184 | `udp-relay` |
| [185.md](185.md) | 185 | `tcp-mulrelay` |

状态均为 `draft`。
