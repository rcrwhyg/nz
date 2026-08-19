# SMB / CIFS 共用字段

套接字可选字段见 [transport/_common.md](../transport/_common.md)。对照 [doc/netwox/smb.md](../../../doc/netwox/smb.md) 与 C `NETWOX_SMBCLI_TOOLARG_*`。

## 客户 — share 模式（199–209）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--dst-ip` | `-i` | （必填） | 服务器 |
| `--dst-port` | `-p` | `139` | TCP 端口 |
| `--share` | `-s` | （必填） | 共享名 |
| `--user` | `-u` | | 用户名 |
| `--password` | `-w` | | 密码 |
| `--authversion` | `-v` | **`2`** | 0=Lanman，1=NTLMv1，2=NTLMv2 |
| `--timeout` | `-T` | `60000` | ms |
| `--verbose` | `-V` | 关 | SMB 跟踪 |
| `--debug` | `-D` | 关 | SMB 报文 |
| `--netbiosname` | `-N` | | 服务器 NetBIOS 名 |

## 客户 — IPC 模式（198）

同 share 模式但**无** `--share`；内部 `isipc=true`，用于列共享。

## 服务端（217）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--src-port` | `-P` | `139` | 监听 |
| `--share` | `-s` | `share` | 共享名 |
| `--rootdir` | `-r` | | 根目录 |
| `--user` | `-u` | | 可选认证 |
| `--password` | `-w` | | |
| `--timeout` | `-T` | **`600000`** | ms |
| `--allowed-clients` | `-c` | `all` | 允许客户 |
| `--allow-put` | `-U` | **开** | 允许写 |
| `--verbose` | `-V` | 关 | |
| `--debug` | `-D` | 关 | |

需 port≤1024 特权（139）。仅 ASCII 文件名。

## 非目标

- 不对未授权 SMB 主机操作
- 不把 authversion 默认改成「best 探测」
