# skills

协商面已关闭。正文在 `.cursor/skills/<name>/SKILL.md`。协议族 skill 仍按需再拆，不一工具一 skill。

## 流程（每次任务）

| name | 路径 | 何时用 |
|------|------|--------|
| `nz-workflow` | [`.cursor/skills/nz-workflow/SKILL.md`](../.cursor/skills/nz-workflow/SKILL.md) | 闸门、带方案提问、禁止抢跑 |
| `nz-spec-authoring` | [`.cursor/skills/nz-spec-authoring/SKILL.md`](../.cursor/skills/nz-spec-authoring/SKILL.md) | 按模板从 HTML/`000NNN.c` 写 spec |
| `nz-parity` | [`.cursor/skills/nz-parity/SKILL.md`](../.cursor/skills/nz-parity/SKILL.md) | 对照原参数、输出、pcap |
| `nz-testing` | [`.cursor/skills/nz-testing/SKILL.md`](../.cursor/skills/nz-testing/SKILL.md) | 验收、特权隔离、覆盖率 |
| `nz-git-feature` | [`.cursor/skills/nz-git-feature/SKILL.md`](../.cursor/skills/nz-git-feature/SKILL.md) | review 后一笔；文档闸结束编码前统一 push |

## 领域（实现时加载）

| name | 路径 | 何时用 |
|------|------|--------|
| `nz-netwib-map` | [`.cursor/skills/nz-netwib-map/SKILL.md`](../.cursor/skills/nz-netwib-map/SKILL.md) | dat/sys/net/pkt/shw → crate，禁止 C API |
| `nz-packet-codec` | [`.cursor/skills/nz-packet-codec/SKILL.md`](../.cursor/skills/nz-packet-codec/SKILL.md) | 以太/IP/TCP/UDP/ICMP/ARP 与校验和 |
| `nz-pcap-record` | [`.cursor/skills/nz-pcap-record/SKILL.md`](../.cursor/skills/nz-pcap-record/SKILL.md) | record 格式、DLT、重组 |
| `nz-cli-compat` | [`.cursor/skills/nz-cli-compat/SKILL.md`](../.cursor/skills/nz-cli-compat/SKILL.md) | 数字工具号、原参数名、help 树 |
| `nz-arg-parser` | [`.cursor/skills/nz-arg-parser/SKILL.md`](../.cursor/skills/nz-arg-parser/SKILL.md) | `nz-arg`：bool 三态、help2、formupdate；非 clap |
| `nz-privileges` | [`.cursor/skills/nz-privileges/SKILL.md`](../.cursor/skills/nz-privileges/SKILL.md) | raw socket、降权、授权目标 |
| `nz-tool0-protocol` | [`.cursor/skills/nz-tool0-protocol/SKILL.md`](../.cursor/skills/nz-tool0-protocol/SKILL.md) | GUI 与 CLI 的工具 0 契约 |
| `nz-gui-parity` | [`.cursor/skills/nz-gui-parity/SKILL.md`](../.cursor/skills/nz-gui-parity/SKILL.md) | lessons 工作流；egui only |

## 协议族（第 3 闸后再定是否拆）

可选：`nz-proto-dns`、`nz-proto-http`、`nz-proto-smb` 等。现在不拆。

## 明确不做

- Axum / SQLx skill
- 复制 `~/.cursor/skills` 里的通用 Rust skill
- Tcl/Tk 移植 skill
- webview / Tauri skill
