# DNS

相位：`spec/_index.md` 第 10 项（DNS 族）。金标准：`tools/N.html`。
套接字可选字段见 [transport.md](transport.md)。105 需 sniff+spoof，仅实验室。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名。

## 102 — Query a DNS server

建议名：`dns-query`
Usage：`nz 102 -i ip -n hostname -y data [-c data] [-u|+u] [-C|+C] [-p port] [-T uint32] …`

| 参数 | 含义 |
|------|------|
| `-i/--dst-ip` | 服务器（必填） |
| `-n/--name` | 查询名（必填） |
| `-y/--type` | `a`、`ptr`、`ns` 等（必填；例 `a`） |
| `-c/--class` | 缺省 `in` |
| `-u/--recurdesired` | RD 位 |
| `-C/--tcp` | 改走 TCP；默认 UDP |
| `-p/--dst-port` | 缺省 53 |
| `-T/--timeout` | 毫秒，缺省 60000 |

## 103 — Obtain version of a Bind DNS server

建议名：`dns-bind-version`
向 Bind 查版本字符串。`--dst-ip` 必填；`--tcp`、口 53、超时同 102。

## 104 — DNS server always answering same values

建议名：`dns-fixed`
Usage：`nz 104 -h hostname -H ip -a hostname -A ip [-P port] [-T uint32] …`

对所有查询给同一套答案。必填：`--hostname`、`--hostnameip`、`--authns`、`--authnsip`。监听口缺省 53。`--ttl` **秒** 缺省 10。

## 105 — Sniff and send DNS answers

建议名：`dns-answer`
嗅探 DNS 请求并伪造固定应答（可把流量拐到指定机）。需 sniff+spoof。字段同 104 的主机/NS/TTL，另加 `--device`、`--filter`、`--spoofip`。
A 查询不用 `--hostname`；PTR 不用 `--hostnameip`。

## 非目标

- 105 不对未授权网络注入应答
- 不把现代 DNSSEC 混进本族（进 `spec/modern/`）
