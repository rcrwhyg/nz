# SMTP 共用字段

套接字可选字段见 [transport/_common.md](../transport/_common.md)。对照 [doc/netwox/proto-misc.md](../../../doc/netwox/proto-misc.md) SMTP 段。

## 客户（106、177、223）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--dst-ip` | `-i` | （必填） | SMTP 服务器 |
| `--dst-port` | `-p` | `25` | 端口 |
| `--timeout` | `-T` | **106/223：`180000`**；**177：`60000`** | ms |

## 发信/转发邮件头与信封（106、223）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--from` | `-f` | `user@example.com` | From 头（必填） |
| `--from-name` | `-n` | | 显示名 |
| `--to` | `-r` | `user2@example.fr` | To 头（必填） |
| `--subject` | `-S` | `hello` | Subject（必填） |
| `--mail-from` | `-F` | `me@example.com` | SMTP MAIL FROM；未设则用 `--from` |
| `--rcpt-to` | `-R` | `you@example.fr` | SMTP RCPT TO；未设则用 `--to` |
| `--file-body` | `-B` | `file-body.txt` | 正文文件（可选） |
| `--file-att` | `-A` | `file-att.tgz` | 附件（**106** 可选） |
| `--file-fwd` | `-A` | `file-fwd.eml` | 待转发邮件（**223 必填**） |

106/223 四种 MIME 组合：仅头、+body、+att、+body+att（223 为 +body+fwd 或 +fwd）。

## 服务端（189）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--src-port` | `-P` | `25` | 监听 |
| `--maildir` | `-m` | （必填） | 存信目录 |
| `--timeout` | `-T` | `180000` | 会话 ms |
| `--allowed-clients` | `-c` | `all` | 允许客户 |

需 **port ≤1024** 特权（或 capability）。收到 DATA 以 `msg{N}.txt` 落盘，不转发。

## 非目标

- 不对未授权 SMTP 服务器发信/探测
- 189 不当生产 MTA
