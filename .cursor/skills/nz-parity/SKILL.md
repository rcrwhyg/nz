---
name: nz-parity
description: >-
  Checks nz capability coverage and mappable semantics against netwox 5.39.0,
  not 1:1 cloning. Use when implementing a tool, comparing CLI flags, judging
  output format, or deciding whether a behavior change is allowed.
---

# 对照口径

产品是能力全集覆盖，不是逐项克隆。见 `AGENTS.md` 复刻口径。

## 必须可映射

- 工具号 0–223（后置 20 个除外，未批准不实现）
- 参数长名/短选项/默认值/互斥关系
- 协议行为（校验和自动 vs 字面、UDP 开口无回包、DHCP 接受首个应答等）
- 工具 0 信息面（不必输出 Tcl）

对照摘录：`doc/netwox/`、`doc/netwib/`、`doc/netwag/workflows.md`。有冲突时：C 参数表 > HTML > 摘录，并修正摘录。

## 允许不同

- 命令组织（双模式：`nz 49` 与 `nz ping-icmp`）
- 内部架构（Rust 类型，不复制 `netwib_*`）
- 屏幕排版（字段必须还在）
- GUI 控件（egui，不对齐 Tk 像素）

## 禁止

- 对齐完成前用「更现代的默认」改旧语义
- 为单个工具私有一套编解码（走 `nz-net`）
- 漏工具却称覆盖完成（工具 218 是测试替代，见 `doc/netwox/info.md`）
- 把玩笑工具 190 删掉

## 检查步骤

1. 打开对应 `doc/netwox/*.md` 与 spec。
2. 列出参数与默认，和实现逐项打勾。
3. 用 record/假接口对关键路径，而不是目测。
4. 缺口写进 spec，不要暗改生产行为来凑覆盖率。
