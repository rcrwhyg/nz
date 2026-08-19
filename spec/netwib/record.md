# 库能力 spec

- 模块：sys / record
- 对照：`netwib-doc_html/` 中 record 相关 + `sys/record.h`、`sys/record.c`
- 状态：draft

## 能力

捕获文件的读/写。工具 14–20、7（落盘）、29（文本当 record 读）共用，禁止每工具一套编解码。

### 读

按**文件内容**识别，不必指定输入 `recordencode`：

| 前 4 字节（大端看） | 格式 |
|---------------------|------|
| `0xA1B2C3D4` / `0xA1B2CD34` 及其字节序对调 | pcap（tcpdump/libpcap） |
| `0xA84C1FE6` | bin |
| 其它 | 文本（dump / hexa / mixed 等；包之间空行；`#` 注释） |

bin 包：魔数之后反复 `u32` 长度（与 C 同字节序）+ 载荷。空长度表示空包。文本允许文件不以空行结束。pcap 时间戳读入后可忽略（工具不依赖）。

### 写

由 `recordencode` 决定。CLI 缺省 **`bin`**。建议值：`bin`、`pcap`、`mixed_wrap`。

CLI 枚举（与工具 12 一致）：`bin` `dump` `hexa` `hexa_wrap` `mixed` `mixed_wrap` `mixedh_wrap` `pcap`。HTML「7 种」漏了 `mixedh_wrap`。

- **bin**：先写魔数，每包长度+载荷
- **pcap**：libpcap 经典头（实现闸钉 DLT 字段；无工具 DLT 参数时缺省 **ether**）
- **文本**：可有注释头；每包按对应 encode 写出，包间空行

DLT **不**存在 bin/文本里；由工具参数告诉解码器包从哪一层开始。pcap 文件头自带链路类型，但 netwox 工具仍用 CLI `--dlt` 解释包（不必自动覆盖 CLI）。

### 追加

文本/bin 可追加。pcap 追加对照 C：**未实现**，nz 同样失败。

## Rust 形状

`enum RecordEncode` + `fn open_read(path) -> impl Iterator<Item=Result<Vec<u8>>>` + `fn open_write(path, encode)`。禁止 `netwib_*`。本闸不加 pcap crate（实现闸再选）。测试用临时文件，不抓真网。

## 非目标

- 不实现 sniff/spoof IO
- 不要求文本注释与 C 逐字相同
- 不在本 spec 做 IP 重组（见 [pkt.md](pkt.md)）

## 验收

- [ ] `record_bin_roundtrip`：两包（含一空包）写 bin 再读回
- [ ] `record_mixed_blank_separates`：mixed 空行分两包
- [ ] `record_read_sniffs_pcap_magic`：合法 pcap 魔数走 pcap 路径（可用最小夹具）
- [ ] `record_read_sniffs_bin_magic`：`0xA84C1FE6` 走 bin
- [ ] `record_pcap_append_fails`：对 pcap 追加失败
- [ ] `record_dump_readable_as_text`：dump 写出的文件能当文本 record 读回（工具 28/29）

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：真 pcap 时间戳/nano 变体可分期，须列出未做魔数。

## 依赖

[err.md](err.md)、[dat.md](dat.md)
