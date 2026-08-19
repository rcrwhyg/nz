# GUI 工作流 spec

- 工作流名：Running
- 对照课：`netwag-doc_html/html/lessons.html` Lesson 10–14
- 依赖：工具 0 `-r -b FILE`、`-R -b FILE`、`-k -u PID -b MS`
- 状态：draft

## 用户可见行为

- 命令行 + Run 按钮；可选 NW（新窗口，`0 -R`）
- 内嵌输出：命令用标记行括起；结束有结束标记
- 需要 stdin 的工具才显示发送区
- 运行中：Copy_command / Interrupt（`0 -k`）/ Pause 显示 / autoscroll
- 结束后：Copy_command / Run_it_again
- 多工具并行：分页切换；进行中与已结束可区分；可关闭某一页

## 契约

- `nz 0 -r -b FILE`：内嵌执行，stdout/stderr 逐行流回
- `nz 0 -R -b FILE`：新上下文执行
- `nz 0 -k -u PID -b 500`：中断运行进程

## native 约束

egui 滚动文本输出；禁止 webview。

## 验收

- [ ] 命令可运行且输出实时显示
- [ ] Interrupt 可中断运行中工具
- [ ] stdin 工具有发送区
- [ ] 多工具分页并行

## 非目标

截图级像素、Tk 主题复刻。
