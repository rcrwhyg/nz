# GUI 工作流 spec

- 工作流名：Local_info
- 对照课：`netwag-doc_html/html/lessons.html` Lesson 18
- 依赖：工具 0 `-c`；字段语义对齐工具 1 / 169
- 状态：draft

## 用户可见行为

- 展示本机 Devices / IP / ARP（或邻居缓存） / Routes
- 启动数据来自 `nz 0 -c`（不是直接调工具 1）
- 子标签按 Devices / Ip / Arp_cache / Routes 分

## 契约

`nz 0 -c`：返回本机配置文本（对齐工具 1 / 169 输出格式）。

## native 约束

egui 标签页；禁止 webview。

## 验收

- [ ] 启动显示本机配置
- [ ] 四子面非空

## 非目标

截图级像素、Tk 主题复刻。
