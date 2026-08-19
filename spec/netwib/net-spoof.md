# 库能力 spec

- 模块：net / spoof
- 对照：`netwib-doc_html/` sniff/spoof 段 + `net/spoof*.h`
- 状态：draft

## 能力

打开伪造发送通道，把已组好的帧写出去。两条路径：

1. **链路**：按网卡打开；工具 32–37、140–143 要求 DLT 为 `ether`，否则失败。
2. **IP**：`--spoofip`（缺省 `best`=`linkraw`）决定先 raw 还是先 link、源 MAC 填还是留空。取值与别名见 [spoof/_common.md](../netwox/spoof/_common.md) 与工具 12。

组包本身走 [pkt.md](pkt.md)（自动 checksum vs 字面高级字段；`ip4-id` / `tcp-seqnum` 未指定则随机）。本 spec 只负责发出去。

默认测试用**假后端**：记下发出的字节与选用的路径，不打真网。真路径 `privileged-tests`；CI 不对公网、不依赖 root。仅本机 / 实验室 / 书面授权目标。

## Rust 形状

`fn open_spoof_link(device) -> Spoof` + `fn open_spoof_ip(SpoofIp) -> Spoof` + `fn send(&[u8])`。禁止 `netwib_*`。本闸不加运行时依赖。

## 非目标

- 不在本 spec 解析 BPF（那是 sniff）
- 不为每个工具复制编解码
- 不对公网发包

## 验收

- [ ] `spoof_link_fake_records_frame`：假链路通道记下完整以太网帧
- [ ] `spoof_link_rejects_non_ether`：假 raw DLT 通道失败
- [ ] `spoof_ip_best_alias`：`best` 与 `linkraw` 同一策略
- [ ] `spoof_real_ignored_in_ci`：真发送路径 feature/忽略

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：真 raw socket 在 CI 可能测不到，缺口写「仅假后端计入覆盖」。

## 依赖

[err.md](err.md)、[net-device.md](net-device.md)、[pkt.md](pkt.md)
