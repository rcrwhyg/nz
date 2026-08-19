# 非网络 / 文件工具

相位：`spec/_index.md` 第 4 项。金标准：`src/netwox-doc_html/tools/N.html`。
无特权、不对网。布尔仍是 `-x|+x|--no-x`。

## 21 — Convert a number

建议名：`conv-num`
Usage：`nz 21 -n data` + 输入进制开关（decimal 默认开 / binary / octal / hexadecimal / char）+ `--title` + 输出开关 `--disp-decimal` 等。
例：`nz 21 --hexa 3D4B --disp-decimal`

## 22 — Convert a string

建议名：`conv-str`
Usage：`nz 22 -d data`
输入：string（默认）/ hexa / mixed / base64。
输出：string、hexa、mixed、base64，以及 md2/md4/md5、ripemd128/160、sha1/224/256/384/512（哈希只算不能反解）。
例：`nz 22 --hexa 4142 --disp-mixed`

## 23 — Display ASCII table

建议名：`ascii-table`
Usage：`nz 23 [-e|+e] [-E|+E]`
默认 0–127；`--extended` 扩字符；`--all` 全部扩字符。GUI 课表用此工具做选中示例。

## 24 — Convert IP addresses ranges

建议名：`ip-calc`
Usage：`nz 24 -i ips [-h|+h] [-r|+r] [-n|+n] [-m|+m]`
网段 ↔ `ip-ip` / `ip/num` / `ip/mask`；`--hnrange` 显示主机名。逐地址详情用工具 3 `-a`；展开列表用工具 213。

## 25 — Test if a directory is secure

建议名：`dir-secure`
Usage：`nz 25 -d dir`
Unix：目录是否对所有人可写（symlink 攻击面）。Windows 不需要。例：`nz 25 -d /tmp`

## 26 — Dump a file

建议名：`file-dump`
Usage：`nz 26 -f file [-e encode]`
默认 dump：左 hex、右不可打印改点。其它 encode 见工具 12（base64_wrap、mixed 等）。

## 27 — Compute MD5 of a file

建议名：`file-md5`
Usage：`nz 27 -f file`
仅 MD5。多算法用工具 219。

## 28 / 29 — mixed 文本 ↔ 二进制

建议名：`bin-to-mixed` / `mixed-to-bin`
Usage：`nz 28|29 -i file -o file`
28 把二进制写成可编辑 mixed；29 再变回二进制。

## 30 / 31 — 换行 unix ↔ dos

建议名：`unix2dos` / `dos2unix`
Usage：`nz 30|31 -i file -o file`
`0x0A` ↔ `0x0D0A`。只处理文本，不处理二进制。

## 127 — XOR 文件

建议名：`file-xor`
Usage：`nz 127 -p password -i file -o file`
口令生成伪随机再与文件 XOR，加解密同一操作。原文写明强度低、可知明文攻击。保留能力，不当成安全加密。

## 128 / 129 — 切片 / 拼回

建议名：`file-split` / `file-join`
`nz 128 -f file [-s uint32]` → `file.0`、`file.1`…（默认块大小约 1.4MB）。
`nz 129 -f file` 使用**不含** `.0/.1` 的原名拼回。

## 186 — Millisecond sleep

建议名：`sleep-ms`
Usage：`nz 186 -m uint32`
实际睡眠略长于请求值。测试用假时钟，不要真睡很久。

## 190 — Make coffee

建议名：`coffee`
Usage：`nz 190 [-n uint32]`
玩笑工具，能力覆盖要保留；输出是趣味文本，不接硬件。

## 191 — Generate a password

建议名：`gen-password`
Usage：`nz 191 [-n uint32] [-i uint32] [-E|+E] [-F|+F] [-S|+S] [-p|+p] [-m uint32] [-a data]`
默认英语词图：每图取前两字母拼口令，并给 variation。`--pronounceable` 按音节。语言：英（默认）/法/西。

## 216 — Beep

建议名：`beep`
Usage：`nz 216 [-a|+a]`
响铃；`--alarm` 多次。无终端铃铛时仍要有可测副作用（或明确降级）。

## 219 — 文件哈希（多算法）

建议名：`file-hash`
Usage：`nz 219 -f file` + 与工具 22 相同的哈希显示开关。
算法：md2/md4/md5、ripemd128/160、sha1/224/256/384/512。实现可用 Rust 生态；md2 等弱算法仍要能出与原文一致的摘要以便对照。

## 220 / 221 — 文件 base64

建议名：`file-b64-enc` / `file-b64-dec`
Usage：`nz 220|221 -i file -o file`

## 非目标

- 不把 127 宣传为可靠加密
- 186 的 CI 不用长睡眠
- 190 不接咖啡机
