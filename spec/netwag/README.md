# GUI 工作流 spec

相位：`spec/_index.md` 第 12 项。对照 [doc/netwag/workflows.md](../../doc/netwag/workflows.md)。实现 crate：**egui**（禁止 webview）。

笔记本结构：Local_info / Remote_info / Clipboard / Tool（内含 Search / Help / Form / Running / History）。

| 文件 | 工作流 |
|------|--------|
| [search.md](search.md) | Search |
| [form.md](form.md) | Form + Help |
| [run.md](run.md) | Running |
| [history.md](history.md) | History |
| [clipboard.md](clipboard.md) | Clipboard |
| [local-info.md](local-info.md) | Local_info |
| [remote-info.md](remote-info.md) | Remote_info |

状态均为 `draft`。

## 第 6 闸最小验收集

1. 搜索（编号 / 树 / 关键字）并选中工具
2. 打开 Help 与 Form
3. Generate → Run，看到输出
4. History 能再跑
5. Local_info 能显示本机配置
6. Remote_info 能查一个主机名

课表 Lesson 2/19/20/22 的呈现细节（字体/配色/动态帮助）不阻塞最小集。
