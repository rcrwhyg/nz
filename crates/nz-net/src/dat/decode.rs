//! 字节缓冲解码读入。
//!
//! 对照 netwib `dat/bufdec.h`。非法 hex / mixed / base64 归 [`Error::invalid_parameter`]。

use crate::error::{Error, Result};

/// 解码输入格式（相位 1 子集）。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeFormat {
    /// 输入文本按 UTF-8 字节原样读入。
    Data,
    /// 连续十六进制对，可含空白。
    Hexa,
    /// 引号文本与十六进制混排（`mixed1`）。
    Mixed,
    /// 标准 Base64（含 padding）。
    Base64,
}

/// 按格式把文本解码为字节。
///
/// # Errors
///
/// 非法 hex、mixed 或 base64 语法时返回 [`Error::InvalidParameter`]。
pub fn decode_input(input: &str, format: DecodeFormat) -> Result<Vec<u8>> {
    match format {
        DecodeFormat::Data => Ok(input.as_bytes().to_vec()),
        DecodeFormat::Hexa => decode_hex_pairs(input),
        DecodeFormat::Mixed => decode_mixed(input),
        DecodeFormat::Base64 => decode_base64(input),
    }
}

fn decode_hex_pairs(input: &str) -> Result<Vec<u8>> {
    let hex_digits: String = input.chars().filter(|ch| !ch.is_whitespace()).collect();
    if hex_digits.is_empty() {
        return Ok(Vec::new());
    }
    if !hex_digits.len().is_multiple_of(2) {
        return Err(Error::invalid_parameter("odd hex digit count"));
    }
    if !hex_digits.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(Error::invalid_parameter("invalid hex character"));
    }

    hex_digits
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let high = hex_nibble(pair[0])?;
            let low = hex_nibble(pair[1])?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn decode_mixed(input: &str) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        if ch == '\'' {
            chars.next();
            output.extend(decode_quoted_segment(&mut chars)?);
        } else {
            output.extend(decode_hex_run(&mut chars)?);
        }
    }

    Ok(output)
}

fn decode_quoted_segment<I>(chars: &mut std::iter::Peekable<I>) -> Result<Vec<u8>>
where
    I: Iterator<Item = char>,
{
    let mut segment = Vec::new();
    loop {
        match chars.next() {
            Some('\'') => {
                if chars.peek() == Some(&'\'') {
                    chars.next();
                    segment.push(b'\'');
                } else {
                    return Ok(segment);
                }
            }
            Some(ch) => segment.push(ch as u8),
            None => return Err(Error::invalid_parameter("unterminated quoted string")),
        }
    }
}

fn decode_hex_run<I>(chars: &mut std::iter::Peekable<I>) -> Result<Vec<u8>>
where
    I: Iterator<Item = char>,
{
    let mut digits = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            break;
        }
        if ch == '\'' {
            break;
        }
        if !ch.is_ascii_hexdigit() {
            return Err(Error::invalid_parameter("invalid mixed hex character"));
        }
        digits.push(ch);
        chars.next();
    }

    if digits.is_empty() {
        return Err(Error::invalid_parameter("expected hex or quoted text"));
    }
    if !digits.len().is_multiple_of(2) {
        return Err(Error::invalid_parameter("odd hex digit count"));
    }

    digits
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let high = hex_nibble(pair[0])?;
            let low = hex_nibble(pair[1])?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn hex_nibble(byte: u8) -> Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(Error::invalid_parameter("invalid hex digit")),
    }
}

fn decode_base64(input: &str) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0u32;

    for ch in input.chars().filter(|c| !c.is_whitespace()) {
        if ch == '=' {
            break;
        }
        let value = base64_value(ch).ok_or_else(|| Error::invalid_parameter("invalid base64"))?;
        buffer = (buffer << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push(u8::try_from(buffer >> bits).expect("base64 emits bytes"));
            buffer &= (1 << bits) - 1;
        }
    }

    Ok(output)
}

fn base64_value(ch: char) -> Option<u32> {
    match ch {
        'A'..='Z' => Some(u32::from(ch as u8 - b'A')),
        'a'..='z' => Some(u32::from(ch as u8 - b'a' + 26)),
        '0'..='9' => Some(u32::from(ch as u8 - b'0' + 52)),
        '+' => Some(62),
        '/' => Some(63),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{DecodeFormat, decode_input};
    use crate::error::Error;

    /// spec `decode_mixed_quoted_and_hex`
    #[test]
    fn decode_mixed_quoted_and_hex() {
        assert_eq!(
            decode_input("'AB' 00", DecodeFormat::Mixed).expect("mixed decode"),
            b"AB\x00"
        );
        assert_eq!(
            decode_input("'a''b'", DecodeFormat::Mixed).expect("escaped quote"),
            b"a'b"
        );
        assert_eq!(
            decode_input("'hello' 09 'bob'", DecodeFormat::Mixed).expect("mixed sentence"),
            b"hello\tbob"
        );
    }

    /// spec `decode_bad_hex_is_param_error`
    #[test]
    fn decode_bad_hex_is_param_error() {
        assert!(matches!(
            decode_input("abc", DecodeFormat::Hexa),
            Err(Error::InvalidParameter { .. })
        ));
        assert!(matches!(
            decode_input("gg", DecodeFormat::Hexa),
            Err(Error::InvalidParameter { .. })
        ));
        assert!(matches!(
            decode_input("'open", DecodeFormat::Mixed),
            Err(Error::InvalidParameter { .. })
        ));
    }

    #[test]
    fn decode_hexa_ignores_whitespace() {
        assert_eq!(
            decode_input("01 02", DecodeFormat::Hexa).expect("hexa"),
            vec![0x01, 0x02]
        );
    }

    #[test]
    fn decode_data_is_utf8_bytes() {
        assert_eq!(
            decode_input("hello", DecodeFormat::Data).expect("data"),
            b"hello".to_vec()
        );
    }

    #[test]
    fn decode_base64_roundtrip_vector() {
        assert_eq!(
            decode_input("QUJD", DecodeFormat::Base64).expect("base64"),
            b"ABC".to_vec()
        );
    }

    #[test]
    fn decode_mixed_empty_hex_run_is_error() {
        assert!(decode_input("''", DecodeFormat::Mixed).is_ok());
        assert!(matches!(
            decode_input("xy", DecodeFormat::Mixed),
            Err(Error::InvalidParameter { .. })
        ));
    }

    #[test]
    fn decode_hexa_empty_input() {
        assert_eq!(
            decode_input("  ", DecodeFormat::Hexa).expect("empty"),
            Vec::new()
        );
    }

    #[test]
    fn decode_base64_invalid_character() {
        assert!(matches!(
            decode_input("Q@JD", DecodeFormat::Base64),
            Err(Error::InvalidParameter { .. })
        ));
    }

    #[test]
    fn decode_mixed_hex_only_run() {
        assert_eq!(
            decode_input("4142", DecodeFormat::Mixed).expect("hex run"),
            b"AB".to_vec()
        );
    }
}
