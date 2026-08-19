# SMB / CIFS

相位：`spec/_index.md` 第 10 项。金标准：`tools/N.html`。
套接字可选字段见 [transport.md](transport.md)。仅授权目标。默认口 **139**。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名。

## 客户共用

`--dst-ip` 必填。除 198 外要 `--share`。`--user`/`--password`、`--netbiosname`。
`--authversion`：0=Lanman，1=NTLMv1，2=NTLMv2；未指定取 best（表单例常为 2）。
`--timeout` 毫秒缺省 60000。`--verbose` 打 SMB 跟踪；`--debug` 打 SMB 包。

| 号 | 标题 | 建议名 |
|----|------|--------|
| 198 | list shares | `smb-shares` |
| 199 | create a directory | `smb-mkdir` |
| 200 | delete a directory | `smb-rmdir` |
| 201 | rename a directory | `smb-mv-dir` |
| 202 | list contents of a directory | `smb-ls` |
| 203 | delete a file | `smb-rm` |
| 204 | rename a file | `smb-mv` |
| 205 | get a file | `smb-get` |
| 206 | put a file | `smb-put` |
| 207 | recursively get a directory | `smb-get-dir` |
| 208 | recursively put a directory | `smb-put-dir` |
| 209 | recursively delete a directory | `smb-rm-dir` |

路径长名以各工具 HTML 为准（远端/本地 `file`/`dir` 对）。

## 217 — SMB/CIFS server

建议名：`smb-server`
`--share`（例 `share`）、`--rootdir`、`--user`/`--password`、口 139、`--timeout` 毫秒缺省 600000、`--allowed-clients` 缺省 `all`、`--allow-put` **默认开**。同样有 verbose/debug。

## 非目标

- 不实现现代 SMB3/加密作为「改旧语义」；扩展进 `spec/modern/`
- 不对未授权主机做文件操作
