---
name: nz-gui-parity
description: >-
  netwag workflow parity in native egui: Search, Form, Run, History, Clipboard,
  Local_info, Remote_info. Use when implementing nz-gui, lessons, or wiring
  tool 0. Never use webview/Tauri/Tcl-Tk.
---

# GUI 工作流

对照：`doc/netwag/workflows.md`、`spec/TEMPLATE-gui.md`。实现 crate：**egui**。

## 禁止

Tauri / Wry / webview / Tcl-Tk 移植。不对齐 Tk 颜色、字体、像素。

## 必须有的面

笔记本：Local_info / Remote_info / Clipboard / Tool。
Tool 内：Search / Help / Form / Running / History。
Local_info：Devices / Ip / Arp_cache / Routes（字段对齐工具 1/169；启动配置走工具 0 `-c`）。

| 面 | 工具 0 |
|----|--------|
| Search | `0 --tools` |
| Help | `0 -h -u N` |
| Form update | `0 -f -b FILE -u N` |
| Run 内嵌 | `0 -r -b FILE` |
| Run 新上下文 | `0 -R -b FILE` |
| Kill | `0 -k` |
| 版本 | `0 -v` |

Remote_info = 工具 3，不经过工具 0。

## 验收

- 课表步骤可完成（lessons 1–22 的工作流，不是像素）
- 契约测试走工具 0，不依赖截图
- stdin/backspace 工具在 Form/Run 里有对应处理

GUI 闸在库与 CLI（含工具 0）之后。未到第 6 闸不要实现界面。
