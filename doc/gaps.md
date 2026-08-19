# 缺口

第 0 闸（骨架 + 开源基础设施）已关闭。已关闭项见文末。

## 待拍板

（当前无待拍板事项。crates.io 包名若将来发布，与 CLI 名 `nz` 分开处理。）

## 已关闭

- netwib 源码与 HTML 手册：已在对照树内
- netwox 每工具 HTML：`tools/N.html`
- 工具 0 协议：netwag Tcl 调用点已定位
- 参考树是否入库：**不进 git**（`.gitignore`）
- GUI 是否用 webview：**不用**
- git 节奏：提交前 review；确认后一笔；同笔可 amend。CI 失败则修进同一笔，已 push 时用 `--force-with-lease`。功能集完成或已有后续功能则新提交
- 复刻口径：走“能力全集覆盖、非 1:1 克隆”路线
- 许可证：按 GPL-3 路线推进（当前阶段不走 MIT/Apache）
- native GUI crate：**egui**（即时模式、生态成熟、LLM 友好；GPUI 因上游暂停对外维护而排除）
- CLI 调用方式：**双模式**（`nz 49` 数字兼容 + `nz ping-icmp` 命名，维护静态映射表）
- 目标平台：Tier 1 = macOS aarch64 + Linux x86_64；Tier 2 = Windows + FreeBSD（不阻塞提交）
- workspace 切分：**现在就拆**（`nz-net` 库 + `nz` CLI + `nz-gui`，骨架阶段建好结构）
- 原 `test/netwibtest`：**分批整理成测试对照表**（随模块实现进度提取，放 `doc/netwib/test-matrix.md`，确保不遗漏边界用例）
- Cargo edition：以根 `Cargo.toml` workspace 为准，edition 2024
- crate 发布名：产品/CLI 仍叫 `nz`；若上 crates.io 再另取包名（现有 `nz` crate 已被占用）
- GitHub：仓库 `rcrwhyg/nz`，默认分支 `main`；release-plz 仅手动触发。第 1–3 闸确认提交后即 push
- 本地质量门：**pre-commit 唯一入口**（commit 钩子 + `pre-commit run --all-files`）；GitHub Actions 做远端对照，不另写平行 sh
