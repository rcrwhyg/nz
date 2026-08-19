# GUI 工作流 spec

- 工作流名：Search
- 对照课：`netwag-doc_html/html/lessons.html` Lesson 3–5
- 依赖：工具 0 `--tools`
- 状态：draft

## 用户可见行为

- 两种浏览：按编号（sort）、按分类树（tree）
- 关键字过滤（输入 `tcp` 只留相关工具）；可切回「show all」
- 单击选中 → Help/Form 针对该工具
- 双击 → 直接打开 Form
- 数据来自 `nz 0 --tools`（树 + stdin/backspace 标记）

## 契约

`nz 0 --tools` 输出每行：`NUM TITLE`（树节点用缩进/特殊前缀），需要 stdin 的工具后缀 `[stdin]`。

## native 约束

egui `TextEdit` / `TreeView`；禁止 webview。

## 验收

- [ ] 编号排序可浏览全部工具
- [ ] 树浏览正确分组
- [ ] 关键字过滤缩窄列表
- [ ] 选中工具联动 Help/Form

## 非目标

截图级像素、Tk 主题复刻。
