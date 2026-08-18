# netwib 能力树

对照：`src/netwib-doc_html/netwib/*.html` 与 `src/netwib-src/src/netwib/{err,dat,sys,net,pkt,shw}.h`。
口径：能力清单，**不**复制 C 函数名或 ABI。Rust 侧按模块映射到 `nz-net`。

HTML 入口把库分成 5 节（dat/sys/net/pkt/shw）；`err` 只在头文件里，同样要覆盖。

## err

错误分区（头文件注释）：

| 区间 | 含义 |
|------|------|
| 0 | OK |
| 1000–1999 | 常见数据/路由类失败 |
| 2000–2999 | 参数错误 |
| 3000–3999 | 逻辑错误 |
| 4000–9999 | 函数错误 |
| ≥10000 | 用户自定义码起点 |

`nz-net` 用 Rust `Error`/`Result` 表达同等分区语义，不保留数值 1:1，除非某工具输出依赖具体码（若有，在对应 spec 写明）。

## dat

| 子能力 | 原 HTML 注释 | 映射意向 |
|--------|----------------|----------|
| 基础类型 | types | 整数/布尔/比较等标量 |
| C 字符串与扩展 | c / c2 | 仅内部需要时用；公共 API 走 `&str`/`Vec<u8>` |
| 指针 | ptr | 不暴露；Rust 所有权替代 |
| 缓冲区 | buf | 核心：可变字节缓冲 |
| 缓冲池 | bufpool | 按需；先观察是否值得独立类型 |
| 编码 | bufenc | hex / mixed / base64 等写出 |
| 解码 | bufdec | hex / mixed / base64 等读入 |
| 格式化 | fmt | sprintf 风格；Rust 用 `format!` + 专用编码器 |
| TCP/IP checksum | checksum | **必须有**独立可测实现 |
| 正则 | regexp | 用 Rust 正则 crate，语义对齐工具用法 |
| 双向链表 | ring / ringi | `VecDeque` 或链表，按调用点定 |
| 哈希表 | hash / hashi | `HashMap` |
| TLV | tlv | 编解码能力 |
| 数组 | array | `Vec` |
| uint64 仿真 | uint64 | 不需要；Rust 原生 `u64` |

## sys

| 子能力 | 映射意向 |
|--------|----------|
| 时间 | 单调/墙钟时间，超时 |
| 随机 | 密码学与非密码学分流（spec 时定） |
| 路径 / 路径名 / stat | `std::path` + 元数据 |
| 目录名 / 文件名 / 目录遍历 | 文件系统 |
| 线程与同步（mutex、rwlock、cond、tsd、list） | `std`/`parking_lot`；公共 API 尽量不暴露原模型 |
| IO 抽象 / 新 IO / 常用 IO | 统一读写句柄（文件、套接字、record） |
| wait / waitlist | 多路等待；后续对齐 sock/sniff |
| 文件 / fd / FILE 流 | 文件与描述符；Rust 不暴露 `FILE*` |
| 键盘 / 屏幕 / beep | CLI 交互；GUI 另议 |
| record 存储 | pcap/record 读写（与工具 14–20 强相关） |
| 全局配置 / init / 结束 | 进程级初始化 |
| error 展示 | 与 err 模块衔接 |
| unix / windows 特化 | `cfg` 分支，不进公共语义 |

## net

| 子能力 | 映射意向 |
|--------|----------|
| 网卡 | 列举、选设备 |
| IP / Ethernet / port | 地址类型 |
| 地址集合（ips/eths/ports 及迭代） | 范围、列表、排除 |
| 主机配置（dev / ip / arp / route） | 读本机网络配置（工具 1、6、169） |
| socket 客户/服务 | TCP/UDP 客户与服务 |
| sniff | 捕获 |
| spoof | 伪造发送 |

## pkt

| 子能力 | 映射意向 |
|--------|----------|
| Link 头 | Ethernet 等链路 |
| IPv4/IPv6 头与选项/扩展 | 含 IPv4 options、IPv6 ext |
| ARP | ARP 头 |
| UDP / TCP（含 TCP options） | 传输层 |
| ICMPv4 / ICMPv6 / NDP | 控制报文 |
| layer / packet | 分层构造与解码 |
| conv | 报文转换 |
| ipfrag | IP 分片/重组 |

## shw

原库说明：展示弱于专用嗅探 GUI；解码进结构后再显示，而不是 Ethereal 式即时树。

| 子能力 | 映射意向 |
|--------|----------|
| 展示数组 | 表格化输出 |
| 各层 show（link/ip/arp/udp/tcp/icmp） | Debug/`Display` 或专用 formatter |
| packet show | 整包可读输出 |

工具侧需要稳定文本时，在对应 tool spec 钉输出契约；默认允许现代化排版。
