# HTTP / URL / spider

相位：`spec/_index.md` 第 10 项（HTTP 族）。金标准：`tools/N.html`。
原文部分标题把 HTML 拼错；本文一律按 HTML。仅授权目标；CI 不对公网。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名。

## 方法客户 118–124

共用：`--uri`（必填）、`--timeout` 毫秒缺省 60000、`--user-agent` 缺省 `Mozilla/5.0`、可选代理 `--proxy-ip` / `--proxy-port`（3128）/ `--proxy-login` / `--proxy-password`。
屏幕：`--display-status`、`--display-headers`、`--display-body`（有 body 的方法才有）。落盘：`--file-headers`、`--file-body`。

| 号 | 标题 | 建议名 | 额外 |
|----|------|--------|------|
| 118 | HTTP GET | `http-get` | body |
| 119 | HTTP HEAD | `http-head` | 无 body |
| 120 | HTTP POST | `http-post` | `--parameters` |
| 121 | HTTP PUT | `http-put` | `--file-body` 作为上传（必填） |
| 122 | HTTP DELETE | `http-delete` | |
| 123 | HTTP TRACE | `http-trace` | body |
| 124 | HTTP OPTIONS | `http-options` | |

## 125 — HTTP server

建议名：`http-server`
静态根目录服务。`--rootdir`、`--src-port` 缺省 80、`--timeout` 60000 ms、`--allowed-clients` 缺省 `all`、可选 `--login`/`--password`。不是 126（远程执行）。

## 下载 136 / 175

| 号 | 标题 | 建议名 |
|----|------|--------|
| 136 | Web download (`http://` 或 `ftp://`) | `web-get` |
| 175 | 同上并核 MD5 | `web-get-md5` |

136：`--uri` `--dst-file`；`--ftp-passive`；`--display-status`。
175：`--uri` `--md5`；无落盘文件名（只校验）。

## URL / HTML 离线 133–135、211–212、222

无网。

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 133 | Convert an url/uri | `url-convert` | `--uri` |
| 134 | Obtain urls/uris in an HTML file | `html-urls` | `--filename`；http/https/ftp/local/unknown 开关全关则**全打** |
| 135 | Convert urls/uris in an HTML file to absolute urls | `html-abs` | `--src-file` `--dst-file` `--url` |
| 211 | 本地下载文件名 → 原 URL | `spider-file-to-url` | `--conffile` `--file` |
| 212 | URL → 本地下载文件名 | `spider-url-to-file` | `--conffile` `--uri` |
| 222 | 去掉指向本地 URL 的链接 | `html-unlink-local` | `--src-file` `--dst-file` |

## spider 137–139、210

工具 0：138、139、210 要处理 backspace。

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 137 | Create a sample configuration file for tool 138 | `spider-conf-sample` | 写出样例后必须手改 |
| 138 | Web spider（读 137 的配置） | `spider-conf` | `--conffile` |
| 139 | Web spider on command line (fully recursive) | `spider-full` | `--url`；`--rootdir` 缺省 `./spider`；`--verbose` |
| 210 | Web spider on command line (stay in same directory) | `spider-dir` | 同 139，但不离开起始目录 |

## 非目标

- HTTPS 语义对齐完成前不混进旧工具（现代 TLS 进 `spec/modern/`）
- 不对公网做 spider 集成测试
- 126 在 [remadm.md](remadm.md)
