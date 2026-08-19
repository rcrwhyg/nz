# DNS 共用字段

套接字可选字段见 [transport/_common.md](../transport/_common.md)。本族 DNS 语义与 `modules/dns/` 共用。

## 客户查询（102、103）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--dst-ip` | `-i` | （必填） | DNS 服务器 |
| `--dst-port` | `-p` | `53` | 目的端口 |
| `--tcp` | `-C` | 关 | 开则走 TCP；默认 UDP |
| `--timeout` | `-T` | `60000` | 等待应答（ms） |

102 另有 `--name`、`--type`（缺省 `a`）、`--class`（缺省 `in`）、`--recurdesired`（缺省关）。103 固定查 `version.bind.`、type TXT、class CH（硬编码，无 CLI 参数）。

## 固定应答 server（104、105）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--hostname` | `-h` | `www.example.com` | A/MX 应答名；PTR 应答字符串 |
| `--hostnameip` | `-H` | `1.2.3.4` | A 记录 IP |
| `--authns` | `-a` | `ns.example.com` | 权威 NS 名 |
| `--authnsip` | `-A` | `1.2.3.5` | 权威 NS A 附加 |
| `--ttl` | `-T` | `10` | TTL（**秒**） |

105 专用：`--device`、`--filter`、`--spoofip`（缺省 `best`）；嗅探 filter 自动 AND `udp and port 53`。A 查询**不用** `--hostname`；PTR **不用** `--hostnameip`（C 描述与 `dnspktex_answer` 一致）。

104 监听 UDP `--src-port` 缺省 53（`UDP_MULSERPORT("53")`）；需绑定 ≤1024 时可能需特权（C `toolpriv_port1024`）。

## 非目标

- 105 不对未授权网络注入应答
- 不把 DNSSEC 混进本族
