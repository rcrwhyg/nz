# TELNET 共用字段

套接字可选字段见 [transport/_common.md](../transport/_common.md)。对照 [doc/netwox/proto-misc.md](../../../doc/netwox/proto-misc.md) TELNET 段。

## 客户（99、100）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--dst-ip` | `-i` | （必填） | 服务器 |
| `--dst-port` | `-p` | `23` | TCP 端口 |

99 额外：`--line-by-line`（`-L`）缺省关；**registry stdin**（交互式键盘 → 网络）。

100 额外：`--login`（`-l`）必填、`--password`（`-w`）必填、`--timeout`（`-T`）`60000` ms；尾部 `cmd1 cmd2 …`（`TOOLARG_MORE`）。

## 服务端（170）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--src-port` | `-P` | `23` | 监听 |
| `--login` | `-l` | | 可选认证 |
| `--password` | `-w` | | |
| `--timeout` | `-T` | **`180000`** | 会话 ms |
| `--allowed-clients` | `-c` | `all` | 允许客户 |
| `--line-by-line` | `-L` | 关 | 行模式 |

需 port≤1024 特权。

## 非目标

- 后置 101（brute）不在本族
- 仅授权实验室
