# netwox toolarg 调研

对照源（只读）：`netw-ib-ox-ag-5.39.0/src/netwox-src/src/modules/tool/{toolarg,arg,toolargstore}.*`。
仓库摘录：`doc/netwox/`、`spec/netwox/registry.md`、`doc/netwox/tool0.md`。

本文只记**行为与约束**，不贴大段 C。

## 1. 职责划分（C 侧）

| 模块 | 职责 |
|------|------|
| `toolarg.h` | 参数**描述表**：类型、必选/可选、短键、长名、说明、默认、是否 advanced、MORE |
| `arg.c` / `arg.h` | **解析** argv → 按 key 存值；isset；help/argfile/kbd；formupdate 模式 |
| `toolargstore` | 类型化存储与从字符串赋值 |
| 各 `000NNN.c` | 静态 `toolarg[]` 表 + 读 `arg_*` |

nz 应对齐：**描述表 + 解析 + isset/取值 + 帮助分流 + formupdate**，不复制符号名。

## 2. 表面语法（金标准）

摘自 `toolarg.h` 注释：

```
--long value
--lo value          # 长名无冲突前缀缩写
-k value
```

仅布尔：

```
--long / --lo / --no-long
-k / -jk            # 多个布尔短选项可连写
+k / +jk            # 显式关
```

另（`arg.c`）：

| Token | 行为 |
|-------|------|
| `--help` / `--?` | 普通帮助（不含 Advanced），解析以特殊错误结束 |
| `--help2` / `--??` | 含 Advanced 的帮助 |
| `--argfile FILE` | 读文件拼进 argv 再解析（注释/空行规则同 conffile） |
| `--kbd` | 交互补全未给参数 |
| `--kbd-X` / `--kbd-name` | 交互补全单个参数 |

`formupdate` 模式下：**不处理** help / argfile / kbd（避免 GUI 回填时弹出帮助或删文件副作用搞乱）。

## 3. 参数类与类型

### 3.1 类（class）

- `OPT`：可选
- `REQ`：必选（未出现且非 formupdate 时失败或走 kbd）
- `MORE`：尾部变长位置参数（`TOOLARG_MORE`）
- `END`：表结束

### 3.2 类型（type）摘要

- **布尔**：`BOOL`；GUI 互斥用 `BOOL_RADIO1/2/3`
- **地址**：`IP` / `IP4` / `IP6` 及 `_SRC`/`_DST`；`ETH` 及 src/dst；`PORT` 及 src/dst
- **集合**：`IPS_*` / `ETHS_*` / `PORTS_*`（排序/去重变体 SU、U、N）
- **整数**：`UINT32`/`INT32`/`UINT64`/`INT64`
- **缓冲**：通用 `BUF` + 语义变体（login、password、device、file、dir、filter、oid、mixed、email、uri、…）
- **枚举**：`ENCODE`、`DECODE`、`RECORDENCODE`、`DLT`、`SPOOFIP`、`IPTYPE`

默认值字符串常由类型推导；`NULL` 表示「用类型默认」或「无默认」。

### 3.3 Advanced

`OPTA` / 描述里 `advanced=true` → 只在 `--help2` 与 form 的 Advanced 分组出现。

## 4. 解析状态机（行为级）

对每个 argv（跳过 `argv[0]`）：

1. `--…` → 长选项分支（help / argfile / kbd / `--no-` / 布尔开关 / 取值）
2. `+…` → 仅布尔关（可连写）
3. `-…` → 短选项：单布尔、布尔连写、或 `-k value`
4. 其它 → MORE（若允许）否则错误

关键细节：

- 长名匹配：精确或**无歧义前缀**（`arg_key_init_name_string`）
- 布尔默认：描述默认或关；**未出现**与**显式关**可区分（`setbyuser`）
- 取值选项：下一 argv 为值；缺值则错误
- MORE：剩余 argv 收集为列表

## 5. 与工具 0 / GUI 的接点

| 能力 | 依赖 toolarg |
|------|----------------|
| `--toolhelp` form | 描述表 → 普通/Advanced 参数元数据 |
| `--formupdate` | `formupdate=true` 解析命令文件参数；只回填用户显式项 |
| Form Generate | 解析结果 → 命令行字符串（逆操作） |
| Run 行 | 与 CLI 同一解析器 |

## 6. 与 clap / 常见库的差距（调研结论）

| 需求 | clap 4 典型能力 | 差距 |
|------|-----------------|------|
| `+k` 关布尔 | 无一等支持 | 高 |
| 布尔三态（未设/开/关） | 多为二态 | 高 |
| `--help2` / `--??` | 自定义困难 | 中 |
| formupdate 抑制 help/argfile | 需旁路 | 中 |
| Advanced 分组元数据 | 无内建 | 中 |
| 长名无冲突缩写 | `infer_long_args` 近似 | 中 |
| 与 GUI 同构描述表 | 需自建一层 | 高 |

结论：自研核（方案 A）合理；clap 不适合作 nz 主路径。

## 7. 非目标（对照源有、nz-arg MVP 可不做）

- 完整 `--kbd` 交互（可用假键盘或工具侧延后）
- 密码隐藏输入等 TTY 细节
- 把全部 `BUF_*` 语义校验一次做完（可分期；MVP 先当字符串）
- 复制 `netwox_arg_*` C API

## 8. 参考文件指针

- `toolarg.h`：语法注释、类型枚举、宏
- `arg.c`：`netwox_arg_update_argcargv_*`
- `spec/netwox/registry.md`：共用参数语法摘要
- `doc/netwox/tool0.md`：formupdate / 命令文件
