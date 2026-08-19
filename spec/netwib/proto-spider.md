# 库能力 spec

- 模块：net / webspider
- 对照：`modules/webspider/`；工具 137–139、210–212
- 状态：draft

## 能力

站点镜像 spider 与本地路径映射：

1. **webspidercf**：137 写样例配置；138/211/212 读配置（localrootdir 等）。
2. **run 流水线**（138/139/210）：`run` → `relink` → `index` → `createurllist` → `deltmp`。
3. **139**：全递归；`--rootdir` 缺省 `./spider`（canon 后作 localrootdir）；`--verbose` 关时 loglevel=SCALE，开=NORMAL。
4. **210**：同 139 参数；额外 ACL：`defaulttarget=reject`，仅允许与起始 URL **同目录** 的 URL（stay in directory）。
5. **211/212**：用 spider 配置的 `localrootdir` 做本地文件名 ↔ 原 URL 互查（`urllocalbdd`）。

registry **backspace**：138、139、210（工具 0）。

## Rust 形状

`SpiderConfig` + `Spider::run_pipeline` + `local_to_url` / `url_to_local`。

## 非目标

- 不对公网做 spider 集成测试
- 不做 robots.txt 现代扩展（对齐 5.39.0 行为为准）

## 验收

- [ ] `spider_cf_write_sample`
- [ ] `spider_139_full_recursive_fake`
- [ ] `spider_210_rejects_outside_dir`
- [ ] `spider_url_local_roundtrip`
- [ ] `registry_backspace_138_139_210`

## 覆盖率

库代码目标 ≥ 95%。缺口：大规模真站抓取在 CI 不测。

## 依赖

[err.md](err.md)、[proto-http.md](proto-http.md)、[proto-html.md](proto-html.md)
