//! 6 字节 Ethernet 地址及其集合。
//!
//! 文本形态为冒号分隔十六进制（大小写不敏感）。集合支持列表、范围、`/prefix` 与 `%prefix`。

use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};

/// 6 字节 Ethernet 地址。
///
/// [`Display`] 输出为小写 `aa:bb:cc:dd:ee:ff` 形态（spec `eth_parse_colon`）。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EthernetAddress([u8; 6]);

impl EthernetAddress {
    /// 从 6 字节构造。
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }

    /// 只读字节视图。
    #[must_use]
    pub const fn octets(&self) -> &[u8; 6] {
        &self.0
    }

    /// 转为 `u64`（高 16 位为 0），便于范围与 prefix 运算。
    #[must_use]
    pub fn to_u64(&self) -> u64 {
        u64::from_be_bytes([
            0, 0, self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
        ])
    }

    /// 从 `u64` 低 48 位构造。
    #[must_use]
    pub fn from_u64(value: u64) -> Self {
        let bytes = value.to_be_bytes();
        Self([bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
    }
}

impl fmt::Display for EthernetAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl FromStr for EthernetAddress {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self> {
        parse_ethernet_address(text)
    }
}

/// 解析 `aa:bb:cc:dd:ee:ff`（大小写均可）。
pub fn parse_ethernet_address(text: &str) -> Result<EthernetAddress> {
    let parts: Vec<&str> = text.split(':').collect();
    if parts.len() != 6 {
        return Err(Error::invalid_parameter(
            "ethernet address requires 6 octets",
        ));
    }
    let mut octets = [0u8; 6];
    for (index, part) in parts.iter().enumerate() {
        if part.is_empty() || part.len() > 2 || !part.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return Err(Error::invalid_parameter("invalid ethernet octet"));
        }
        let padded = if part.len() == 1 {
            format!("0{part}")
        } else {
            (*part).to_string()
        };
        octets[index] = u8::from_str_radix(&padded, 16)
            .map_err(|_| Error::invalid_parameter("invalid ethernet hex"))?;
    }
    Ok(EthernetAddress(octets))
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum EthernetInclude {
    All,
    Single(EthernetAddress),
    Range { start: u64, end: u64 },
}

/// Ethernet 地址集合。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EthernetAddressSet {
    includes: Vec<EthernetInclude>,
    excludes: HashSet<EthernetAddress>,
}

impl EthernetAddressSet {
    /// 解析集合文本（逗号分隔；`all`；`!` 排除）。
    ///
    /// # Errors
    ///
    /// 语法非法或 prefix 越界时返回 [`Error::InvalidParameter`]。
    pub fn parse(text: &str) -> Result<Self> {
        let mut set = Self::default();
        for token in split_top_level_commas(text) {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            if let Some(excluded) = token.strip_prefix('!') {
                set.excludes
                    .insert(parse_ethernet_address(excluded.trim())?);
            } else if token.eq_ignore_ascii_case("all") {
                set.includes.push(EthernetInclude::All);
            } else {
                set.push_include_token(token)?;
            }
        }
        Ok(set)
    }

    /// 是否包含地址。
    #[must_use]
    pub fn contains(&self, address: EthernetAddress) -> bool {
        let value = address.to_u64();
        if self.excludes.contains(&address) {
            return false;
        }
        self.includes.iter().any(|include| match include {
            EthernetInclude::All => true,
            EthernetInclude::Single(item) => *item == address,
            EthernetInclude::Range { start, end } => *start <= value && value <= *end,
        })
    }

    /// 加入条目；已存在则忽略（spec 与 IP 集合相同语义）。
    ///
    /// # Errors
    ///
    /// 解析失败时返回 [`Error::InvalidParameter`]。
    pub fn add(&mut self, token: &str) -> Result<()> {
        let token = token.trim();
        if token.is_empty() {
            return Ok(());
        }
        if let Some(excluded) = token.strip_prefix('!') {
            let address = parse_ethernet_address(excluded.trim())?;
            self.excludes.insert(address);
            return Ok(());
        }
        if token.eq_ignore_ascii_case("all") {
            if !self
                .includes
                .iter()
                .any(|item| matches!(item, EthernetInclude::All))
            {
                self.includes.push(EthernetInclude::All);
            }
            return Ok(());
        }
        self.push_include_token(token)
    }

    /// 删除条目；不存在则忽略。
    ///
    /// # Errors
    ///
    /// 解析失败时返回 [`Error::InvalidParameter`]。
    pub fn remove(&mut self, token: &str) -> Result<()> {
        let token = token.trim();
        if let Some(excluded) = token.strip_prefix('!') {
            let address = parse_ethernet_address(excluded.trim())?;
            self.excludes.remove(&address);
            return Ok(());
        }
        if token.eq_ignore_ascii_case("all") {
            self.includes
                .retain(|item| !matches!(item, EthernetInclude::All));
            return Ok(());
        }
        let parsed = parse_ethernet_include(token)?;
        self.includes.retain(|item| item != &parsed);
        Ok(())
    }

    fn push_include_token(&mut self, token: &str) -> Result<()> {
        let parsed = parse_ethernet_include(token)?;
        if self.includes.contains(&parsed) {
            return Ok(());
        }
        self.includes.push(parsed);
        Ok(())
    }
}

fn parse_ethernet_include(token: &str) -> Result<EthernetInclude> {
    if let Some((left, right)) = token.split_once('%').or_else(|| token.split_once('/')) {
        let exclude_ends = token.contains('%');
        let base = parse_ethernet_address(left.trim())?;
        let prefix: u8 = right
            .trim()
            .parse()
            .map_err(|_| Error::invalid_parameter("invalid ethernet prefix"))?;
        if prefix > 48 {
            return Err(Error::invalid_parameter("ethernet prefix out of range"));
        }
        let (start, end) = prefix_bounds(base.to_u64(), prefix, exclude_ends);
        return Ok(EthernetInclude::Range { start, end });
    }
    if let Some((left, right)) = token.split_once('-') {
        let start = parse_ethernet_address(left.trim())?.to_u64();
        let end = parse_ethernet_address(right.trim())?.to_u64();
        if start > end {
            return Err(Error::invalid_parameter("ethernet range start after end"));
        }
        return Ok(EthernetInclude::Range { start, end });
    }
    Ok(EthernetInclude::Single(parse_ethernet_address(token)?))
}

fn prefix_bounds(base: u64, prefix: u8, exclude_ends: bool) -> (u64, u64) {
    const ADDRESS_BITS: u64 = 48;
    let host_bits = ADDRESS_BITS.saturating_sub(u64::from(prefix));
    let host_mask = if host_bits >= ADDRESS_BITS {
        (1u64 << ADDRESS_BITS) - 1
    } else {
        (1u64 << host_bits) - 1
    };
    let network_mask = ((1u64 << ADDRESS_BITS) - 1) & !host_mask;
    let network = base & network_mask;
    let broadcast = network | host_mask;
    if exclude_ends && broadcast > network {
        (network + 1, broadcast - 1)
    } else {
        (network, broadcast)
    }
}

fn split_top_level_commas(text: &str) -> Vec<&str> {
    text.split(',').collect()
}

#[cfg(test)]
mod tests {
    use super::{EthernetAddressSet, parse_ethernet_address};

    /// spec `eth_parse_colon`
    #[test]
    fn eth_parse_colon() {
        let lower = parse_ethernet_address("aa:bb:cc:dd:ee:ff").expect("lower");
        let upper = parse_ethernet_address("AA:BB:CC:DD:EE:FF").expect("upper");
        assert_eq!(lower, upper);
        assert_eq!(lower.to_string(), "aa:bb:cc:dd:ee:ff");
    }

    /// spec `eth_percent_excludes_ends`
    #[test]
    fn eth_percent_excludes_ends() {
        let set = EthernetAddressSet::parse("a:b:c:d:e:0%40").expect("parse");
        let low = parse_ethernet_address("a:b:c:d:e:00").expect("low");
        let high = parse_ethernet_address("a:b:c:d:e:ff").expect("high");
        let mid = parse_ethernet_address("a:b:c:d:e:01").expect("mid");
        assert!(!set.contains(low));
        assert!(!set.contains(high));
        assert!(set.contains(mid));
    }

    #[test]
    fn eth_set_add_remove_and_range() {
        let mut set =
            EthernetAddressSet::parse("aa:bb:cc:dd:ee:00-aa:bb:cc:dd:ee:02").expect("range");
        assert!(set.contains(parse_ethernet_address("aa:bb:cc:dd:ee:01").expect("inside")));
        set.add("aa:bb:cc:dd:ee:00-aa:bb:cc:dd:ee:02")
            .expect("dup range");
        set.remove("aa:bb:cc:dd:ee:00-aa:bb:cc:dd:ee:02")
            .expect("remove");
        assert!(!set.contains(parse_ethernet_address("aa:bb:cc:dd:ee:01").expect("inside")));
        set.remove("!aa:bb:cc:dd:ee:ff")
            .expect("remove exclude noop");
    }

    #[test]
    fn eth_set_all_with_exclude() {
        let set = EthernetAddressSet::parse("all,!aa:bb:cc:dd:ee:ff").expect("all");
        assert!(!set.contains(parse_ethernet_address("aa:bb:cc:dd:ee:ff").expect("excluded")));
        assert!(set.contains(parse_ethernet_address("aa:bb:cc:dd:ee:00").expect("included")));
    }

    #[test]
    fn eth_parse_and_prefix_errors() {
        assert!(parse_ethernet_address("aa:bb:cc:dd:ee").is_err());
        assert!(parse_ethernet_address("gg:bb:cc:dd:ee:ff").is_err());
        assert!(EthernetAddressSet::parse("aa:bb:cc:dd:ee:ff-aa:bb:cc:dd:ee:00").is_err());
        assert!(EthernetAddressSet::parse("aa:bb:cc:dd:ee:ff/49").is_err());
        let slash = EthernetAddressSet::parse("aa:bb:cc:dd:ee:00/48").expect("slash");
        assert!(slash.contains(parse_ethernet_address("aa:bb:cc:dd:ee:00").expect("host")));
    }

    #[test]
    fn eth_add_all_and_remove_all() {
        let mut set = EthernetAddressSet::default();
        set.add("all").expect("all");
        set.add("all").expect("dup all");
        set.remove("all").expect("remove all");
        assert!(!set.contains(parse_ethernet_address("aa:bb:cc:dd:ee:00").expect("any")));
    }

    #[test]
    fn eth_slash_prefix_and_exclude_remove() {
        let mut set = EthernetAddressSet::parse("all,!aa:bb:cc:dd:ee:01").expect("parse");
        set.remove("!aa:bb:cc:dd:ee:01").expect("unexclude");
        assert!(set.contains(parse_ethernet_address("aa:bb:cc:dd:ee:01").expect("now included")));
        let prefix_zero = EthernetAddressSet::parse("aa:bb:cc:dd:ee:ff/0").expect("prefix0");
        assert!(
            prefix_zero.contains(parse_ethernet_address("00:11:22:33:44:55").expect("any mac"))
        );
    }

    #[test]
    fn eth_empty_octet_and_from_str() {
        assert!(parse_ethernet_address("aa:bb:cc:dd:ee:").is_err());
        let address: super::EthernetAddress = "aa:bb:cc:dd:ee:ff".parse().expect("from str");
        assert_eq!(
            address,
            parse_ethernet_address("aa:bb:cc:dd:ee:ff").expect("parse")
        );
        assert_eq!(super::EthernetAddress::from_u64(address.to_u64()), address);
    }
}
