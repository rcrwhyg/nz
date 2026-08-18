# netwag 工作流

对照：`src/netwag-doc_html/html/lessons.html`（Lesson 1–22）。
后端 Tcl 调用点（不移植 Tcl，只记契约）：

| 面 | 源文件 | 工具 0 |
|----|--------|--------|
| Search | `netwag_toolsearch.tcl` | `0 --tools` |
| Help | `netwag_toolhf.tcl` | `0 -h -u N` |
| Form Update | `netwag_toolhf.tcl` | `0 -f -b FILE -u N` |
| Run（内嵌） | `netwag_runtext.tcl` | `0 -r -b FILE`；杀进程 `0 -k -u PID -b 500` |
| Run（新窗口） | `netwag_runnw.tcl` | `0 -R -b FILE` |
| Local_info | `netwag_infol.tcl` | `0 -c` |
| 启动版本 | `netwag_main.tcl` | `0 -v` |

Remote_info 走普通工具 3，不经过工具 0。

nz-gui 用 **egui** 实现同等工作流。不对齐 Tk 颜色/字体/像素；要对齐信息面和可测步骤。

## 笔记本结构（Lesson 1）

顶层：Local_info / Remote_info / Clipboard / Tool。
Tool 内：Search / Help / Form / Running / History。
Local_info 内：Devices / Ip / Arp_cache / Routes。

## 工作流

### Search

- 两种浏览：按编号（sort）、按分类树（tree）
- 关键字过滤（例：输入 `tcp` 只留相关工具）；可切回「show all」
- 单击选中 → Help/Form 针对该工具
- 双击 → 直接打开 Form
- 数据来自 `nz 0 --tools`（树 + stdin/backspace 标记）

### Help

- 必须先选中工具，否则提示未选择
- 内容来自 `nz 0 -h -u N`
- Example 按钮把示例命令填进 Run 行

### Form

- 必须先选中工具
- 每个参数：左侧勾选「是否写入命令行」+ 控件（列表 / 字符串 / 布尔）
- Generate：按勾选生成命令行
- Run / Run_it（生成并跑）/ Reset / Update（按当前命令行工具号刷新表单，对应 `0 -f`）
- 改值时自动勾选该参数

### Run

- 命令行 + Run；可选 NW（新窗口，对应 `0 -R`）
- 内嵌输出：命令用标记行括起；结束有结束标记
- 需要 stdin 的工具才显示发送区（工具号见 [tool0.md](../netwox/tool0.md)）
- 运行中：Copy_command / Interrupt（`0 -k`）/ Pause 显示 / autoscroll
- 结束后：Copy_command / Run_it_again
- 多工具并行：分页切换；进行中与已结束可区分；可关闭某一页

### History

- 每次 Run 记一条
- Add_current / Copy_line / Run_it / Delete_line

### Clipboard

- 多个可编辑剪贴板 + 跨笔记本共享小剪贴板
- 复制粘贴即可；不要求 Tk 快捷键 1:1

### Local_info

- 展示本机 devices / IP / ARP 或 neighbor / routes
- 启动数据来自 `nz 0 -c`（不是直接调工具 1）
- 字段语义对齐工具 1 / 169

### Remote_info

- 输入主机名或 IP，查询
- 后端对齐工具 3（可走 `nz 3` 或等价库 API）

## 课表里可忽略的呈现

Lesson 2/19/20/22：悬停帮助、字体、配色含义、动态帮助开关。可作为 UX 增强，**不作为验收**。
Lesson 21：关闭时保存窗口大小/剪贴板/字体。可做，不阻塞第 6 闸最小集。

## 第 6 闸最小验收集

1. 搜索（编号 / 树 / 关键字）并选中工具
2. 打开 Help 与 Form
3. Generate → Run，看到输出
4. History 能再跑
5. Local_info 能显示本机配置
6. Remote_info 能查一个主机名（实验室/授权目标）
