# spec

每个可实现单元一份说明。没有对应 spec 不写业务代码。

| 文件 | 用途 |
|------|------|
| [TEMPLATE-lib.md](TEMPLATE-lib.md) | netwib 等价能力 |
| [TEMPLATE-tool.md](TEMPLATE-tool.md) | 单个 netwox 工具 |
| [TEMPLATE-gui.md](TEMPLATE-gui.md) | 一条 netwag 工作流 |
| [_index.md](_index.md) | 功能族、相位、闸门 |
| [netwib/](netwib/README.md) | 库骨架 spec（相位 1，draft） |
| [netwox/](netwox/README.md) | CLI 注册表、工具 0、…/dns/http/ftp/dhcp/smtp/snmp/smb（相位 2–10，draft） |
| [netaudit/_deferred.md](netaudit/_deferred.md) | 后置审计/暴力破解 |

复制模板到族目录后填写，例如 `spec/netwib/err.md`、`spec/netwox/info/001.md`。当前闸按 `_index.md` 相位填写，不要一次写 223 份。
