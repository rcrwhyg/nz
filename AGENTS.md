# nz

Rust 复兴 Laurent Constantin 的 **netwib + netwox + netwag 5.39.0** 能力全集，再扩展现代协议。

对照源（只读、不进 git）：`netw-ib-ox-ag-5.39.0/`。细则见 `.cursor/rules/`。

## 三层产品

| 层 | 原组件 | nz | 对齐对象 |
|----|--------|-----|----------|
| 库 | netwib | `nz-net` | `dat/sys/net/pkt/shw` **能力**，不复制 C API |
| 解析 | toolarg/arg | `nz-arg` | netwox 风格参数语义（非 clap API） |
| CLI | netwox | `nz` | 223 个工具能力全覆盖 + **工具 0**（GUI 契约） |
| GUI | netwag | 计划中的 `nz-gui` | lessons 工作流；**egui**，禁止 webview |

依赖方向：`nz-arg` / `nz-net` → CLI → GUI。禁止在库能力未定义时堆工具特例。`nz-arg` ↛ `nz-net`（MVP）。设计见 `doc/nz-arg/`。

## 闸门

0. 骨架（本文件与 `doc/` `spec/` `skills/`）
1. 从 HTML/源码抽详细文档
2. 写 `.cursor/skills/` 正文
3. 按模板写 spec（先库，再工具族）
4. crate + CLI 注册表 + 工具 0 — **已完成**
5. 按族实现工具 — **当前**（一次一工具；已实现：工具 1）
6. native GUI

未经确认不进入下一闸。对齐完成前禁止用「现代化」改旧工具语义。

## 复刻口径

- 能力口径：覆盖 netwox/netwib/netwag 的功能全集，不遗漏工具和关键工作流
- 产品口径：允许非 1:1 克隆（命令组织、内部架构、UI 呈现可现代化）
- 兼容口径：工具参数语义、协议行为、工具 0 契约必须可映射并可验证
- 许可证口径：按 GPL-3 路线推进；除非后续有法律层面的新决议

## 必须做

- 非琐碎决策：问题 + 2～3 方案 + 优缺点 + 推荐；等确认再落盘
- 一次任务 = 一个可验收完整功能（测试全绿、零 warning）
- 本地质量门以 **pre-commit** 为准。克隆后必须 `pre-commit install`；未装钩子禁止提交（禁止 `--no-verify`）。全量复跑用 `pre-commit run --all-files`。不要另写平行 sh 脚本。CI 是远端对照
- 提交前用户 review（可多轮）；确认无误后才提交一次。第 1–3 闸只本地提交；文档阶段全部结束后、编码前统一 push；第 4 闸起写代码后再问是否每笔都 push
- 提交后若只改这一笔且尚未开始后续功能，发现问题可 `--amend`；若该笔已 push 且 GitHub Actions 失败，修进同一笔后用 `git push --force-with-lease`。功能集已完成或后面已有别的功能提交则必须新提交，不 amend
- `cargo build` / `cargo test` / `cargo clippy --all-targets -- -D warnings` 零告警
- 嗅探/伪造/扫描仅本机、实验室或书面授权目标
- 编码标准见 `.cursor/rules/06-coding-standards.mdc`：见名知意、完整注释、合理拆分、优秀 crate 先商量、覆盖率守住

## 禁止

- **当前闸**：按族实现用户工具（一次一工具）。禁止 GUI、后置审计；运行时依赖须先商量再引入
- `git add netw-ib-ox-ag-5.39.0/`
- 修改对照源；把 C 大段贴进 `doc/`
- GUI 使用 Tauri / Wry / webview
- 未单独批准就实现后置审计工具（见 `spec/netaudit/_deferred.md`）
- 针对未授权系统编写利用过程；对公网做集成测试
- 为冲覆盖率改生产行为；无 spec 理由的 `#[allow]`

## 测试「100%」

硬门槛：该单元 spec 每条验收都有自动化测试且全绿。库代码行覆盖目标 ≥ 95%（缺口写进 spec）。特权路径用 pcap 回放 / 假接口 / `privileged-tests`，CI 不依赖 root。

## 工程基础设施

- **Workspace**：`crates/nz-net`（库）+ `crates/nz-arg`（参数解析）+ `crates/nz`（CLI）+ `crates/nz-gui`（GUI）
- **CI**：GitHub Actions（`ci.yml` fmt/clippy/nextest/deny/typos/coverage，macOS + Linux 矩阵）；Dependabot 每周扫 cargo 与 Actions
- **Release**：git-cliff changelog + GitHub Release（tag `v*` 触发）；release-plz 手动触发 bump PR
- **本地钩子**：pre-commit 是唯一本地质量门（fmt → clippy → nextest → deny → typos；每次 commit 全跑）
- **供应链**：cargo-deny（许可证 + 安全公告 + 来源审计）
- **覆盖率**：CI `coverage` job 对库 crate 强制 `--fail-under-lines 95`（`nz-net` + `nz-arg`；工具 0 / GUI 完成后按 crate 扩展）；Codecov 仅报告。本地 `cargo llvm-cov` 可选，不进 pre-commit
- **格式**：rustfmt（stable 配置）
- **Commit 规范**：Conventional Commits（`feat/fix/docs/refactor/test/ci`）

## 文档地图

- `doc/` 对照摘录与缺口（含 `doc/nz-arg/` 解析器设计）
- `spec/` 任务说明（库 / `nz-arg` / 工具 / GUI）
- `skills/README.md` 技能索引；正文在 `.cursor/skills/`（含 `nz-arg-parser`）
- `README.md` 仓库首页（英文默认）；`README.zh.md` 中文。两份改动必须同步
