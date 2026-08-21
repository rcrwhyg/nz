# 库能力 spec

- 模块：net / 网卡
- 对照：`netwib-doc_html/netwib/net.html` + `net/device.h` + `net/confdev.h`
- 状态：implemented

## 能力

列举本机网卡（对照 `conf_devices`），供 sniff/spoof/socket 选设备。每块至少：

- 编号
- 易记名（原 `deviceeasy`，CLI 例 `Eth0`）
- 真实设备名
- 硬件类型：unknown / ether（含 Wi-Fi，原注来不及单列）/ loopback / ppp / 过时 parallel 与 serial
- MTU
- Ethernet 地址（`hwtype` 为 ether 时；否则可空）

DLT 枚举与 `device.h` 同模块，但**各卡 sniff/spoof 能否用及 DLT** 是打开通道时探测的（工具 13），不在本列举里。

不在本 spec 做键盘选设备（那是 CLI）。

## Rust 形状

`struct Device { .. }` + `fn list_devices() -> Result<Vec<Device>>`。测试用假列表，不要求 CI 有真网卡。禁止 `netwib_*`。读真网卡的代码可 `cfg` 且默认测试不调用。

## 非目标

- 不在本 spec 实现 sniff/spoof IO，也不探测每卡 DLT
- 不在本 spec 实现 IP/ARP/路由 conf（后续 `net-conf`，服务工具 1/6/169）
- 不暴露 libpcap 类型到公共 API

## 验收

- [ ] `device_hwtype_roundtrip`：各硬件类型可显示且解析
- [ ] `device_list_fake`：假后端返回至少一块 ether 与一块 loopback，字段齐全
- [ ] `device_list_real_ignored_in_ci`：真列举路径存在则用 feature 或忽略；CI 不依赖 root

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：真网卡列举在 feature `system-inventory` 下启用（`if-addrs`）；CI `--all-features` 覆盖，不依赖 root。MAC/MTU 在真路径上可能为占位（MTU 默认值、MAC 暂空）。

## 依赖

[err.md](err.md)
