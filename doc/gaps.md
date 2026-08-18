# 缺口（第 0 闸记录）

动手写业务代码前仍需拍板。已关闭项见文末。

## 待拍板

1. **许可证**：对照 GPL-3 源码做行为复刻，发行默认 GPL-3。若要 MIT/Apache，需要可证明净室，成本很高。
2. **产品名与 CLI**：crate 现为 `nz`。是否同时兼容 `nz 49` 与 `nz ping-icmp`。
3. **目标平台**：建议先 macOS/Linux；Windows 单列（原 Windows IPv6 本身不完整）。
4. **native GUI crate**：egui / iced / slint 三选一（**webview 已排除**）。实现 GUI 闸时再比。倾向：egui（工具面板、即时模式、生态成熟）。
5. **workspace 切分**：何时拆 `nz-net` + `nz` + `nz-gui`。建议第 4 闸（crate/CLI）时拆库与二进制，GUI 更晚。
6. **原 `test/netwibtest`**：是否整理成库能力对照表（不移植 C harness）。
7. **Cargo edition**：以根目录 `Cargo.toml` 为准（当前 2024），不臆造 API。

## 已关闭

- netwib 源码与 HTML 手册：已在对照树内
- netwox 每工具 HTML：`tools/N.html`
- 工具 0 协议：netwag Tcl 调用点已定位
- 参考树是否入库：**不进 git**（`.gitignore`）
- GUI 是否用 webview：**不用**
