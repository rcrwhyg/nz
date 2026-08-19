---
name: nz-pcap-record
description: >-
  Handles netwox record/pcap formats, DLT, encode enums, IP reassembly and TCP
  reorder. Use when implementing tools 14-20, sniff save/load, or sys/record IO.
---

# record / pcap

对照：`doc/netwox/record.md`、`doc/netwox/sniff.md`。编解码只在库里做一份。

## 格式

读：按内容识别，不必指定输入 `recordencode`。
写：`--recordencode`，CLI 缺省 `bin`。

枚举：`bin` `dump` `hexa` `hexa_wrap` `mixed` `mixed_wrap` `mixedh_wrap` `pcap`。
HTML「7 种」漏了 `mixedh_wrap`。建议写出：`bin`、`pcap`、`mixed_wrap`。

## DLT

包从哪一层开始。最常见 `ether`（缺省）与 `raw`（IP 头）。全表工具 12；网卡 sniff/spoof DLT 工具 13。

## 工具要点

- 14 发送 record：record DLT 必须等于 spoof 通道 DLT；`--keypress` 要 stdin。
- 16 可改 DLT（缺字段填空白）。
- 18 默认 IP 重组；`--tcpreord` 另排 TCP；允许丢包。
- 19 包号从 1；`start=0`→1；`end=0` 无上限。
- 20 `-S/-M/-R` 互斥，默认 string；正则前把 `NUL` 换成空格。

嗅探落盘（工具 7）的 split 文件名：`basename.YYYYMMDD_HHMMSS.N`。

测试用仓库内或生成的 record，CI 不抓真网。
