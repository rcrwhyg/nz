# 库 spec（nz-net）

模板：[TEMPLATE-lib.md](../TEMPLATE-lib.md)。对照 [doc/netwib/modules.md](../../doc/netwib/modules.md)。

相位 1（库骨架）已起草；相位 4 补了哈希；相位 5–10 补了 record、sniff、spoof、sock、DNS 与 HTTP/HTML/spider：

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
| [net-spoof.md](net-spoof.md) | 伪造发送通道与假后端（相位 7） |
| [net-sock.md](net-sock.md) | TCP/UDP 套接字与虚拟 sockv（相位 9） |
| [pkt-dns.md](pkt-dns.md) | DNS 编解码、查询与固定应答（相位 10） |
| [proto-http.md](proto-http.md) | HTTP 客户端/服务端与 URL 下载（相位 10） |
| [proto-html.md](proto-html.md) | 离线 HTML/URL 处理（相位 10） |
| [proto-spider.md](proto-spider.md) | Web spider 与本地映射（相位 10） |

尚未写（后续相位）：shw、dat regexp/TLV。CLI 见 [../netwox/README.md](../netwox/README.md)。

状态均为 `draft`，用户批准后改 `approved`。
