# 库能力 spec

- 模块：net / snmp + asn1
- 对照：`modules/snmp/`、`modules/asn1/`；工具 159–164
- 状态：draft

## 能力

SNMP over UDP 编解码与会话（v1 / v2c / v3 USM）：

1. **版本**：`--version` 1/2/3，缺省 **1**；v1/v2 `--community` 缺省 **`public`**；v3 `--md5auth` **默认开**，需 `--username`/`--password`，可选 engine/context 字段（`authoritativeengineid/boots/time`、`contextengineid/name`）。v3 未给 engine 时先发 **Reportable GET** 探 engine。
2. **超时/显示**：`--timeout` **10000 ms**；`--display` 缺省 **关**（array 打包打屏）。
3. **PDU**：GET / GETNEXT / SET / TRAP(v1) / TRAP2 / INFORM；BER 编解码；varbind 列表。
4. **OID 值类型**（`--oidtype`/`--type`）：`i,s,m,n,o,a,c,g,t,M,u,U`（对照 `asn1data_init_arg`）。
5. **Walk**（160）：循环 GETNEXT；打印 `oid: value`；v1 以 errorstatus≠0 结束，v2 以返回 oid 与当前 oid **相等**结束（`DATAEND`）。
6. **Trap2/Inform**（162/163）：固定 varbind `sysUpTime.0` + `snmpTrapOID.0`（enterprise）+ 用户 oid/value。

默认测试用**假 UDP SNMP 通道**，不对公网。

## Rust 形状

`SnmpSession` + `get/get_next/set/trap/trap2/inform` + `SnmpMessage` encode/decode + v3 USM MD5。

## 非目标

- 不做 SNMP over TCP/TLS（modern）
- v3 仅对齐 C 已有 USM/MD5 子集
- 不对未授权设备查询/改 MIB

## 验收

- [ ] `snmp_v1_community_public_default`
- [ ] `snmp_v3_md5auth_default_on`
- [ ] `snmp_get_prints_synth_value`
- [ ] `snmp_walk_getnext_until_dataend`
- [ ] `snmp_trap_v1_no_reply`
- [ ] `snmp_trap2_three_varbinds`
- [ ] `snmp_inform_waits_response`
- [ ] `snmp_set_prints_set_value`
- [ ] `snmp_display_off_by_default`

## 覆盖率

库代码目标 ≥ 95%。缺口：真 UDP 161/162 在 CI 不测。

## 依赖

[err.md](err.md)、[net-sock.md](net-sock.md)
