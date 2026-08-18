# nz

Rust 复刻 Laurent Constantin 的 **netwib + netwox + netwag 5.39.0**，再扩展现代协议。

对照源（只读、不进 git）：`netw-ib-ox-ag-5.39.0/`。细则见 `.cursor/rules/`。

## 三层产品

| 层 | 原组件 | nz | 对齐对象 |
|----|--------|-----|----------|
| 库 | netwib | 计划中的 `nz-net` | `dat/sys/net/pkt/shw` **能力**，不复制 C API |
| CLI | netwox | `nz` | 223 个工具的行为 + **工具 0**（GUI 契约） |
| GUI | netwag | 计划中的 `nz-gui` | lessons 工作流；**仅 native crate** |

依赖方向：库 → CLI → GUI。禁止在库能力未定义时堆工具特例。

## 闸门

0. 骨架（本文件与 `doc/` `spec/` `skills/`）— 当前
1. 从 HTML/源码抽详细文档
2. 写 `.cursor/skills/` 正文
3. 按模板写 spec（先库，再工具族）
4. crate + CLI 注册表 + 工具 0
5. 按族实现工具
6. native GUI

未经确认不进入下一闸。对齐完成前禁止用「现代化」改旧工具语义。

## 必须做

- 非琐碎决策：问题 + 2～3 方案 + 优缺点 + 推荐；等确认再落盘
- 一次任务 = 一个可验收完整功能（测试全绿、零 warning、一次 git 提交）
- review 意见落地后再提交一次，不 amend 上一笔
- `cargo build` / `cargo test` / `cargo clippy --all-targets -- -D warnings` 零告警
- 嗅探/伪造/扫描仅本机、实验室或书面授权目标

## 禁止

- 当前闸：写业务代码、加运行时依赖、实现任何工具或 GUI
- `git add netw-ib-ox-ag-5.39.0/`
- 修改对照源；把 C 大段贴进 `doc/`
- GUI 使用 Tauri / Wry / webview
- 未单独批准就实现后置审计工具（见 `spec/netaudit/_deferred.md`）
- 针对未授权系统编写利用过程；对公网做集成测试
- 为冲覆盖率改生产行为；无 spec 理由的 `#[allow]`

## 测试「100%」

硬门槛：该单元 spec 每条验收都有自动化测试且全绿。库代码行覆盖目标 ≥ 95%（缺口写进 spec）。特权路径用 pcap 回放 / 假接口 / `privileged-tests`，CI 不依赖 root。

## 文档地图

- `doc/` 对照摘录与缺口
- `spec/` 任务说明（库 / 工具 / GUI）
- `skills/README.md` 技能草案（正文确认后放 `.cursor/skills/`）
