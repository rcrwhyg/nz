# 相位与功能族

闸门：骨架 → 抽文档 → skills 正文 → spec → crate/CLI/工具 0 → 按族实现 → GUI。

一次任务 = 表中一行里的**一个**可验收单元（一个工具或一块库能力），不要整族一次做完。确认后才提交；同笔修补可 amend（含 CI 失败），见 `.cursor/rules/04-git.mdc`。

验收口径：本项目是“能力全集覆盖”而非“逐项 1:1 克隆”。只要功能不遗漏、语义可映射、契约可测，即可通过；不要求保留原命令分组、内部实现路径或 Tcl/Tk 交互细节。

## 相位

1. 库骨架：err/dat 子集、pkt 编解码、net 地址/设备 — draft 见 [netwib/README.md](netwib/README.md)
2. CLI 注册表 + **工具 0** — draft 见 [netwox/README.md](netwox/README.md)
3. 信息：1–6、12–13、169、187、213、218 — draft 见 [netwox/info/README.md](netwox/info/README.md)
4. 非网络/文件：21–31、127–129、186、191、216、219–221、190 — draft 见 [netwox/files/README.md](netwox/files/README.md)
5. record：14–20 — draft 见 [netwox/record/README.md](netwox/record/README.md)
6. sniff：7–11 — draft 见 [netwox/sniff/README.md](netwox/sniff/README.md)
7. spoof：32–48、140–147、192
8. ping / traceroute / scan：49–72、148–151、214–215
9. 通用 client/server/relay/perf/bridge：87–92、110、155–158、183–185
10. 应用协议按族（一族一闸，族内仍一工具一提交）：DNS、HTTP、FTP、TFTP、DHCP、SMTP、SNMP、SMB、TELNET、SYSLOG、IRC、NNTP、IDENT、WHOIS、SNTP
11. remadm：93–96、126
12. GUI：Search / Form / Run / History / Clipboard / Local_info / Remote_info（native crate）
13. **后置** netaudit + brute：73–86、98、101、130–132（单独批准）
14. `spec/modern/`：TLS、QUIC、HTTP/2/3、DNSSEC 等（对齐完成前不混进旧工具语义）

应用协议工具号见 [doc/inventory.md](../doc/inventory.md)。

## 依赖

库能力 → CLI → 无特权工具 → record/sniff/spoof → 诊断 → 通用传输 → 协议族 → remadm → GUI → 后置审计 → 现代扩展。
