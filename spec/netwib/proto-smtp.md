# 库能力 spec

- 模块：net / smtp + mailex
- 对照：`modules/smtp/`、`modules/mailex.c`；工具 106、177、189、223
- 状态：draft

## 能力

SMTP over TCP（缺省端口 25）与 MIME 邮件构造：

1. **客户端会话**（`smtp_send`）：读 220 → 问候（domain 取自 MAIL FROM 的 `@` 后缀）→ `MAIL FROM` → `RCPT TO`（接受 250 或 251）→ `DATA` → 354 后写正文 → 以 `.` 结束 → 250 → `QUIT` → 221。
2. **探测**（177）：连上后读 220 → 仅 `QUIT` → 221；**不**发问候或 MAIL。
3. **MIME 构造**（mailex）：From/To/Subject 头；可选 `--from-name`；正文 8bit；附件 base64；转发模式嵌入 `--file-fwd` 原文（106 可选 body+att；223 必填 fwd）。
4. **信封 vs 头**：`--mail-from`/`--rcpt-to` 未设时分别用 `--from`/`--to`。
5. **服务端**（189）：TCP mulser 25；会话写 `maildir/msg{N}.txt`（递增 N）；接受问候、MAIL、RCPT、DATA、RSET、NOOP、QUIT 等；**不转发**到外 MTA；`--allowed-clients` 缺省 all；会话超时 **180000 ms**。

默认测试用**假 SMTP 通道**，不对公网。

## Rust 形状

`SmtpClient::probe` + `SmtpClient::send` + `MimeMessage::build_*` + `SmtpServer::serve_mulser`。

## 非目标

- 不做 SMTPS/STARTTLS（modern）
- 189 不实现真实 MTA 投递链
- 不对未授权邮件服务器发信

## 验收

- [ ] `smtp_send_greeting_mail_rcpt_data_quit`
- [ ] `smtp_probe_quit_only_177`
- [ ] `smtp_envelope_fallback_from_to_headers`
- [ ] `mailex_body_attachment_multipart`
- [ ] `mailex_forward_embeds_file_fwd`
- [ ] `smtp_server_writes_msg_txt_in_maildir`
- [ ] `smtp_server_allowed_clients_reject`

## 覆盖率

库代码目标 ≥ 95%。缺口：真 TCP 25 监听在 CI 不测。

## 依赖

[err.md](err.md)、[net-sock.md](net-sock.md)
