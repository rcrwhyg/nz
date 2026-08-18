# 工具 0（GUI 契约）

对照：`src/netwox-src/src/tools/000000.c`（无 `tools/0.html`）。
标题：Obtain information needed by netwag。
说明：非交互用户工具；netwag 通过它拉目录、帮助、表单、运行与配置。

`nz` 必须提供**同等契约**（参数字母与长名可映射）。输出原为 Tcl 片段；nz-gui 是 egui，**不要求输出 Tcl**，但每个开关的信息面必须可测。

## 参数

| 短 | 长 | 附带 | 作用 |
|----|----|------|------|
| `-t` | `--tools` | | 工具树 + stdin/backspace 需求表 |
| `-h` | `--toolhelp` | `-u` 工具号 | 该工具的 help / example / usage / form |
| `-f` | `--formupdate` | `-u` 工具号，`-b` 命令文件 | 把命令行解析回表单更新；读完删文件 |
| `-r` | `--run` | `-b` 命令文件 | 读文件当命令行并执行；读完删文件 |
| `-R` | `--run-key` | `-b` 命令文件 | 同 `--run`，另处理按键/前台运行细节 |
| `-k` | `--kill` | `-u`、`-b` | 结束已启动的工具进程；目标已死则忽略 |
| `-e` | `--error` | `-u` | 把错误码格式化为可读文本 |
| `-c` | `--conf` | | 导出 GUI 所需配置 |
| `-v` | `--version` | | 版本三元组 |
| `-b` | `--buf` | 字符串 | 文件路径或缓冲（run/formupdate/kill） |
| `-u` | `--uint` | u32 | 工具号或错误码 |

清单初稿漏了 `-e` / `-b` / `-u`，以本表为准。

## `--tools` 信息面

- 分类树（原 `tooltree`）
- 工具起始编号常量（原 `toolstart`）
- 需要 stdin 的工具号：7、14、87、88、89、90、99、152、171
- 需要 backspace 处理的工具号：138、139、210

## `--toolhelp` 信息面

对指定工具号输出：

- help 正文
- example 行
- usage 行
- form：普通参数 +（若有）Advanced parameters

## 文件副作用

`--formupdate` / `--run` / `--run-key` 在成功读到命令文件后会**删除该文件**。复刻时保持该副作用，或在 spec 里显式改为「调用方负责清理」并让 GUI 适配——实现前再拍板。

## 不在 HTML 工具列表

工具 0 不出现在 `netwox-doc_html/tools/`。测试以本文件 + 将来 `spec` 为准。
