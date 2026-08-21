# nz-arg：命令行参数解析

独立 crate 规划：对齐 netwox `toolarg`/`arg` **语义**，作 nz 风格的生产可用解析器（clap 解耦替代，非 1:1 克隆 clap）。

| 文件 | 内容 |
|------|------|
| [research.md](research.md) | 对照调研（语法、类型、特殊开关、formupdate） |
| [design.md](design.md) | 架构、MVP 边界、分期、API 草图、与 nz/nz-net/GUI 关系 |
| [roadmap.md](roadmap.md) | 实现闸顺序与验收挂钩 |

实现 spec：[`spec/nz-arg/`](../spec/nz-arg/README.md)。
技能：`.cursor/skills/nz-arg-parser/SKILL.md`。
硬约束：`.cursor/rules/07-nz-arg.mdc`。

**状态**：设计草稿（draft）。未经用户批准不建 crate、不写业务解析代码。
