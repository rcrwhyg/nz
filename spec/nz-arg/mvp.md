# 库能力 spec：nz-arg MVP

- 模块：CLI 参数解析（对齐 netwox `toolarg`/`arg` 能力子集）
- 对照：`modules/tool/toolarg.h`、`arg.c`、`conffile.c`（argfile 行规则）、`cmdline.c`（引号分词）
- 摘录：[`doc/nz-arg/research.md`](../../doc/nz-arg/research.md)
- 状态：approved（用户确认 crate 名 `nz-arg` 与扩大 MVP）
- Crate：`nz-arg`

## 能力

可复用的参数**描述表**与 **argv 解析**，行为对齐 netwox：

- 短/长选项；布尔三态（`-x` / `+x` / `--no-x`）；短布尔连写
- 长名**无冲突前缀**缩写（精确优先；多匹配则冲突错误）
- `--help`/`--?` 与 `--help2`/`--??` 分流（库返回结果，不打印）
- `--argfile PATH`：读文件（跳过空行与 `#` 注释行），拼成命令行再解析并入当前结果
- `MORE`：`--` 之后或全部 required 已满足后的尾部位置参数列表
- 位置参数填槽：非选项 token 优先填第一个未设置的 `Required`
- `isset`（用户是否显式设置）与取值（bool / string / u32）
- `FormUpdate`：抑制 help / argfile / kbd 特殊分支
- Required：Cli 模式下未设置且无默认 → 错误
- 保留长名：`help` / `kbd` / `argfile` 不可作为普通参数名

不复制 C API；不依赖 `nz-net`。

## Rust 形状

- `ArgSpec` / `ArgSchema` / `ParseMode` / `ParsedArgs` / `ParseOutcome` / `ParseError`
- `parse(schema, args, mode) -> Result<ParseOutcome, ParseError>`（`args` **不含**程序名）
- MVP `value_kind`：`Bool`、`String`、`U32`

详见 [`doc/nz-arg/design.md`](../../doc/nz-arg/design.md)。

## 非目标（本闸后置）

- `--kbd` / `--kbd-*` 交互补全（遇到时返回明确错误，不假装成功）
- RADIO 组语义、命令行 Generate、derive 宏、子命令框架
- 全部 BUF/IP/Eth/Port 类型校验（字符串取出后由 `nz`/`nz-net` 处理）
- 在库内渲染帮助正文或删除命令文件
- clap 兼容 API

## 验收

- [ ] `schema_rejects_duplicate_keys` — 重复短键或长名构建失败
- [ ] `schema_rejects_reserved_long_names` — `help`/`kbd`/`argfile` 拒绝
- [ ] `parse_long_string_and_u32` — `--name val` / `-k val` 正确
- [ ] `bool_triple_short_long` — `-x` 开、`+x` 关、`--flag` 开、`--no-flag` 关；`isset` 为真
- [ ] `bool_cluster_short` — `-ab` / `+ab` 仅当 a,b 皆布尔
- [ ] `long_name_unique_prefix` — `--dev` 匹配唯一前缀 `device`
- [ ] `long_name_ambiguous_prefix_errors` — 前缀命中多个长名 → Err
- [ ] `help_flags_basic` — `--help` 与 `--?` → `Help { include_advanced: false }`
- [ ] `help_flags_advanced` — `--help2` 与 `--??` → `Help { include_advanced: true }`
- [ ] `formupdate_ignores_help` — FormUpdate 下 `--help` → 未知长名错误（不进入 Help）
- [ ] `formupdate_ignores_argfile` — FormUpdate 下 `--argfile` 不当作加载文件
- [ ] `required_missing_errors` — Cli 下缺必选且无默认 → Err
- [ ] `unknown_option_errors` — 未知 `-z` / `--zzz` → Err
- [ ] `value_option_missing_value_errors` — `-k` 缺后续值 → Err
- [ ] `more_after_double_dash` — `--` 后收集 MORE
- [ ] `more_after_required_satisfied` — required 已满后的裸 token 进 MORE
- [ ] `positional_fills_required` — 裸 token 填第一个未设 Required
- [ ] `argfile_loads_and_merges` — 文件内选项合并进结果；`#` 与空行忽略
- [ ] `argfile_quoted_tokens` — 引号保护含空格的值
- [ ] `kbd_not_supported_errors` — Cli 下 `--kbd` → 明确错误

（接入 `nz` 后：）

- [ ] `registry_bool_triple_parses` — 见 `spec/netwox/registry.md`
- [ ] `registry_help_and_help2_flags_exist` — 同上

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：无。

## 依赖

- 可选：`thiserror`（与 `nz-net` 同 workspace 依赖，已批准用于错误类型）
- 无其它 nz crate

## 约束交叉引用

- `.cursor/rules/07-nz-arg.mdc`
- `.cursor/skills/nz-arg-parser/SKILL.md`
