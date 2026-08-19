# 库能力 spec

- 模块：net / http + url
- 对照：`modules/http/`、`modules/url/`；工具 118–125、136、175、182
- 状态：draft

## 能力

HTTP/1.x 明文客户端与服务端（**不含 TLS**；HTTPS 进 `spec/modern/`）及 `http://` / `ftp://` 下载：

1. **httpclictx**：timeout 缺省 60000 ms；proxy 仅 `--proxy-ip` isset 时启用；`--proxy-port` 缺省 3128；proxy 凭据 sensitive；`ftp_passive` 仅 isset 时开。
2. **请求**：解析 URI（scheme/authority/path/query）；直连仅 `http://`（否则 `NOTCONVERTED`）；经 proxy 时发完整 URL。方法 GET/HEAD/DELETE/TRACE/OPTIONS/POST/PUT。`--user-agent` **仅 isset** 才发 User-Agent 头（帮助示例 `Mozilla/5.0`，非自动默认）。
3. **响应**：读 status + headers；可选写 headers/body 到 screen 或 `--file-headers` / `--file-body`（缺省文件名 `filehdr.txt` / `filebody.txt`）。`--display-*` 与 file isset 互斥：isset file 时不走 display 分支（对照 118 core）。
4. **下载**：`url_download_file` / `url_download_io` / `url_download_size`；136/175 共用 ctx；182 只打 size 一行。
5. **httpserctx**：TCP mulser；`rootdir` init 为 `"."`；`timeoutms` 60000；`allowed-clients` NULL=全允许；可选 basic auth login/password。

默认测试用**假 HTTP/FTP 通道**与假 DNS cache，不对公网。

## Rust 形状

`HttpClientCtx` + `request(method, uri, …)` + `UrlDownload` + `HttpServerCtx` + `serve_static_mulser`。禁止 `netwox_*`。

## 非目标

- 不做 HTTPS/TLS（modern 闸）
- 不做 HTTP/2
- 不为每个工具复制 URL 解析

## 验收

- [ ] `http_get_fake_status_and_body`
- [ ] `http_no_user_agent_when_unset`
- [ ] `http_proxy_full_url_when_enabled`
- [ ] `http_non_http_scheme_fails_direct`
- [ ] `url_download_size_prints_uint32`
- [ ] `http_server_rootdir_default_dot`

## 覆盖率

库代码目标 ≥ 95%。缺口：真 TCP 80/443 在 CI 不测。

## 依赖

[err.md](err.md)、[dat.md](dat.md)、[net-sock.md](net-sock.md)、[hash.md](hash.md)
