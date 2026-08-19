# GUI 工作流 spec

- 工作流名：Remote_info
- 对照课：`netwag-doc_html/html/lessons.html` Lesson 18
- 依赖：工具 3（或等价库 API）
- 状态：draft

## 用户可见行为

- 输入主机名或 IP，查询远端信息
- 后端走 `nz 3`（不经过工具 0）

## 契约

`nz 3 -i HOST` 或等价库调用。

## native 约束

egui 输入框 + 文本输出；禁止 webview。

## 验收

- [ ] 输入主机名可查询
- [ ] 仅授权/实验室目标

## 非目标

截图级像素、Tk 主题复刻。
