---
name: nz-privileges
description: >-
  Privilege rules for sniff/spoof/raw sockets, Linux losepriv, and authorized
  targets only. Use when implementing capture, injection, scan, DHCP client,
  remadm on low ports, or adding privileged-tests.
---

# 权限

嗅探、伪造、扫描**仅**本机、实验室或书面授权目标。CI 不对公网、不依赖 root。

## 谁需要特权

| 类 | 例 | 说明 |
|----|----|------|
| sniff | 7–11、105 | 抓包 |
| spoof | 14、32+、record 发送 | 发假包 |
| sniff+spoof | ping/scan/diagnose、110 bridge、171 DHCP | 两边都要 |
| 低端口 | HTTP/FTP 服务口 80/21 等 | 非特权可改高口 |

无特权路径（文件、多数 TCP 客户）不要误加 raw socket。

## 实现

- 默认测试：pcap 回放 / 假接口。
- 真网卡：`privileged-tests` feature，文档标明。
- Linux `--losepriv`（工具 7 的 `-Q`）：降到 nobody，默认关。
- macOS：不要在 CI 假设 BPF 可用。
- 工具 13 报告每块网卡 sniff/spoof 与 DLT。

## 禁止

- 为「跑通示例」对公网发探测。
- 未批准实现后置洪水/暴力工具。
- 在 GUI/CLI 把弱默认口令宣传成安全配置。
