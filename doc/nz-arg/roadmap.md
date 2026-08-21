# nz-arg 实现路线

| 步 | 交付 | 验收 |
|----|------|------|
| D0 | 文档包 | 用户确认 ✓ |
| M1 | `crates/nz-arg` MVP（含前缀/argfile/MORE） | `spec/nz-arg/mvp.md` + 覆盖率 ≥95% |
| M2 | `nz` 接入 registry bool_triple / help2 | `spec/netwox/registry.md` |
| M3 | RADIO / Generate / 类型钩子 | `types-ext.md` |
| T0 | 工具 0 全契约 | `spec/netwox/000.md` |

## 后置（不做进 M1）

`--kbd` 交互、RADIO、Generate、derive、全 netwox 类型枚举校验。

## 风险

| 风险 | 缓解 |
|------|------|
| argfile 引号语义与 C 不完全一致 | 测常见路径；缺口写进 spec |
| 范围再膨胀 | 后置清单冻结；新增走决策协议 |
