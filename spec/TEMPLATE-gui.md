# GUI 工作流 spec

- 工作流名：（Local_info / Search / Form / Run / History / …）
- 对照课：`netwag-doc_html/html/lessons.html` Lesson N
- 依赖：工具 0 子命令、后端工具号
- 状态：draft | approved | implemented

## 用户可见行为

对齐 lessons 的步骤，不对齐 Tcl 控件坐标。

## 契约

调用的 `nz 0 ...` / 等价 API 与期望输出形状。

## native 约束

禁止 webview。实现 crate：**egui**（已拍板）。

## 验收

- [ ] 契约测试（不依赖像素）
- [ ] 课表步骤可完成

## 非目标

截图级像素、Tk 主题复刻。
