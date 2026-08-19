# SNMP 共用字段

套接字可选字段见 [transport/_common.md](../transport/_common.md)。对照 [doc/netwox/proto-misc.md](../../../doc/netwox/proto-misc.md) SNMP 段与 C `NETWOX_SNMP_ARG`。

## 客户 UDP

| 长名 | 短 | 159/160/164 | 161–163 | 默认 |
|------|-----|-------------|---------|------|
| `--dst-ip` | `-i` | ✓ | ✓ | （必填） |
| `--dst-port` | `-p` | **161** | **162** | 见列 |

## SNMP 通用（NETWOX_SNMP_ARG）

| 长名 | 短 | 默认 | 含义 |
|------|-----|------|------|
| `--version` | `-v` | `1` | 1 / 2 / 3 |
| `--community` | `-c` | `public` | v1/v2 community |
| `--md5auth` | `-m` | **开** | v3 MD5 认证 |
| `--username` | `-u` | | v3 用户名 |
| `--password` | `-w` | | v3 密码 |
| `--authoritativeengineid` | `-A` | | v3 engine id |
| `--authoritativeengineboots` | `-B` | | v3 engine boots |
| `--authoritativeenginetime` | `-M` | | v3 engine time |
| `--contextengineid` | `-C` | | v3 context engine id |
| `--contextname` | `-N` | | v3 context name |
| `--timeout` | `-T` | `10000` | ms |
| `--display` | `-y` | **关** | array 打包显示报文 |

v3：`--md5auth` 开时必须设 `--password`。

## OID 类型字母

`i,s,m,n,o,a,c,g,t,M,u,U` — 用于 trap/set 的 `--oidtype`/`--type` + `--oidvalue`/`--value`。

## 非目标

- 仅授权实验室/书面授权目标
- 不把 `--display` 默认改成开
