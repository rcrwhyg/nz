# nz-arg 设计

## 1. 目标与非目标

### 目标

- **解耦**：参数描述与解析不绑在 `nz` 二进制或某个工具里；`nz` / 工具 0 / 未来 GUI 共用。
- **语义**：对齐 netwox toolarg（见 [research.md](research.md)），不是对齐 clap API。
- **生产可用**：清晰错误、无 panic 解析路径、≥95% 行覆盖、零 warning、GPL-3 兼容。
- **MVP**：支撑注册表剩余验收 + 工具 0 参数面；能做的对齐能力尽量一次做全（含长名缩写、argfile、MORE）。

### 非目标（本阶段）

- 子命令树、derive 宏、shell 补全、RADIO、命令行 Generate、`--kbd` 真交互
- 依赖 `nz-net`（类型化 IP/Eth 放适配层）
- 1:1 复刻 clap 的 `Command`/`ArgMatches` 命名

## 2. Crate 边界

Crate 名：**`nz-arg`**（`crates/nz-arg`）——已确认。

```
nz-arg          # 纯解析核
  ↑
nz              # 登记表 + 工具描述表 + 调用
  ↑
nz-gui          # Form 消费同一描述/解析结果（第 6 闸）
```

`nz-arg` ↛ `nz-net`。地址等：`nz` 取出字符串后再调 `nz-net`。

```
nz-arg → thiserror（可选）
nz-net → thiserror
nz     → nz-net, nz-arg
```

## 3. 核心概念

### 3.1 `ArgSpec`

| 字段 | 含义 |
|------|------|
| `key` | 短选项字符（MORE 描述项无短键入口） |
| `long_name` | 长名（不可为 `help`/`kbd`/`argfile`） |
| `class` | Optional / Required / More |
| `value_kind` | Bool / String / U32 |
| `advanced` | 是否 Advanced |
| `default` | 可选默认字符串 |
| `help` | 说明 |

`More` 类只打开 `allow_more` 并携带帮助文案，不占 key 表。

### 3.2 `ArgSchema`

有序 specs + 校验（短键唯一、长名唯一、保留名拒绝）。

### 3.3 `ParseMode`

| 模式 | 行为 |
|------|------|
| `Cli` | 处理 `--help`/`--help2`/`--argfile`；`--kbd` → 明确错误 |
| `FormUpdate` | 抑制 help/argfile/kbd 特殊分支 |

### 3.4 `ParsedArgs`

- 按 `key`：`set_by_user` + 值
- 布尔与 default / isset 分离
- `more: Vec<String>`

### 3.5 `ParseOutcome`

```text
Parsed(ParsedArgs)
Help { include_advanced: bool }
```

库不打印帮助、不删文件。

## 4. MVP 功能集（本闸必须）

1. Schema 构建与校验（含保留名）
2. `--long` / `-k` 与取值
3. 布尔三态 + 短布尔连写
4. 长名无冲突前缀
5. `--help`/`--?`、`--help2`/`--??`
6. `--argfile`（空行/`#` 跳过；引号分词；并入当前解析）
7. `MORE` + `--` + 位置填 Required
8. `isset` / bool / string / u32
9. `FormUpdate` 抑制 help/argfile
10. Required 检查；稳定 `ParseError`
11. Cli 下 `--kbd` → `InteractiveNotSupported`

### 明确后置

- `--kbd` 真交互、RADIO、Generate、derive、全类型校验

## 5. API 草图

```rust
pub fn parse(
    schema: &ArgSchema,
    args: &[impl AsRef<str>], // 不含程序名
    mode: ParseMode,
) -> Result<ParseOutcome, ParseError>;
```

## 6. 错误 / 质量 / 许可

- `ParseError` + `thiserror`
- 行覆盖 ≥ 95%（CI 强制 `nz-arg`）
- `GPL-3.0-only`

## 7. 衔接

| 现有 | 衔接 |
|------|------|
| `nz::registry` | 工具 schema + bool_triple / help2 |
| 工具 0 | FormUpdate + toolhelp 元数据 |

## 8. 决策记录

- 2026-08-21：方案 A；crate 名 `nz-arg`；MVP 含长名缩写 / argfile / MORE；文档后立即实现。
