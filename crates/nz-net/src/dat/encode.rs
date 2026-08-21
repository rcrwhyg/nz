//! 字节缓冲编码写出。
//!
//! 对照 netwib `dat/bufenc.h` 与工具 12 表单别名。`hexa`=`hexa1`，`mixed`=`mixed1`，`array`=`array8`。

use std::fmt::Write;

/// 编码输出格式（相位 1 子集）。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodeFormat {
    /// 按 Latin-1 原样写入可表示字符（不可表示字节按 `\xHH` 逃逸）。
    Data,
    /// 字节间空格的十六进制（`hexa1`）。
    Hexa,
    /// 可打印 ASCII 用引号，其余用十六进制（`mixed1`）。
    Mixed,
    /// 标准 Base64。
    Base64,
    /// UTF-8 文本（不可解码时替换 U+FFFD）。
    Text,
    /// 空串。
    Nothing,
    /// 合成可读串：可打印字符直出，其余空格分隔 hex。
    Synth,
    /// 十六进制转储（偏移 + 十六进制 + ASCII 栏）。
    Dump,
    /// C 风格 `{0xNN, …}`（`array8`）。
    Array,
    /// [`EncodeFormat::Hexa`] 并按列宽折行。
    HexaWrap,
    /// [`EncodeFormat::Mixed`] 并按列宽折行。
    MixedWrap,
    /// 仅 hex 的 mixed 风格折行（`mixedh_wrap`）。
    MixedHexWrap,
    /// 十六进制小写。
    Lowercase,
    /// 十六进制大写。
    Uppercase,
}

/// 默认折行宽度（对照 netwib wrap 常用 76 列）。
const WRAP_WIDTH: usize = 76;

/// 把字节切片编码为文本。
#[must_use]
pub fn encode_bytes(data: &[u8], format: EncodeFormat) -> String {
    match format {
        EncodeFormat::Data => encode_data(data),
        EncodeFormat::Hexa | EncodeFormat::HexaWrap => wrap_lines(
            &encode_hexa(data, HexCase::Lower),
            matches!(format, EncodeFormat::HexaWrap),
        ),
        EncodeFormat::Mixed | EncodeFormat::MixedWrap => wrap_lines(
            &encode_mixed(data),
            matches!(format, EncodeFormat::MixedWrap),
        ),
        EncodeFormat::MixedHexWrap => wrap_lines(&encode_mixed_hex_only(data), true),
        EncodeFormat::Base64 => encode_base64(data),
        EncodeFormat::Text => String::from_utf8_lossy(data).into_owned(),
        EncodeFormat::Nothing => String::new(),
        EncodeFormat::Synth => encode_synth(data),
        EncodeFormat::Dump => encode_dump(data),
        EncodeFormat::Array => encode_array(data),
        EncodeFormat::Lowercase => encode_hexa(data, HexCase::Lower),
        EncodeFormat::Uppercase => encode_hexa(data, HexCase::Upper),
    }
}

fn encode_data(data: &[u8]) -> String {
    let mut output = String::new();
    for &byte in data {
        if byte.is_ascii_graphic() || byte == b' ' {
            output.push(char::from(byte));
        } else {
            let _ = write!(output, "\\x{byte:02x}");
        }
    }
    output
}

#[derive(Clone, Copy)]
enum HexCase {
    Lower,
    Upper,
}

fn encode_hexa(data: &[u8], case: HexCase) -> String {
    data.iter()
        .map(|byte| match case {
            HexCase::Lower => format!("{byte:02x}"),
            HexCase::Upper => format!("{byte:02X}"),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn encode_mixed(data: &[u8]) -> String {
    let mut parts = Vec::new();
    let mut index = 0usize;
    while index < data.len() {
        if data[index].is_ascii_graphic() || data[index] == b' ' {
            let start = index;
            index += 1;
            while index < data.len() && (data[index].is_ascii_graphic() || data[index] == b' ') {
                index += 1;
            }
            parts.push(encode_quoted(&data[start..index]));
        } else {
            let start = index;
            index += 1;
            while index < data.len() && !(data[index].is_ascii_graphic() || data[index] == b' ') {
                index += 1;
            }
            parts.push(encode_hexa(&data[start..index], HexCase::Lower));
        }
    }
    parts.join(" ")
}

fn encode_mixed_hex_only(data: &[u8]) -> String {
    encode_hexa(data, HexCase::Lower)
}

fn encode_quoted(bytes: &[u8]) -> String {
    let mut output = String::from("'");
    for &byte in bytes {
        if byte == b'\'' {
            output.push_str("''");
        } else {
            output.push(char::from(byte));
        }
    }
    output.push('\'');
    output
}

fn encode_synth(data: &[u8]) -> String {
    if data.is_empty() {
        return String::from("<empty>");
    }
    let mut parts = Vec::new();
    for &byte in data {
        if byte.is_ascii_graphic() {
            parts.push(format!("'{byte}'"));
        } else {
            parts.push(format!("{byte:02x}"));
        }
    }
    parts.join(" ")
}

fn encode_dump(data: &[u8]) -> String {
    let mut lines = Vec::new();
    for (offset, chunk) in data.chunks(16).enumerate() {
        let hex: String = chunk
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<Vec<_>>()
            .join(" ");
        let ascii: String = chunk
            .iter()
            .map(|byte| {
                if byte.is_ascii_graphic() {
                    char::from(*byte)
                } else {
                    '.'
                }
            })
            .collect();
        lines.push(format!("{offset:08x}  {hex:<47}  {ascii}"));
    }
    lines.join("\n")
}

fn encode_array(data: &[u8]) -> String {
    if data.is_empty() {
        return String::from("{}");
    }
    let body = data
        .iter()
        .map(|byte| format!("0x{byte:02x}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{{{body}}}")
}

fn encode_base64(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::new();
    let mut index = 0usize;
    while index < data.len() {
        let remaining = data.len() - index;
        let b0 = data[index];
        let b1 = if remaining > 1 { data[index + 1] } else { 0 };
        let b2 = if remaining > 2 { data[index + 2] } else { 0 };

        output.push(char::from(TABLE[(b0 >> 2) as usize]));
        output.push(char::from(TABLE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize]));
        if remaining > 1 {
            output.push(char::from(TABLE[(((b1 & 0x0F) << 2) | (b2 >> 6)) as usize]));
        } else {
            output.push('=');
        }
        if remaining > 2 {
            output.push(char::from(TABLE[(b2 & 0x3F) as usize]));
        } else {
            output.push('=');
        }
        index += 3;
    }
    output
}

fn wrap_lines(text: &str, enabled: bool) -> String {
    if !enabled || text.len() <= WRAP_WIDTH {
        return text.to_owned();
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    for token in text.split(' ') {
        let extra = if current.is_empty() {
            token.len()
        } else {
            token.len() + 1
        };
        if !current.is_empty() && current.len() + extra > WRAP_WIDTH {
            lines.push(std::mem::take(&mut current));
            current.push_str(token);
        } else if current.is_empty() {
            current.push_str(token);
        } else {
            current.push(' ');
            current.push_str(token);
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{EncodeFormat, encode_bytes};

    /// spec `encode_hexa1_spaces`
    #[test]
    fn encode_hexa1_spaces() {
        assert_eq!(encode_bytes(&[0x01, 0x02], EncodeFormat::Hexa), "01 02");
    }

    #[test]
    fn encode_nothing_is_empty() {
        assert_eq!(encode_bytes(&[0x01], EncodeFormat::Nothing), "");
    }

    #[test]
    fn encode_mixed_matches_decode_examples() {
        let encoded = encode_bytes(b"a'b", EncodeFormat::Mixed);
        assert_eq!(encoded, "'a''b'");
    }

    #[test]
    fn encode_base64_abc() {
        assert_eq!(encode_bytes(b"ABC", EncodeFormat::Base64), "QUJD");
    }

    #[test]
    fn encode_array_format() {
        assert_eq!(
            encode_bytes(&[0x01, 0x02], EncodeFormat::Array),
            "{0x01, 0x02}"
        );
    }

    #[test]
    fn encode_lowercase_and_uppercase_hex() {
        assert_eq!(encode_bytes(&[0xAB], EncodeFormat::Lowercase), "ab");
        assert_eq!(encode_bytes(&[0xAB], EncodeFormat::Uppercase), "AB");
    }

    #[test]
    fn encode_wrap_inserts_newlines_for_long_hex() {
        let data = vec![0x11; 40];
        let wrapped = encode_bytes(&data, EncodeFormat::HexaWrap);
        assert!(wrapped.contains('\n'));
    }

    #[test]
    fn encode_data_text_synth_and_dump_are_non_empty() {
        let bytes = b"Hi!\x00";
        assert!(encode_bytes(bytes, EncodeFormat::Data).contains("\\x00"));
        assert_eq!(encode_bytes(b"Hi", EncodeFormat::Text), "Hi");
        assert!(encode_bytes(bytes, EncodeFormat::Synth).contains("00"));
        assert!(encode_bytes(bytes, EncodeFormat::Dump).contains("00000000"));
        assert_eq!(encode_bytes(&[], EncodeFormat::Synth), "<empty>");
    }

    #[test]
    fn encode_mixed_wrap_can_wrap() {
        let data = vec![b'a'; 120];
        let wrapped = encode_bytes(&data, EncodeFormat::MixedWrap);
        assert!(wrapped.starts_with('\''));
    }

    #[test]
    fn encode_mixed_hex_wrap_and_empty_dump() {
        let data = vec![0x01, 0x02];
        assert!(encode_bytes(&data, EncodeFormat::MixedHexWrap).contains("01"));
        assert_eq!(encode_bytes(&[], EncodeFormat::Dump), "");
        assert_eq!(encode_bytes(b"abc", EncodeFormat::Data), "abc");
    }

    #[test]
    fn encode_base64_padding_lengths() {
        assert_eq!(encode_bytes(b"A", EncodeFormat::Base64), "QQ==");
        assert_eq!(encode_bytes(b"AB", EncodeFormat::Base64), "QUI=");
    }

    #[test]
    fn encode_hexa_wrap_short_text_unchanged() {
        assert_eq!(encode_bytes(&[0x01, 0x02], EncodeFormat::HexaWrap), "01 02");
    }
}
