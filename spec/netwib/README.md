# 库 spec（nz-net）

模板：[TEMPLATE-lib.md](../TEMPLATE-lib.md)。对照 [doc/netwib/modules.md](../../doc/netwib/modules.md)。

相位 1（库骨架）已起草；相位 4 补了哈希；相位 5–10 补了 record、sniff、spoof、sock、DNS 与 HTTP/HTML/spider：

| 文件 | 范围 |
|------|------|
| [err.md](err.md) | 错误分区与 `Result` — **implemented** |
| [dat.md](dat.md) | buf、编解码、Internet checksum — **implemented** |
| [net-addr.md](net-addr.md) | IP/Eth/port 与集合语法 — **implemented** |
| [net-device.md](net-device.md) | 网卡列举（测试用假后端）— **implemented** |
| [net-conf.md](net-conf.md) | 本机 IP/ARP/路由与到达（假后端）— **implemented** |
| [pkt.md](pkt.md) | 分层编解码、conv、分片 — **partial**（Eth+IPv4+UDP） |
| [hash.md](hash.md) | 文件/字节摘要（md2–sha512；相位 4） |
| [record.md](record.md) | record/pcap 读写（相位 5） |
| [net-sniff.md](net-sniff.md) | 嗅探通道与假后端（相位 6） |
| [net-spoof.md](net-spoof.md) | 伪造发送通道与假后端（相位 7） |
| [net-sock.md](net-sock.md) | TCP/UDP 套接字与虚拟 sockv（相位 9） |
| [pkt-dns.md](pkt-dns.md) | DNS 编解码、查询与固定应答（相位 10） |
| [proto-http.md](proto-http.md) | HTTP 客户端/服务端与 URL 下载（相位 10） |
| [proto-html.md](proto-html.md) | 离线 HTML/URL 处理（相位 10） |
| [proto-spider.md](proto-spider.md) | Web spider 与本地映射（相位 10） |
| [proto-ftp.md](proto-ftp.md) | FTP 客户端/服务端（相位 10） |
| [proto-tftp.md](proto-tftp.md) | TFTP 客户端/服务端（相位 10） |
| [pkt-dhcp.md](pkt-dhcp.md) | DHCPv4 编解码与客户端状态机（相位 10） |
| [proto-smtp.md](proto-smtp.md) | SMTP 客户端/服务端与 MIME（相位 10） |
| [proto-snmp.md](proto-snmp.md) | SNMP v1/v2c/v3 编解码与会话（相位 10） |
| [proto-smb.md](proto-smb.md) | SMB/CIFS 客户端/服务端（相位 10） |

尚未写（后续相位）：shw、dat regexp/TLV。CLI 见 [../netwox/README.md](../netwox/README.md)。

状态均为 `draft`，用户批准后改 `approved`。
