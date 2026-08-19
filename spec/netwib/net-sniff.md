# 库能力 spec

- 模块：net / sniff
- 对照：`netwib-doc_html/netwib/net.html` sniff 段 + `net/sniff*.h`
- 状态：draft

## 能力

打开嗅探通道，按网卡与 BPF/pcap 过滤读包，并报告该通道 DLT。可选挂上 IP 重组与 TCP 序号重排（语义见 [pkt.md](pkt.md)，允许丢包）。

默认测试走**假后端**：注入事先准备的帧（或 [record.md](record.md) 回放），不打开真网卡。真路径可 `cfg` + `privileged-tests`；CI 不依赖 root、不对公网。macOS CI 不假设 BPF 设备可用。

过滤语义对齐 libpcap BPF（`host`/`net`/`port`/`ether host` 等）。实现闸可链系统 pcap；不必自研完整 BPF 编译器。假后端可用谓词或预过滤夹具代替。

`--rawip` 时通道 DLT 为 `raw`（从 IP 头起）；否则为该网卡 sniff DLT（工具 13）。

Linux 打开通道后可降权到 nobody（工具 7 `--losepriv`）；降权失败则报错。测试不要求真降权。

## Rust 形状

`fn open_sniff(device, filter) -> Sniff` + `fn dlt() -> Dlt` + 读下一帧。重组/重排是可叠的读过滤器，不是每工具一份。禁止 `netwib_*`。本闸不加运行时依赖。

## 非目标

- 不实现 spoof 发送（后续 `net-spoof`）
- 不在本 spec 解析 DNS/DHCP（shw）
- 不对公网抓包

## 验收

- [ ] `sniff_fake_yields_injected`：假通道按序交出注入的两帧
- [ ] `sniff_fake_dlt_ether`：假 ether 网卡 DLT 为 `ether`
- [ ] `sniff_rawip_dlt_raw`：rawip 模式 DLT 为 `raw`
- [ ] `sniff_filter_host_predicate`：过滤 `host 1.2.3.4` 时假后端只留该主机相关帧（或预过滤夹具）
- [ ] `sniff_real_ignored_in_ci`：真网卡路径存在则 feature/忽略

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：真 pcap/BPF 编译在 CI 可能测不到，缺口写「仅假后端计入覆盖」。

## 依赖

[err.md](err.md)、[net-device.md](net-device.md)、[pkt.md](pkt.md)、[record.md](record.md)
