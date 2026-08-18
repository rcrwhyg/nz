# 架构对照

原栈与 nz 目标（能力对齐，不是 C ABI 对齐）。

```
netwib (C 库)     -->  nz-net（Rust 等价能力）
     |
netwox (CLI)      -->  nz（clap 数字工具号 + 具名子命令，CLI 双入口待拍板）
     |
netwag (Tcl/Tk)   -->  nz-gui（native crate：egui / iced / slint，禁止 webview）
```

安装/运行依赖（原版）：libpcap 或 WinPcap、可选 libnet、Tcl/Tk（仅 GUI）。Rust 侧用生态 crate 替代，不链原 `libnetwib`。

## 指针

发行版根：`netw-ib-ox-ag-5.39.0/`（README 只指向 Unix/Windows 安装说明）。

### 库 netwib

- 源码：`netw-ib-ox-ag-5.39.0/src/netwib-src/`
- 入口：`src/netwib-src/src/netwib.h` → `err` `dat` `sys` `net` `pkt` `shw`
- HTML：`src/netwib-doc_html/index.html`
- 自述：`src/netwib-src/doc/presentation.txt`（作者声明 API 不稳定、取各平台最小公倍数）

### 工具 netwox

- 源码：`src/netwox-src/`（`src/tools/000NNN.c`，`modules/` 协议实现）
- 分类树：`src/netwox-src/src/modules/tool/tooltree.h` / `tooltree.c`
- 扁平清单：`src/netwox-src/doc/toollist.txt`
- HTML：`src/netwox-doc_html/index.html`，每工具 `src/netwox-doc_html/tools/N.html`
- 未实现：`src/netwox-src/doc/unimplemented.txt`

### GUI netwag

- 源码：`src/netwag-src/src/*.tcl`
- 通过 `netwox 0` 驱动：`--tools` `-h` `-f` `-r` `-R` `-k` `-c` `-v`
- 课表：`src/netwag-doc_html/html/lessons.html`

### 原测试

- `netw-ib-ox-ag-5.39.0/test/README.TXT`：`netwibtest`、netwox `checkparams`
- 第 1 闸可摘能力清单；不移植 C 测试框架

### 不复刻

- `src/*-bin_windows/` 预编译包
- `installwindows.exe` 安装器
- 对照树本身的构建系统（`genemake`）
