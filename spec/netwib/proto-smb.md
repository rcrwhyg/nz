# 库能力 spec

- 模块：net / smb
- 对照：`modules/smb/`；工具 198–209、217
- 状态：draft

## 能力

SMB/CIFS over TCP（缺省端口 **139**）客户端与服务端：

1. **客户端会话**：negotiate → sessionsetup（Lanman/NTLMv1/NTLMv2）→ treeconnect；ASCII 文件名 only（Unicode/重音不支持）。
2. **IPC**（198）：无 `--share`，连 IPC$ 列 share/comment/sharetype。
3. **文件/目录**：mkdir/rmdir/rename、ls、get/put、rm/mv、递归 get/put/rm-dir。
4. **服务端**（217）：TCP mulser 139；`--share` 缺省 `share`；`--rootdir` 可选；可选 user/password；`--allow-put` **C 默认开**；`--allowed-clients` 缺省 all；`--timeout` **600000 ms**；假 FS 统计（见 `smbser.h`）。
5. **调试**：`--verbose` SMB 跟踪、`--debug` 报文；均缺省 **关**。

默认测试用**假 SMB 通道**，不对公网。

## Rust 形状

`SmbClient` + share/ls/get/put/recursive_* + `SmbServer::serve_mulser`。

## 非目标

- 不做 SMB3/加密（modern）
- 不支持 Unicode/非 ASCII-127 文件名
- 217 不做 share 级认证（仅 user 级）

## 验收

- [ ] `smb_client_authversion_default_2`
- [ ] `smb_ipc_lists_shares`
- [ ] `smb_dir_ls_file_and_dir_lines`
- [ ] `smb_file_get_put_roundtrip`
- [ ] `smb_dirrec_get_fake_tree`
- [ ] `smb_server_allow_put_default_on`
- [ ] `smb_server_allowed_clients_reject`

## 覆盖率

库代码目标 ≥ 95%。缺口：真 TCP 139 在 CI 不测。

## 依赖

[err.md](err.md)、[net-sock.md](net-sock.md)
