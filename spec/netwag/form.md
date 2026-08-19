# GUI 工作流 spec

- 工作流名：Form + Help
- 对照课：`netwag-doc_html/html/lessons.html` Lesson 6–9
- 依赖：工具 0 `-h -u N`、`-f -b FILE -u N`
- 状态：draft

## 用户可见行为

### Help

- 必须先选中工具，否则提示未选择
- 内容来自 `nz 0 -h -u N`
- Example 按钮把示例命令填进 Run 行

### Form

- 必须先选中工具
- 每个参数：左侧勾选「是否写入命令行」+ 控件（列表 / 字符串 / 布尔）
- Generate：按勾选生成命令行
- Run / Run_it（生成并跑）/ Reset / Update（`0 -f`）
- 改值时自动勾选该参数

## 契约

- `nz 0 -h -u N`：返回帮助文本 + 参数表
- `nz 0 -f -b FILE -u N`：返回 `--formupdate` 格式（key=value 行）

## native 约束

egui 面板布局；禁止 webview。

## 验收

- [ ] 帮助显示正确
- [ ] 参数表单按 formupdate 格式渲染
- [ ] Generate 产生正确命令行
- [ ] Run_it 等价 Generate + Run

## 非目标

截图级像素、Tk 主题复刻。
