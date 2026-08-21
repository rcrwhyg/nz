---
name: nz-workflow
description: >-
  Enforces nz gate order, one-feature tasks, and decision protocol (options before
  writing). Use when starting a task, changing scope, skipping a gate, implementing
  a tool or GUI, adding dependencies, or when the user says continue to the next
  phase.
---

# nz 工作流

每次任务先读 `AGENTS.md` 与 `spec/_index.md`。细则在 `.cursor/rules/00-decision-protocol.mdc`、`02-workflow.mdc`。

## 闸门

0 骨架 → 1 抽文档 → 2 skills 正文 → 3 spec → 4 crate/CLI/工具 0 → 5 按族实现 → 6 GUI。

**未经用户确认不进入下一闸。** 当前为第 5 闸：按族实现用户工具（一次一工具）。禁止 GUI、后置审计；运行时依赖须先商量。第 4 闸（注册表 / `nz-arg` / 工具 0）已完成。

对齐 5.39.0 完成前，禁止用「现代化」改旧工具语义。现代协议只进 `spec/modern/`。

## 一次任务

一个可独立验收、可 git 回退的单元（一块库能力、一个工具、一条 GUI 工作流）。不要整族一次做完。

完成才算完：范围内 spec 存在 → 实现 + 自动化测试 → `cargo test` 全绿 → 零 warning → pre-commit → 用户 review → 确认后才提交。第 1–3 闸只本地提交。

## 决策

非琐碎决策（范围、crate、协议语义、GUI、后置工具、许可证）必须先问：

1. 问题（一句话）
2. 2～3 个方案
3. 每个优缺点
4. 推荐与理由

等确认再落盘。琐碎（错别字、已拍板约束的机械执行）可直接做，并在结果里说明。

## 依赖方向

库（`nz-net`）→ CLI（`nz`）→ GUI（`nz-gui`）。禁止库能力未定义时堆工具特例。禁止为单个工具复制一套私有编解码。

## 后置

工具 73–86、98、101、130–132 见 `spec/netaudit/_deferred.md`。未单独批准：`src/` 不得有实现。
