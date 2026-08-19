---
name: nz-spec-authoring
description: >-
  Writes nz specs from HTML and 000NNN.c using TEMPLATE-lib, TEMPLATE-tool, or
  TEMPLATE-gui. Use when authoring or filling spec/, mapping a netwox tool, a
  netwib capability, or a netwag workflow into an implementable unit.
---

# 写 spec

没有对应 spec 不写业务代码。本闸填模板，不要一次写 223 份。

## 选模板

| 单元 | 模板 | 落地示例 |
|------|------|----------|
| 库能力 | `spec/TEMPLATE-lib.md` | `spec/netwib/pkt/ipv4.md` |
| 单个工具 | `spec/TEMPLATE-tool.md` | `spec/netwox/info/001.md` |
| GUI 工作流 | `spec/TEMPLATE-gui.md` | `spec/netwag/search.md` |

相位与族顺序：`spec/_index.md`。后置审计只允许草稿，见 `spec/netaudit/_deferred.md`。

## 金标准

1. 先读 `doc/` 摘录（已规范化）。
2. 再核 `netw-ib-ox-ag-5.39.0/src/netwox-doc_html/tools/N.html`。
3. 默认值、互斥 radio、高级字段以 `src/netwox-src/src/tools/000NNN.c` 与 `toolarg.c` 为准。
4. 库对照 `netwib-doc_html/` + 头文件**能力**，不抄 C 符号。

禁止把 C 大段贴进 spec。禁止修改对照树。

## 必填

- 状态：`draft` → 用户批准后 `approved` → 实现后 `implemented`；后置用 `deferred`。
- 行为：可测的语义，不是 UI 排版。
- CLI：Usage + 参数表（长名、短选项、类型、默认、含义）。布尔是 `-x|+x|--no-x`。
- 验收：每条将来都有自动化测试；库 spec 写覆盖率目标 ≥ 95% 与缺口。
- 非目标：本单元明确不做的事。
- 依赖：库 spec 名或底层能力。

具名子命令是建议名，CLI 闸再钉死；spec 里同时写工具号与建议名。

## 顺序

先库能力 spec，再工具族，再 GUI。工具 spec 必须指向已有（或本任务一并写的）库 spec。
