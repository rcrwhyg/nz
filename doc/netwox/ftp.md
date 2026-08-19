# FTP / TFTP

相位：`spec/_index.md` 第 10 项。金标准：`tools/N.html`。
套接字可选字段见 [transport.md](transport.md)。仅授权目标。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名。

## FTP 客户 111–117、174

默认 `--user`=`anonymous`、`--pass`=`user@`。`--passive` 默认关。`--timeout` 毫秒缺省 60000。`--dst-ip` 必填；控制口缺省 21。

| 号 | 标题 | 建议名 | 路径 |
|----|------|--------|------|
| 111 | FTP listing a directory | `ftp-ls` | `--dir` 远端 |
| 112 | FTP client : get a file | `ftp-get` | `--file` 远端、本地文件 |
| 113 | FTP client : put a file | `ftp-put` | 本地 → 远端 |
| 114 | FTP client : del a file | `ftp-rm` | 远端文件 |
| 115 | FTP client : get a directory recursively | `ftp-get-dir` | 远端/本地目录 |
| 116 | FTP client : put a directory recursively | `ftp-put-dir` | 本地 → 远端目录 |
| 117 | FTP client : del a directory recursively | `ftp-rm-dir` | 远端目录 |
| 174 | FTP client : get a file and check its MD5 | `ftp-get-md5` | `--file` 远端 + `--md5` |

## 168 — FTP server

建议名：`ftp-server`
`--rootdir`、`--login`/`--password`、口缺省 21、`--timeout`、`--allowed-clients` 缺省 `all`、`--allow-put`（默认开）。

## TFTP 165–167、176

UDP。口缺省 69。`--timeout` 毫秒缺省 10000；`--retry` 缺省 3。`--mode`：`octet`（缺省）或 `netascii`。客户用 `--remote-file` / `--local-file`。

| 号 | 标题 | 建议名 |
|----|------|--------|
| 165 | TFTP client : get a file | `tftp-get` |
| 166 | TFTP client : put a file | `tftp-put` |
| 176 | TFTP client : get a file and check its MD5 | `tftp-get-md5` |
| 167 | TFTP server | `tftp-server` |

167：`--rootdir`、`--allowed-clients`、`--allow-get`/`--allow-put` 默认开。

## 非目标

- 匿名默认不得当安全配置
- 递归删只在授权实验室
