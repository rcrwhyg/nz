# record / pcap 工具

相位：`spec/_index.md` 第 5 项。金标准：`src/netwox-doc_html/tools/N.html`，默认值与互斥项以 `000NNN.c` / `toolarg.c` 为准。
布尔仍是 `-x|+x|--no-x`。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名，实现 CLI 闸再钉死。

## 共用：record 是什么

record 是捕获文件：嗅探得到，或手工编辑。读写走库能力（netwib `sys/record`），工具不要各自私有编解码。

**读**时按文件内容识别格式，不必指定输入 `recordencode`。
**写**时用 `--recordencode`，CLI 缺省 **`bin`**（`toolarg.c` 类型默认）。建议值：`bin`、`pcap`、`mixed_wrap`。

CLI 枚举（工具 12 / 表单 listbox）：

`bin` `dump` `hexa` `hexa_wrap` `mixed` `mixed_wrap` `mixedh_wrap` `pcap`

HTML 写「7 种」时漏了 `mixedh_wrap`。`pcap` 与 tcpdump/libpcap 兼容；`bin` 快、人不可读；其余便于阅读编辑。

DLT（Data Link Type）表示包从哪一层开始：最常见 **`ether`**（以太网头）和 **`raw`**（IP 头）。CLI 缺省 **`ether`**。全表见工具 12；本机各网卡 sniff/spoof DLT 见工具 13。

## 14 — Spoof a record

建议名：`record-spoof`
Usage：`nz 14 -f file [-t dlt] [-s|+s] [-k|+k] [-d device] [-i spoofip]`

读 record 并把每包发到网上。需 spoof 权限。工具 0 把它标为需要 stdin（`--keypress` 逐包按键）。

| 参数 | 含义 |
|------|------|
| `-f/--file` | 输入 record（必填） |
| `-t/--dlt` | 文件内报文 DLT，缺省 `ether` |
| `-s/--screen` | 发送前在屏幕上显示（synth） |
| `-k/--keypress` | 每包发送前按键 |
| `-d/--device` | DLT **不是** `raw` 时的 spoof 网卡 |
| `-i/--spoofip` | DLT **是** `raw` 时如何发 IP；缺省 `best`。取值见工具 12 |

record 的 DLT 必须与 spoof 通道 DLT 一致，否则失败。仅本机/实验室/书面授权目标。
例：`nz 14 -f capture.pcap`

## 15 — Display content of a record

建议名：`record-show`
Usage：`nz 15 -f file [-t dlt] [-H encode] [-D encode] [-x|+x]`

读 record 并显示。无特权。

| 参数 | 含义 |
|------|------|
| `-f/--file` | 输入 record（必填） |
| `-t/--dlt` | DLT，缺省 `ether` |
| `-H/--hdrencode` | 头展示，缺省 `array` |
| `-D/--dataencode` | 载荷展示，缺省 `dump` |
| `-x/--extended` | 尝试解码 DNS/DHCP 等；**默认开** |

常用 encode：`array`、`dump`、`synth`、`nothing`、`text`。全表见工具 12。
例：`nz 15 -f capture.pcap`

## 16 — Convert a record

建议名：`record-convert`
Usage：`nz 16 -f file [-t dlt] -F file [-T dlt] [-r recordencode]`

转换格式，并可改 DLT（例如 pcap → mixed；ether ↔ raw）。输入/输出 DLT 不同时，用库的报文转换，缺字段填空白。无特权。

| 参数 | 含义 |
|------|------|
| `-f/--src-file` | 输入（必填） |
| `-t/--input-dlt` | 输入 DLT，缺省 `ether` |
| `-F/--dst-file` | 输出（必填） |
| `-T/--output-dlt` | 输出 DLT，缺省 `ether` |
| `-r/--recordencode` | 输出编码，缺省 `bin` |

例：`nz 16 -f in.pcap -F out.mixed --recordencode mixed_wrap`

## 17 — Recompute checksums of packets in a record

建议名：`record-csum`
Usage：`nz 17 -f file [-t dlt] -F file [-r recordencode]`

重算校验和写入另一 record。按输入 DLT 从链路层算。算失败则**原包原样写出**（不丢包）。无特权。

| 参数 | 含义 |
|------|------|
| `-f/--src-file` | 输入（必填） |
| `-t/--input-dlt` | 输入 DLT，缺省 `ether` |
| `-F/--dst-file` | 输出（必填） |
| `-r/--recordencode` | 输出编码，缺省 `bin` |

例：`nz 17 -f in.pcap -F out.pcap`

## 18 — Reassemble IP packets of a record, and reorder TCP flow

建议名：`record-reasm`
Usage：`nz 18 -f file [-t dlt] -F file [-r recordencode] [-o|+o]`

读入后做 IP 重组；`--tcpreord` 再按 TCP 序号重排。原文写明可能丢包。无特权。

| 参数 | 含义 |
|------|------|
| `-f/--src-file` | 输入（必填） |
| `-t/--input-dlt` | 输入 DLT，缺省 `ether` |
| `-F/--dst-file` | 输出（必填） |
| `-r/--recordencode` | 输出编码，缺省 `bin` |
| `-o/--tcpreord` | 同时重排 TCP；默认关 |

例：`nz 18 -f in.pcap -F out.pcap --tcpreord`

## 19 — Extract a range of packets from a record

建议名：`record-slice`
Usage：`nz 19 -f file -F file [-r recordencode] [-s uint32] [-e uint32]`

按包序号切片（从 1 起）。无特权。不需要 DLT（原样拷贝字节）。

| 参数 | 含义 |
|------|------|
| `-f/--src-file` | 输入（必填） |
| `-F/--dst-file` | 输出（必填） |
| `-r/--recordencode` | 输出编码，缺省 `bin` |
| `-s/--start` | 起始包号（从 1）。`0` 当作 1 |
| `-e/--end` | 结束包号。`0` = 无上限 |

`start > end` 为无效范围。例：包 2 到 5 → `--start 2 --end 5`。
例：`nz 19 -f in.pcap -F out.pcap --start 2 --end 5`

## 20 — Search for strings in packets from a record

建议名：`record-grep`
Usage：`nz 20 -f file -F file [-r recordencode] -p data [-c|+c] [-S|+S] [-M|+M] [-R|+R]`

在整包字节里搜模式，命中的包写入输出。`-S`/`-M`/`-R` 是互斥单选，**默认 string**。无特权。

| 参数 | 含义 |
|------|------|
| `-f/--src-file` | 输入（必填） |
| `-F/--dst-file` | 输出（必填） |
| `-r/--recordencode` | 输出编码，缺省 `bin` |
| `-p/--pattern` | 模式（必填；mixed 例：`'hello' 09 'bob'`） |
| `-c/--case` | 区分大小写；默认关 |
| `-S/--string` | 按原始字节搜（默认） |
| `-M/--mixed` | 先按 mixed 解码再搜 |
| `-R/--regexp` | 正则；包内 `NUL` 先换成空格再匹配 |

例：`nz 20 -f in.pcap -F out.pcap -p HTTP`

## 非目标

- 不为每个工具复制一套 record 编解码；统一走库
- 14 不对未授权网络发送
- 18 不保证不丢包
- 输出排版可现代化，格式名与 DLT/encode 枚举必须可映射
