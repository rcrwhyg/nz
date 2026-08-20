# nz

[English](README.md)

Rust 复兴 Laurent Constantin 的 **netwib + netwox + netwag 5.39.0** 能力全集，并在对齐完成后扩展现代协议。

仓库：<https://github.com/rcrwhyg/nz>
许可证：[GPL-3.0-only](LICENSE)

## 三层产品

| 层 | 原组件 | crate | 说明 |
|----|--------|-------|------|
| 库 | netwib | `nz-net` | 网络能力，不复制 C API |
| CLI | netwox | `nz` | 数字工具号 + 具名子命令（双模式） |
| GUI | netwag | `nz-gui` | native **egui** 工作台，不用 webview |

当前处于第 4 闸：crate 骨架、CLI 注册表、工具 0。尚未实现用户工具 1–223 与 GUI。

`README.md`（英文，默认）与 `README.zh.md` 必须同步更新。

## 构建

需要 Rust stable（见 `rust-toolchain.toml`）。

```bash
cargo build
pre-commit install
pre-commit run --all-files
rustup component add llvm-tools-preview
cargo binstall cargo-llvm-cov -y
cargo llvm-cov -p nz-net --summary-only --fail-under-lines 95
```

对照源 `netw-ib-ox-ag-5.39.0/` 仅本地只读，不进 git。

## 授权使用

嗅探、伪造、扫描仅用于本机、实验室或书面授权目标。

## 文档

- [AGENTS.md](AGENTS.md) — 范围、闸门、约束
- [CONTRIBUTING.md](CONTRIBUTING.md) — 贡献与质量门
- [SECURITY.md](SECURITY.md) — 漏洞报告
- [doc/](doc/README.md) — 对照摘录
- [spec/](spec/README.md) — 任务说明
- [skills/](skills/README.md) — 技能索引
