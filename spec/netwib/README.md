# 库 spec（nz-net）

模板：[TEMPLATE-lib.md](../TEMPLATE-lib.md)。对照 [doc/netwib/modules.md](../../doc/netwib/modules.md)。

相位 1（库骨架）已起草；相位 4 补了哈希；相位 5–6 补了 record 与 sniff：

| 文件 | 范围 |
|------|------|
| [err.md](err.md) | 错误分区与 `Result` |
| [dat.md](dat.md) | buf、编解码、Internet checksum |
| [net-addr.md](net-addr.md) | IP/Eth/port 与集合语法 |
| [net-device.md](net-device.md) | 网卡列举（测试用假后端） |
| [net-conf.md](net-conf.md) | 本机 IP/ARP/路由与到达（假后端） |
| [pkt.md](pkt.md) | 分层编解码、conv、分片 |
| [hash.md](hash.md) | 文件/字节摘要（md2–sha512；相位 4） |
| [record.md](record.md) | record/pcap 读写（相位 5） |
| [net-sniff.md](net-sniff.md) | 嗅探通道与假后端（相位 6） |

尚未写（后续相位）：net sock/spoof、shw、dat regexp/TLV。CLI 见 [../netwox/README.md](../netwox/README.md)。

状态均为 `draft`，用户批准后改 `approved`。
