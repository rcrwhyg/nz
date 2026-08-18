# 缺口（第 0 闸记录）

动手写业务代码前仍需拍板。已关闭项见文末。

## 待拍板

（当前无待拍板事项。）

## 已关闭

- netwib 源码与 HTML 手册：已在对照树内
- netwox 每工具 HTML：`tools/N.html`
- 工具 0 协议：netwag Tcl 调用点已定位
- 参考树是否入库：**不进 git**（`.gitignore`）
- GUI 是否用 webview：**不用**
- git 节奏：提交前 review；确认后一笔；同笔未 push 可 amend；功能集完成或已有后续功能则新提交
- 复刻口径：走“能力全集覆盖、非 1:1 克隆”路线
- 许可证：按 GPL-3 路线推进（当前阶段不走 MIT/Apache）
- native GUI crate：**egui**（即时模式、生态成熟、LLM 友好；GPUI 因上游暂停对外维护而排除）
- CLI 调用方式：**双模式**（`nz 49` 数字兼容 + `nz ping-icmp` 命名，维护静态映射表）
- 目标平台：Tier 1 = macOS aarch64 + Linux x86_64；Tier 2 = Windows + FreeBSD（不阻塞提交）
- workspace 切分：**现在就拆**（`nz-net` 库 + `nz` CLI + `nz-gui`，骨架阶段建好结构）
- 原 `test/netwibtest`：**分批整理成测试对照表**（随模块实现进度提取，放 `doc/netwib/test-matrix.md`，确保不遗漏边界用例）
- Cargo edition：以根 `Cargo.toml` workspace 为准，edition 2024
