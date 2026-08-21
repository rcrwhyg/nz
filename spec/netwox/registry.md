# CLI 注册表 spec

- 模块：`nz` 分发与工具登记
- 对照：`netwox.c`、`modules/tool/{tool,toolarg,tooltree,arg}.h`、`doc/netwox/`
- 相位：`spec/_index.md` 第 2 项
- 状态：partial（登记表+分发已实现；bool 三元组/`--help2` 阻塞于 [`nz-arg` MVP](../nz-arg/mvp.md)）

## 能力

静态登记每一个可调用工具，并按双模式分发。禁止未登记就实现。

每条登记至少：

| 字段 | 含义 |
|------|------|
| 号 | `0`–`223` |
| 标题 | 原 toollist 标题 |
| 建议名 | 具名子命令；各族 spec 填写，CLI 闸钉死 |
| 树节点 | Search 分类，可多属 |
| stdin | 是否需要 stdin（工具 0 `--tools`） |
| backspace | 是否按退格擦除输出（工具 0 `--tools`） |
| 发布 | `release` / `deferred` / `hidden` |
| spec | 对应工具 spec 路径 |

发布规则：

- **release**：默认编进二进制，出现在 `nz 0 --tools`
- **deferred**：后置审计/暴力破解（见 [netaudit/_deferred.md](../netaudit/_deferred.md)），默认不编进发布二进制，也不出现在默认 `--tools`
- **hidden**：不作为用户工具列出。工具 0 可 `nz 0` 调用但不进 Search；工具 218 用 `cargo nextest` 替代，**不**提供 `nz 218`

### 双模式

`nz 49` 与 `nz ping-icmp` 等价。数字优先：第一参数能整段解析为无符号整数则走工具号，否则走建议名。

原版无参数会进键盘选工具。nz：**无参数或 `--help` 打印总目录**（号 + 标题 + 建议名），不复刻交互树。

未知号或未知名：失败，属「未登记」。

### 共用参数语法（所有工具）

对照 `toolarg.h`：

- 长名 `--long`、无冲突时可缩写前缀；短名 `-k`
- 布尔：`-x` / `+x` / `--no-x`（及 `--x` 表示开）。多个布尔短选项可连写（`-jk`）
- `--help`：普通帮助；`--help2` / `--??`：含 Advanced
- `--argfile FILE`：把文件当额外 argv（注释行与空行规则同工具 0 命令文件）
- `--kbd`：交互补全未给参数（测试用假键盘或不测）

解析时 **argv[0] 当作程序名跳过**；因此 `nz 49 -i 1.2.3.4` 里工具侧看到的是 `-i …`。

布尔未出现且工具 spec 未另写「全关当全开」时，默认关。

难猜枚举（`recordencode`、`dlt`、`spoofip`、encode）与工具 12 一致；取值在各族 spec 写。

### Search 树

对照 `tooltree.c` 顶层：information、network protocol、application protocol、sniff、spoof、record、client、server、ping、traceroute、scan、netaudit、bruteforce、remadm、not-network。子节点与交叉挂载实现时按该文件展开。工具 0 的 `nodes` 为空，不进树。

原 `toolstart=1000` 只是 C 把工具号编进树节点的偏移，nz **不**要求保留 1000。

## Rust 形状

`nz` crate 内编译期表（或等价）：`ToolId` → 元数据 + 入口。建议名反向映射同一入口。禁止 `netwox_*` 符号。本闸不加运行时依赖。

## 非目标

- 不在本 spec 实现任何工具体
- 不输出 Tcl
- 不把后置工具编进默认二进制
- 不复刻无参数时的键盘选单

## 验收

- [x] `dispatch_numeric_eq_named`：同一工具数字与建议名走到同一入口（可用工具 0 或登记桩）
- [x] `dispatch_unknown_id_fails`：`224` 或未登记名失败
- [ ] `bool_triple_parses`：`-t`、`+t`、`--no-tools` 可区分开关（阻塞于 `nz-arg` MVP）
- [ ] `help_and_help2_flags_exist`：`--help` / `--help2` 被识别为帮助，不进业务 argv（阻塞于 `nz-arg` MVP）
- [x] `registry_stdin_backspace_lists`：stdin 为 7、14、87–90、99、152、171；backspace 为 138、139、210
- [x] `registry_hides_zero_and_218`：`--tools` 不含 0 与 218
- [x] `registry_omits_deferred_by_default`：默认表不含 73–86、98、101、130–132

## 覆盖率

分发与登记属 CLI；本分期覆盖登记/分发路径。`bool_triple` / `help2` 待 `nz-arg` MVP 接入后计入。

## 依赖

[000.md](000.md)（工具 0 契约）、[err.md](../netwib/err.md)
