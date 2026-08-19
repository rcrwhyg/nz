---
name: nz-tool0-protocol
description: >-
  Tool 0 GUI contract: --tools/--toolhelp/--formupdate/--run/--kill/--conf and
  stdin/backspace tool lists. Use when implementing nz 0, nz-gui Search/Form/Run,
  or marking a tool as needing stdin.
---

# 工具 0

对照：`doc/netwox/tool0.md`（无 `0.html`，以 `000000.c` 为准）。非交互用户工具。

`nz` 必须提供同等契约（短/长名可映射）。原输出是 Tcl 片段；egui **不要求 Tcl**，每个开关的信息面必须可测。

## 开关

| 短 | 长 | 附带 | 作用 |
|----|----|------|------|
| `-t` | `--tools` | | 树 + stdin/backspace 表 |
| `-h` | `--toolhelp` | `-u` | help / example / usage / form |
| `-f` | `--formupdate` | `-u` `-b` | 命令行→表单；读完删文件 |
| `-r` | `--run` | `-b` | 读文件当 argv 并执行；读完删文件 |
| `-R` | `--run-key` | `-b` | 同 run，另处理按键 |
| `-k` | `--kill` | `-u` `-b` | 杀进程；已死则忽略 |
| `-e` | `--error` | `-u` | 错误码文本 |
| `-c` | `--conf` | | GUI 启动配置 |
| `-v` | `--version` | | 版本三元组 |
| `-b` | `--buf` | 字符串 | 路径/缓冲 |
| `-u` | `--uint` | u32 | 工具号或错误码 |

## 名单（`--tools`）

需要 stdin：7、14、87、88、89、90、99、152、171。
需要 backspace：138、139、210。

## 副作用

`--formupdate` / `--run` / `--run-key` 成功读到命令文件后**删除该文件**。若改为「调用方清理」，必须写进 spec 并让 GUI 适配——实现前再拍板。

Local_info 走 `0 -c`，不是直接跑工具 1。Remote_info 走工具 3。
