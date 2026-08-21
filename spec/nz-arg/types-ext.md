# 库能力 spec：nz-arg 扩展（非 MVP）

- 模块：nz-arg 后续能力
- 对照：同 [`mvp.md`](mvp.md)
- 状态：draft（**禁止在 MVP 闸实现**）

## 能力（规划）

1. RADIO1/2/3 互斥组
2. 命令行**生成**（ParsedArgs + Schema → argv，供 Form Generate）
3. `--kbd` 真交互（或 feature `interactive`）
4. 类型扩展钩子：IP/Eth/Port 等（可选 feature / `nz` 适配，先决策协议）
5. derive 宏（远期）

## 非目标

MVP 已含：长名前缀、`--argfile`、`MORE`。本文件不再重复。

## 验收

实现闸再拆独立 spec。

## 覆盖率

扩展模块各自 ≥ 95%。
