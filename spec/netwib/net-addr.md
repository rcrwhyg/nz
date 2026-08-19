# 库能力 spec

- 模块：net / 地址
- 对照：`netwib-doc_html/netwib/net.html` + `net/{ip,eth,port,ips,eths,ports}.h`
- 状态：draft

## 能力

1. **单值**：IPv4/IPv6、Ethernet 6 字节、TCP/UDP 端口。
2. **集合**：列表 + 范围 + 排除。IP 文本形态（对照 `ips.h` 注释）：
   - 单地址；`a-b` 闭区间
   - `addr/mask` 或 `addr/prefix`；缺末段时补 0（如 `1.2.3/24`）
   - `addr%mask` 或 `addr%prefix`：同 `/` 但**去掉网络/广播端点**
   - 逗号拼接；`all`；`!` 排除
   - 主机名范围用 `=`（因 `-` 可出现在主机名里）
3. 加入已存在或删除不存在的值：**忽略，不当成错误**。
4. 可迭代展开（工具 213、扫描 `--ips`）。

Ethernet 集合另支持 `/prefix` 与 `%prefix`（`%` 去掉两端广播地址）。端口集合：单值、`a-b`、逗号、`all`、`!` 排除。

## Rust 形状

单值：`std::net::IpAddr`；Ethernet 用 `[u8; 6]` newtype（`Display` 如 `e0:69:95:6f:ed:9a`）；`u16` 端口。集合：本仓库类型，解析 `&str` → 规范化区间，迭代器产出单值。禁止 `netwib_*`。本闸不引入 IP 解析 crate。

## 非目标

- 不在本 spec 做 DNS 解析（工具 3 另依赖）
- 不在本 spec 读本机 conf（见后续 `net-conf` spec）
- 不把 hostname 解析绑死在集合类型里；解析失败归 err

## 验收

- [ ] `ip_parse_single_and_cidr`：`1.2.3.4`、`1.2.3.0/24` 展开正确
- [ ] `ip_percent_excludes_ends`：`1.2.3.0%24` 不含 `.0` 与 `.255`
- [ ] `ip_list_all_not_and_comma`：`all,!1.2.3.4` 不含该地址
- [ ] `ip_add_duplicate_ok`：重复加入成功且集合不变
- [ ] `eth_parse_colon`：`aa:bb:cc:dd:ee:ff` 大小写均可读可写
- [ ] `eth_percent_excludes_ends`：`a:b:c:d:e:0%40` 不含 `…:00` 与 `…:ff`
- [ ] `port_range_iter`：`80-82` 产出 80、81、82

## 覆盖率

库代码目标 ≥ 95%。缺口与理由：（实现时填；IPv6 范围若分期须写明）

## 依赖

[err.md](err.md)
