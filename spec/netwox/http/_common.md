# HTTP 共用字段

对照 `modules/http/httpclictx.c`、`toolarg.c`。仅 **http://** 直连（HTTPS 进 `spec/modern/`）。doc 摘录见 [doc/netwox/http.md](../../../doc/netwox/http.md)。

## 方法客户（118–124）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--uri` | `-u` | （必填） | 目标 URI |
| `--proxy-ip` | `-p` | | isset 才启用 proxy |
| `--proxy-port` | `-P` | `3128` | proxy 端口 |
| `--proxy-login` | `-l` | | proxy 登录 |
| `--proxy-password` | `-L` | | proxy 密码 |
| `--user-agent` | `-U` | | isset 才发 User-Agent（帮助示例 `Mozilla/5.0`） |
| `--timeout` | `-T` | `60000` | 毫秒（ctx 缺省；isset 才覆盖） |
| `--display-status` | `-s` | 关 | 打印 statuscode 一行 |
| `--display-headers` | `-h` | 关 | 无 `--file-headers` 时 headers→screen |
| `--file-headers` | `-H` | `filehdr.txt` | 写 headers 到文件 |
| `--display-body` | `-b` | 关 | 无 `--file-body` 时 body→screen |
| `--file-body` | `-B` | `filebody.txt` | 写 body 到文件 |

120 另有 `--parameters`（`-a`）。121 的 `--file-body`（`-B`）为**必填**上传文件（`REQ_BUF_FILE_RD`），无 body display。

## 下载（136、175、182）

共用 proxy/UA/timeout/`--ftp-passive`（`-f`/`-F`，isset 才 passive）。136：`--dst-file`（`-f` 必填）、可选 `--display-status`。175：`--md5`（`-m` 必填），临时文件下载后比 MD5，失败 `BADVALUE`。182：只输出 size 一行。

## 静态 server（125）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--src-port` | `-P` | `80` | 监听口 |
| `--rootdir` | `-r` | `.` | 静态根（ctx init 已是 `.`） |
| `--login` / `--password` | `-l`/`-L` | | 可选 basic auth |
| `--timeout` | `-T` | `60000` | 毫秒 |
| `--allowed-clients` | `-c` | `all` | 允许客户 IP |

非 126（远程执行在 remadm 族，见 `doc/netwox/remadm.md`）。

## 非目标

- HTTPS/TLS 不进本族
- 不对公网 spider/扫描
