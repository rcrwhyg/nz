---
name: nz-cli-compat
description: >-
  Dual CLI for nz: numeric tool ids (nz 49) plus named subcommands, original
  flag letters, bool triples, and tool-12 enums. Use when adding a CLI tool,
  help text, argument parsing, or the static number-to-name map.
---

# CLI 兼容

对照：`doc/netwox/` 各摘录。具名子命令在摘录里是**建议名**，实现 CLI 闸再钉死并维护静态映射。

## 双模式

`nz 49` 与 `nz ping-icmp` 等价。数字模式覆盖 0–223（后置工具默认不编进发布二进制）。

## 参数

- 短选项字母与长名必须可映射到原工具。
- 布尔：`-x` / `+x` / `--no-x`。
- 「显示类」开关全关时常表示全开（如工具 1、3、134）；以该工具摘录为准。
- 难猜枚举（`recordencode`、`dlt`、`spoofip`、encode）与工具 12 一致；GUI 用控件枚举，CLI 仍接受这些字符串。

## 帮助

工具 0 `--toolhelp` 需要 help / example / usage / form（含 Advanced）。用户 `nz N --help` 可以现代化排版，但信息面要能对上 spec。

## 注册

新工具：登记号、建议名、树节点、是否 stdin、是否 backspace。禁止未注册就实现。stdin/backspace 名单见 `nz-tool0-protocol`。
