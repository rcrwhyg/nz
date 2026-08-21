# nz-arg（CLI 参数解析 crate）

独立 workspace 库：对齐 netwox toolarg **语义**，供 `nz` CLI、工具 0、未来 GUI Form 共用。

设计摘录：[`doc/nz-arg/`](../../doc/nz-arg/README.md)。

| Spec | 内容 | 状态 |
|------|------|------|
| [mvp.md](mvp.md) | 最小可用解析核 | draft |
| [types-ext.md](types-ext.md) | 类型扩展与 argfile/MORE（非本闸实现） | draft |

**依赖方向**：`nz-arg` ↛ `nz-net`；`nz` → `nz-arg`。

**状态**：MVP implemented（见 `mvp.md`）。
