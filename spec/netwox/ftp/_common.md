# FTP / TFTP 共用字段

套接字可选字段见 [transport/_common.md](../transport/_common.md)。对照 [doc/netwox/ftp.md](../../../doc/netwox/ftp.md)。

## FTP 客户（111–117、174）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--dst-ip` | `-i` | （必填） | FTP 服务器 |
| `--dst-port` | `-p` | `21` | 控制端口 |
| `--user` | `-u` | `anonymous` | 用户名 |
| `--pass` | `-a` | `user@` | 密码 |
| `--passive` | `-V` | 关 | PASV 数据通道 |
| `--timeout` | `-T` | `60000` | ms |

## FTP 服务端（168）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--src-port` | `-P` | `21` | 监听 |
| `--rootdir` | `-r` | | 根目录 |
| `--login` / `--password` | `-l`/`-L` | | 可选认证 |
| `--timeout` | `-T` | **`180000`** | ms |
| `--allowed-clients` | `-c` | `all` | 允许客户 |
| `--allow-put` | `-U` | **开** | 允许上传 |

## TFTP 客户（165–166、176）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--dst-ip` | `-i` | （必填） | 服务器 |
| `--dst-port` | `-p` | `69` | 端口 |
| `--remote-file` | `-F` | | 远端路径 |
| `--local-file` | `-f` | | 本地路径 |
| `--mode` | `-m` | `octet` | `octet` / `netascii` |
| `--timeout` | `-T` | `10000`（**176：`60000`**） | ms |
| `--retry` | `-R` | `3` | 重试次数 |

176 MD5：`--md5`（`-s` 必填）。

## TFTP 服务端（167）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--src-port` | `-P` | `69` | 监听 |
| `--rootdir` | `-r` | | 根目录 |
| `--allowed-clients` | `-c` | `all` | 允许客户 |
| `--allow-get` | `-G` | **开** | 允许读 |
| `--allow-put` | `-U` | **开** | 允许写 |
| `--timeout` | `-T` | `10000` | ms |
| `--retry` | `-R` | `3` | |

## 非目标

- 匿名 FTP 默认不得当安全策略
- 递归删（117）仅授权实验室
