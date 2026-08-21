---
name: nz-arg-parser
description: >-
  Designs and implements nz-arg: netwox-style CLI argument parsing (bool
  triples, help2, formupdate) as a decoupled crate—not clap. Use when changing
  tool flags, tool 0 form/parse, or ArgSchema.
---

# nz-arg 解析器

加载时机：改工具参数、工具 0 form/formupdate、注册表 bool/help2、设计/实现 `nz-arg`。

## 先读

1. [`doc/nz-arg/research.md`](../../../doc/nz-arg/research.md)
2. [`doc/nz-arg/design.md`](../../../doc/nz-arg/design.md)
3. [`spec/nz-arg/mvp.md`](../../../spec/nz-arg/mvp.md)
4. 硬约束 [`.cursor/rules/07-nz-arg.mdc`](../../rules/07-nz-arg.mdc)

## 原则

- **语义对齐 netwox toolarg，API 不克隆 clap。**
- **MVP**（见 mvp spec）：含长名无冲突前缀、`--argfile`、`MORE`；后置见 `types-ext.md`。
- **库不打印帮助、不删文件**；I/O 与副作用在 `nz`。
- **`nz-arg` 不依赖 `nz-net`**；类型化地址在适配层。
- 布尔：`-x` / `+x` / `--name` / `--no-name`；连写仅布尔。
- `--help`/`--?` vs `--help2`/`--??` → `ParseOutcome::Help { include_advanced }`。
- `FormUpdate` 不进入 help/argfile/kbd 分支。
- Cli 下 `--kbd` → 明确「不支持」，不做假交互。

## 禁止

- 引入 clap 作主路径
- 为单个工具私有再写一套 argv 解析
- 把 GUI 文件删除副作用放进 `nz-arg`
- 把 RADIO/Generate/`--kbd` 交互偷塞进 MVP

## 实现后

- `cargo test -p nz-arg` 全绿；CI 行覆盖 ≥ 95%
- 再接到 `nz` 勾掉 registry 的 bool_triple / help2
