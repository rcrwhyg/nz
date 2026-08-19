# 其它应用协议

相位：`spec/_index.md` 第 10 项剩余族。金标准：`tools/N.html`。
套接字可选字段见 [transport.md](transport.md)。仅授权目标。
99、152、171 需要 stdin（工具 0）。

双模式：`nz N` 与具名子命令等价。下列「具名」是建议名。

## SYSLOG — 97、188

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 97 | SYSLOG client | `syslog-client` | `--dst-ip`；`--priority` 缺省 0；`--message`；口 514 |
| 188 | SYSLOG server | `syslog-server` | `--showscreen` 默认开；`--logfile`；口 514 |

后置工具 98（flood syslog）不在本族。

## TELNET — 99、100、170

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 99 | TELNET client | `telnet-client` | 口 23；`--line-by-line`；需 stdin |
| 100 | TELNET client executing commands | `telnet-exec` | `--login` `--password`；尾部命令；超时 60000 ms |
| 170 | TELNET server | `telnet-server` | 口 23；超时 180000 ms；`--line-by-line` |

后置 101（brute telnet）不在本族。

## SMTP — 106、177、189、223

口缺省 25。发信超时 180000 ms。

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 106 | Send an email | `smtp-send` | `--from` `--to` `--subject`；信封 `--mail-from`/`--rcpt-to` |
| 177 | Check if a SMTP server is up | `smtp-probe` | `--dst-ip` |
| 189 | SMTP server | `smtp-server` | `--maildir` 必填 |
| 223 | Forward an email | `smtp-fwd` | 同 106 + `--file-fwd` 必填 |

## NNTP — 107–109、172–173

口缺省 119。可选 `--login`/`--password`。超时 60000 ms。

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 107 | Post a newsgroup message | `nntp-post` | `--from` `--newsgroup` `--subject` |
| 108 | List newsgroups | `nntp-list` | `--dst-ip` |
| 109 | Download messages | `nntp-get` | `--newsgroup`；首篇/末篇文章编号 |
| 172 | List articles range | `nntp-range` | `--newsgroup` |
| 173 | Download overview | `nntp-over` | 同 109 的范围 |

## IRC — 152–154、178

口缺省 6667。`--dst-ip`；`--nickname` / `--username` / `--password` / `--realname`。152 需 stdin。

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 152 | Interactive IRC client | `irc-client` | nick/user/realname |
| 153 | IRC client listing channels | `irc-list` | |
| 154 | IRC client listening on a channel | `irc-listen` | `--channel`（例 `#chan`） |
| 178 | Check if an IRC server is up | `irc-probe` | |

## SNMP — 159–164

口 Get/Walk/Set 缺省 161；Trap 类口以 HTML 为准。`--version` 1/2/3 缺省 1。v1/v2 `--community` 例 `public`。v3：`--md5auth` 默认开、`--username`/`--password`、engine/context 字段。`--timeout` 10000 ms。`--display` 打包。

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 159 | SNMP Get | `snmp-get` | `--oid` |
| 160 | SNMP Walk | `snmp-walk` | `--oid` |
| 161 | SNMP Trap | `snmp-trap` | 企业 oid、agent IP、generic/specific、uptime、varbind |
| 162 | SNMP Trap2 | `snmp-trap2` | v2 trap |
| 163 | SNMP Inform | `snmp-inform` | |
| 164 | SNMP Set | `snmp-set` | `--oid` `--oidtype` `--value` |

## DHCP — 171、179

需 sniff+spoof。超时 30000 ms。171 需 stdin。

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 171 | DHCP client | `dhcp-client` | DISCOVER；第一个应答即接受。`--device`、`--eth-src` |
| 179 | DHCP client requesting an INFORM | `dhcp-inform` | 另要 `--ip-src` |

## SNTP — 180、181

口缺省 123。显示开关与工具 187 同类；`--disp-rfc822` 默认开。`--version` 缺省 3。客户超时 2000 ms。

| 号 | 标题 | 建议名 |
|----|------|--------|
| 180 | SNTP client obtaining time | `sntp-client` |
| 181 | SNTP server | `sntp-server` |

## IDENT — 193–195

口缺省 113。超时 60000 ms。

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 193 | IDENT client requesting info about an open session | `ident-query` | `--localport` `--remoteport` |
| 194 | IDENT client creating a session and requesting its info | `ident-probe` | `--remoteport` |
| 195 | IDENT server | `ident-server` | `--username`；`--allowed-clients` |

## WHOIS — 196、197

| 号 | 标题 | 建议名 | 要点 |
|----|------|--------|------|
| 196 | WHOIS client | `whois` | `--dst-ip` `--query` |
| 197 | WHOIS client guessing server | `whois-guess` | 只给 `--query` |

超时 60000 ms。

## 非目标

- 后置 brute/flood 不在本文件
- 不对公网做探测集成测试
- 171 接受首个 DHCP 应答：实现闸按原文，不擅自改成「选最优」
