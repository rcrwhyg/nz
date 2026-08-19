# 库 spec（nz-net）

模板：[TEMPLATE-lib.md](../TEMPLATE-lib.md)。对照 [doc/netwib/modules.md](../../doc/netwib/modules.md)。

相位 1（库骨架）已起草：

| 文件 | 范围 |
|------|------|
| [err.md](err.md) | 错误分区与 `Result` |
| [dat.md](dat.md) | buf、编解码、Internet checksum |
| [net-addr.md](net-addr.md) | IP/Eth/port 与集合语法 |
| [net-device.md](net-device.md) | 网卡列举（测试用假后端） |
| [pkt.md](pkt.md) | 分层编解码、conv、分片 |

尚未写（后续相位）：sys/record、net conf/sock/sniff/spoof、shw、dat regexp/TLV。CLI 注册表与工具 0 见 [../netwox/README.md](../netwox/README.md)。

状态均为 `draft`，用户批准后改 `approved`。
