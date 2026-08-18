# 库能力 spec

- 模块：（err / dat / sys / net / pkt / shw 之一或其子集）
- 对照：`netwib-doc_html/` 路径 + 头文件
- 状态：draft | approved | implemented

## 能力

原 netwib 做什么（行为，不是函数名）。

## Rust 形状

拟用 crate / 本仓库类型。禁止复制 `netwib_*` 符号。

## 非目标

明确不搬的 C 细节（例如 buf 内部布局）。

## 验收

- [ ] 行为 1 + 测试名
- [ ] 行为 2 + 测试名

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：

## 依赖

本能力依赖的更底层 spec：
