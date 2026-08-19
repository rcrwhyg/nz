# spoof 共用字段

分层工具 **短选项字母会平移**，实现与测试以**长名**为准。组包规则与 `spoofip` 取值全族共用。各工具 spec 只钉 Usage 与本工具特有语义。

## 发送路径

- **链路**（32–37、140–143）：`--device`；DLT 必须 `ether`，否则打印无 Ether DLT 后失败（对照 `NETWOX_ERR_SPOOF_INVALIDDLT`）。发前用 array+dump **打到屏幕**。
- **IP**（38–41、144–147）：`--spoofip`（Advanced，缺省 `best`）。无 `--device`。发前用 IP array+dump 打到屏幕。
- **样本**（42–48、192）：`--spoofip` 同上；仅 `--display` 开才打印。分片走 `spooffrag`（`--fragsize` 0=不分片）。

`spoofip`：`raw`、`linkf`、`linkb`、`linkfb`、`rawlinkf`/`b`/`fb`、`linkfraw`/`braw`/`fbraw`；别名 `link`=`linkfb`，`rawlink`=`rawlinkfb`，`linkraw`=`linkfbraw`，`best`=`linkraw`。源 IP 伪造时 `linkf` 可能填不出 MAC。

## 组包

未给的头字段用库 `initdefault`。`ip4-id`、`tcp-seqnum` 未指定则随机（测试注入 RNG）。

高级字段（CLI 里 `OPTA`：ihl / totlen / 各层 checksum / TCP doff / 分层工具的 `--eth-type` / IPv6 `--ip6-payloadlength` 等）：**只有用户显式给出**才字面写入。做法对照 C：先正常组包，再解码、改字段、按头逐层重装（不再自动算），并打印：

```
Those options generate an invalid packet. Do not trust sniffer display.
Raw packet display (correct):
…
Nice packet display (don't trust it):
```

载荷与 options 为 mixed。IPv6 `--ip6-src` / `--ip6-dst` / `--ip6-exts` **仅 isset 才覆盖** initdefault。

## Ethernet

`--device`、`--eth-src`、`--eth-dst`、`--eth-type`（32 还有 `--eth-data`）。

- 32：`--eth-type` 非 Advanced，未给则为 0（覆盖 initdefault）。
- 33：`--eth-type` 默认 `2054`（ARP）；RARP=`32821`。
- 34–37：`--eth-type` 为 Advanced，帮助示例 `2048`；未给则组包后的以太网 type 由库填（IPv4）。
- 140–143：`--eth-type` 为 Advanced，帮助仍可能写 `2048`；**未给时用 initdefault（IPv6 ethertype）**，不以帮助示例覆盖。

## ARP（仅 33）

`--arp-op`：1=ARPREQ，2=ARPREP，3=RARPREQ，4=RARPREP（默认 1）。`--arp-ethsrc` / `--arp-ipsrc`（默认 `0.0.0.0`）/ `--arp-ethdst`（默认 `0:0:0:0:0:0`）/ `--arp-ipdst`（默认 `0.0.0.0`）。

## IPv4

`--ip4-tos`、`--ip4-id`、`--ip4-reserved`、`--ip4-dontfrag`、`--ip4-morefrag`、`--ip4-offsetfrag`、`--ip4-ttl`、`--ip4-protocol`、`--ip4-src`、`--ip4-dst`、`--ip4-opt`、无上层时 `--ip4-data`。高级：`--ip4-ihl`、`--ip4-totlen`、`--ip4-checksum`。

## IPv6

`--ip6-trafficclass`、`--ip6-flowlabel`、`--ip6-protocol`（next header）、`--ip6-ttl`（hop limit）、`--ip6-src`、`--ip6-dst`、`--ip6-exts`、无上层时 `--ip6-data`。高级：`--ip6-payloadlength`。

## UDP / TCP / ICMP

- UDP：`--udp-src` `--udp-dst` `--udp-data`；高级 `--udp-len` `--udp-checksum`
- TCP：`--tcp-src` `--tcp-dst` `--tcp-seqnum` `--tcp-acknum`、`--tcp-reserved1`…`4`、`--tcp-cwr` `--tcp-ece` `--tcp-urg` `--tcp-ack` `--tcp-psh` `--tcp-rst` `--tcp-syn` `--tcp-fin`、`--tcp-window` `--tcp-urgptr` `--tcp-opt` `--tcp-data`；高级 `--tcp-doff` `--tcp-checksum`
- ICMPv4（37/41）：先读 `--icmp-type`（未给则为 0），再 `initdefault(type)` 填体；`--icmp-code` 仅 isset。高级 `--icmp-checksum`。
- ICMPv6（143/147）：未给 `--icmp-type` 时为 Echo Request（128），isset 才覆盖；再 `initdefault(type)`；`--icmp-code` 仅 isset。高级 `--icmp-checksum`。

## 样本（42–48、192）

`--sample` 1=`udp_syslog`，2=`tcp_syn`，3=`tcpsynack`，4=`tcpack`，5=`ping`，缺省 1。越界（0 或 >5）失败（对照 `NETWIB_ERR_PATOOHIGH`）。

共用：`--ip4-src` `--ip4-dst` `--tcp-src` `--tcp-dst` `--fragsize`（默认 0=不分片）`--display`（默认关）`--spoofip`。C 把 `--tcp-src`/`--tcp-dst` 都标成 `PORT_DST` 类型，长名仍是这两名；**仅 isset 才覆盖端口**，否则 dst=80（WWW）、src 为 1024–65535 随机。`--ip4-src`/`--ip4-dst` 会覆盖样本 initdefault（`1.2.3.4`/`5.6.7.8`）；未给时走参数类型默认（HTML 示例 src 为本机样例、dst `5.6.7.8`）。

样本 1 的 UDP **目的端口固定 syslog**，不用 `--tcp-dst`。分片向上取整到 8（[pkt.md](../../netwib/pkt.md)）。字节级内容实现闸对照 `sample.c`，不贴进 spec。
