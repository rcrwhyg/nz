---
name: nz-testing
description: >-
  Defines nz test hard gates: spec acceptance coverage, 95% lib line coverage,
  no live NICs in CI, privileged-tests feature. Use when writing tests, claiming
  done, running cargo test, or touching sniff/spoof/scan paths.
---

# 测试

细则：`.cursor/rules/03-testing.mdc`。

## 硬门槛

该单元 spec **每条验收**都有自动化测试，且 `cargo test` / pre-commit 里的 nextest 全绿。没有全绿不得提交、不得宣称完成。

库代码（编解码、校验和、参数解析、工具 0 协议）：行覆盖目标 ≥ 95%。**CI 强制**（现阶段 `-p nz-net --fail-under-lines 95`）；本地可选，不进 pre-commit。

```bash
cargo llvm-cov -p nz-net --summary-only --fail-under-lines 95
```

## 特权 / 真网

默认**不打真网卡**。用：

- pcap / record 回放
- 用户态假接口
- `privileged-tests` feature（本地可选）

CI 不依赖 root，不对公网。嗅探/伪造/扫描仅本机、实验室或书面授权目标。

## GUI

测工作流与工具 0 契约，不以像素截图为主验收。

## 禁止

- 为冲覆盖率改生产行为
- 无断言的空测试
- 把手工步骤当成「已覆盖」
- 无 spec 理由的 `#[allow]`

## 命令

合并或提交前：

```bash
cargo test
cargo clippy --all-targets -- -D warnings
# 可选：cargo llvm-cov -p nz-net --summary-only --fail-under-lines 95
```

本地质量门是 pre-commit（已 `pre-commit install`）。不要另写平行 sh 脚本。
