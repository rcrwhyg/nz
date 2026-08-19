# skills 草案

本目录是协商面。确认后把 `SKILL.md` 写到 `.cursor/skills/<name>/`，并在此更新路径。

**本闸不写 SKILL.md 正文。** 请勾选：要 / 不要 / 合并。

推荐默认：流程 5 个全要；领域前 7 个全要；协议族按需再拆。

## 流程（每次任务）

| 勾选 | name | 何时用 |
|------|------|--------|
| 要 / 不要 / 合并 | `nz-workflow` | 闸门、带方案提问、禁止抢跑 |
| 要 / 不要 / 合并 | `nz-spec-authoring` | 按模板从 HTML/`000NNN.c` 写 spec |
| 要 / 不要 / 合并 | `nz-parity` | 对照原参数、输出、pcap |
| 要 / 不要 / 合并 | `nz-testing` | 验收、特权隔离、覆盖率解释 |
| 要 / 不要 / 合并 | `nz-git-feature` | 提交前 review；确认后一笔；同笔可 amend（含 CI 失败后 force-with-lease）；文档闸结束编码前统一 push；跨功能不 amend；须 `pre-commit install`；不 add 对照树 |

## 领域（实现时加载）

| 勾选 | name | 何时用 |
|------|------|--------|
| 要 / 不要 / 合并 | `nz-netwib-map` | 把 dat/sys/net/pkt/shw 映射到 crate，禁止复制 C API |
| 要 / 不要 / 合并 | `nz-packet-codec` | 以太/IP/TCP/UDP/ICMP/ARP 编解码与校验和 |
| 要 / 不要 / 合并 | `nz-pcap-record` | record 格式、DLT、重组 |
| 要 / 不要 / 合并 | `nz-cli-compat` | 数字工具号、原参数名、help 树 |
| 要 / 不要 / 合并 | `nz-privileges` | raw socket、macOS 权限、降权 |
| 要 / 不要 / 合并 | `nz-tool0-protocol` | GUI 与 CLI 的工具 0 契约 |
| 要 / 不要 / 合并 | `nz-gui-parity` | 按 lessons 验收工作流；native crate only |

## 协议族（下一闸再定是否拆）

可选：`nz-proto-dns`、`nz-proto-http`、`nz-proto-smb` 等。不一工具一 skill。

## 明确不做

- Axum / SQLx skill（本项目不是 Web 后端）
- 复制 `~/.cursor/skills` 里的通用 Rust skill
- Tcl/Tk 移植 skill
- webview / Tauri skill
