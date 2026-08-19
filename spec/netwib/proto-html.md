# 库能力 spec

- 模块：dat / html + url
- 对照：`modules/html/`、`modules/url/urlcanon.c`；工具 133–135、222
- 状态：draft

## 能力

离线 HTML/URL 处理（无网络 I/O）：

1. **urlcanon**：规范化 URI（折叠 `..` 等）；133 成功则打印一行结果。
2. **htmlfile_urllist**：从 HTML 提取链接；134 按 scheme 过滤；**五类 display 开关全关则 display_all**（全打）。
3. **htmlfile 相对→绝对**：135 用基准 `--url` 把 `-i` 中相对链接写成绝对 URL 到 `-o`。
4. **unlink local**：222 去掉指向 local/unknown scheme 的链接，保留 http/https/ftp 等远端链接（对照 `htmltag` uriformat）。

## Rust 形状

`url_canon` + `html_extract_urls` + `html_make_absolute` + `html_unlink_local`。

## 非目标

- 不做完整 HTML5 解析器
- 不做 DOM/CSS

## 验收

- [ ] `url_canon_collapses_dotdot`
- [ ] `html_urls_all_when_no_filter`
- [ ] `html_abs_rewrites_relative`
- [ ] `html_unlink_local_keeps_http`

## 覆盖率

库代码目标 ≥ 95%。

## 依赖

[err.md](err.md)、[dat.md](dat.md)
