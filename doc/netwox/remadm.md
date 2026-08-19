# remadm

相位：`spec/_index.md` 第 11 项。金标准：`src/netwox-doc_html/tools/N.html`。
套接字可选字段见 [transport.md](transport.md)。仅实验室/书面授权主机。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名，实现 CLI 闸再钉死。

这是**远程管理能力**（自建服务 + 客户），不是后置审计暴力破解。默认口令很弱，实现时不得当安全默认宣传。

## 93 — TCP remote administration server

建议名：`remadm-server`
Usage：`nz 93 -P port [-w password] … [-c ips] [-r dir] [-T uint32] [-X|+X] [-G|+G] [-U|+U]`

多客户 TCP 服务，供 94/95/96 执行命令、取文件、放文件。无特权（低端口除外）。

| 参数 | 含义 |
|------|------|
| `-P/--src-port` | 监听口（必填语义；例 1234） |
| `-w/--password` | 缺省 `netwox` |
| `-c/--allowed-clients` | 缺省 `all` |
| `-r/--rootdir` | 文件根；例 `/tmp` |
| `-T/--timeout` | **秒**，缺省 60 |
| `-X/--allow-exec` | 允许 exec；**默认开** |
| `-G/--allow-get` | 允许 get；**默认开** |
| `-U/--allow-put` | 允许 put；**默认开** |

`--no-allow-*` 收紧能力。

## 94 / 95 / 96 — 客户

| 号 | 标题 | 建议名 | 必填 |
|----|------|--------|------|
| 94 | TCP remote administration client (exec) | `remadm-exec` | `--dst-ip` `--dst-port` `--command` |
| 95 | TCP remote administration client (get file) | `remadm-get` | `--dst-ip` `--dst-port` `--file` |
| 96 | TCP remote administration client (put file) | `remadm-put` | `--dst-ip` `--dst-port` `--src-file` `--file` |

共用：`--password`（对 93）、`--dst-file`（94/95 收结果）、`--timeout` **秒** 缺省 100、以及「数据是否加密」开关（原文拼写按 HTML，弱算法，与工具 127 同口径）。
94 的 `--command` 例：`/bin/sh -c ls`。

## 126 — HTTP remote administration server

建议名：`remadm-http`
Usage：`nz 126 [-l login] [-L password] … [-P port] [-T uint32] [-c ips]`

Web 服务，让客户通过 HTTP **跑命令**。默认口 80。`--timeout` **毫秒** 缺省 60000。`--allowed-clients` 缺省 `all`。可选 `--login`/`--password` 保护访问。

与工具 125（静态 HTTP 文件服务）不同：126 是远程执行面。

## 非目标

- 不把默认口令当安全配置
- 不对未授权主机开 exec
- 不与后置 brute（101/130–132）混为一谈
