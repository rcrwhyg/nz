# 库能力 spec

- 模块：net / ftp
- 对照：`modules/ftp/`；工具 111–117、168、174
- 状态：draft

## 能力

FTP 控制连接 + 数据通道（主动/被动）：

1. **客户端**：TCP 连 `--dst-ip`:`--dst-port`（缺省 21）；登录 `--user`/`--pass` 缺省 `anonymous`/`user@`；`--passive` 缺省关。
2. **操作**：LIST 目录（111）；RETR/STOR/DELE 单文件（112–114）；递归 get/put/rm 目录（115–117）。
3. **MD5 get**（174）：下载到临时文件后比 `--md5`，不匹配 `BADVALUE`。
4. **服务端**（168）：TCP mulser 21；`--timeout` **180000 ms**；`--allow-put` **C 默认开**；`--allowed-clients` 缺省 all；可选 rootdir + login/password。

默认测试用**假 FTP 通道**，不对公网。

## Rust 形状

`FtpClient` + `list/get/put/delete/recursive_*` + `FtpServer::serve_mulser`。

## 非目标

- 不做 FTPS/SFTP（modern）
- 匿名默认不当安全配置

## 验收

- [ ] `ftp_login_anonymous_default`
- [ ] `ftp_passive_off_by_default`
- [ ] `ftp_get_writes_local_file`
- [ ] `ftp_recursive_get_fake_tree`
- [ ] `ftp_server_allow_put_default_on`
- [ ] `ftp_get_md5_mismatch_badvalue`

## 覆盖率

库代码目标 ≥ 95%。缺口：真 PASV/PORT 在 CI 不测。

## 依赖

[err.md](err.md)、[net-sock.md](net-sock.md)、[hash.md](hash.md)
