# doc

从对照发行版摘录、分类后的说明。不是源码镜像。

对照根目录（本地、gitignore）：`netw-ib-ox-ag-5.39.0/`。

## 本目录

| 文件 | 内容 |
|------|------|
| [architecture.md](architecture.md) | 三层对照与源码/HTML 指针 |
| [inventory.md](inventory.md) | netwib 模块、223 工具、netwag 功能面 |
| [gaps.md](gaps.md) | 动手写代码前仍待拍板的缺口 |
| [netwib/](netwib/README.md) | 库能力摘录（进行中） |
| [netwox/](netwox/README.md) | 工具/协议摘录（进行中） |
| [netwag/](netwag/README.md) | GUI 工作流摘录（进行中） |

## 抽取规则（第 1 闸才执行）

- 金标准优先：`src/netwox-doc_html/tools/N.html`（Usage / Parameters / Examples）
- 库 API 优先：`src/netwib-doc_html/` + `src/netwib-src/src/netwib.h` 模块头
- GUI 优先：`src/netwag-doc_html/html/lessons.html` + 工具 0 的 Tcl 调用点
- 禁止把 `.c` 整文件贴进本目录；只摘行为、参数、能力边界
- 不修改对照树
