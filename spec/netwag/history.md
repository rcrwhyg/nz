# GUI 工作流 spec

- 工作流名：History
- 对照课：`netwag-doc_html/html/lessons.html` Lesson 15
- 依赖：Run 面
- 状态：draft

## 用户可见行为

- 每次 Run 自动记一条（命令行 + 时间）
- 操作：Add_current / Copy_line / Run_it / Delete_line

## 契约

本地存储；不经过工具 0。

## native 约束

egui 列表；禁止 webview。

## 验收

- [ ] Run 后自动记录
- [ ] Run_it 可重跑历史条目
- [ ] Delete_line 删除

## 非目标

截图级像素、Tk 主题复刻。
