# 非网络 / 文件工具 spec

相位：`spec/_index.md` 第 4 项。对照 [doc/netwox/files.md](../../../doc/netwox/files.md)。无特权、不对网。

| 文件 | 号 | 建议名 |
|------|----|--------|
| [021.md](021.md) | 21 | `conv-num` |
| [022.md](022.md) | 22 | `conv-str` |
| [023.md](023.md) | 23 | `ascii-table` |
| [024.md](024.md) | 24 | `ip-calc` |
| [025.md](025.md) | 25 | `dir-secure` |
| [026.md](026.md) | 26 | `file-dump` |
| [027.md](027.md) | 27 | `file-md5` |
| [028.md](028.md) | 28 | `bin-to-mixed` |
| [029.md](029.md) | 29 | `mixed-to-bin` |
| [030.md](030.md) | 30 | `unix2dos` |
| [031.md](031.md) | 31 | `dos2unix` |
| [127.md](127.md) | 127 | `file-xor` |
| [128.md](128.md) | 128 | `file-split` |
| [129.md](129.md) | 129 | `file-join` |
| [186.md](186.md) | 186 | `sleep-ms` |
| [190.md](190.md) | 190 | `coffee` |
| [191.md](191.md) | 191 | `gen-password` |
| [216.md](216.md) | 216 | `beep` |
| [219.md](219.md) | 219 | `file-hash` |
| [220.md](220.md) | 220 | `file-b64-enc` |
| [221.md](221.md) | 221 | `file-b64-dec` |

库：[hash.md](../../netwib/hash.md)、[dat.md](../../netwib/dat.md)、[net-addr.md](../../netwib/net-addr.md)。

对照注意（以 C 为准）：工具 28 写出 `dump` 不是 mixed；工具 128/129 块号从 `.1` 起（HTML 描述曾写 `.0`）。

状态均为 `draft`。
