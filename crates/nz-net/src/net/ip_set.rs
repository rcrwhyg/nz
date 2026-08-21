//! IP 地址集合（IPv4 / IPv6）。
//!
//! 支持单地址、闭区间、`/prefix`、`%prefix`（去网络/广播端点）、逗号、`all`、`!` 排除。

use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::error::{Error, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
enum IpInclude {
    All,
    Single(IpAddr),
    Range {
        start: IpRangeKey,
        end: IpRangeKey,
    },
    V4Cidr {
        network: u32,
        broadcast: u32,
        exclude_endpoints: bool,
    },
    V6Cidr {
        network: u128,
        broadcast: u128,
        exclude_endpoints: bool,
    },
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum IpRangeKey {
    V4(u32),
    V6(u128),
}

/// IP 地址集合。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IpAddressSet {
    includes: Vec<IpInclude>,
    excludes: HashSet<IpAddr>,
}

impl IpAddressSet {
    /// 解析集合文本。
    ///
    /// # Errors
    ///
    /// 语法非法或 prefix 越界时返回 [`Error::InvalidParameter`]。
    pub fn parse(text: &str) -> Result<Self> {
        let mut set = Self::default();
        for token in text.split(',') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            if let Some(excluded) = token.strip_prefix('!') {
                set.excludes.insert(parse_ip_address(excluded.trim())?);
            } else if token.eq_ignore_ascii_case("all") {
                set.includes.push(IpInclude::All);
            } else {
                set.push_include_token(token)?;
            }
        }
        Ok(set)
    }

    /// 是否包含地址。
    #[must_use]
    pub fn contains(&self, address: IpAddr) -> bool {
        if self.excludes.contains(&address) {
            return false;
        }
        self.includes.iter().any(|include| match include {
            IpInclude::All => true,
            IpInclude::Single(item) => *item == address,
            IpInclude::Range { start, end } => {
                range_contains(*start, *end, address_to_key(address))
            }
            IpInclude::V4Cidr {
                network,
                broadcast,
                exclude_endpoints,
            } => {
                if let IpAddr::V4(ip) = address {
                    let value = ipv4_to_u32(ip);
                    cidr_contains(*network, *broadcast, *exclude_endpoints, value)
                } else {
                    false
                }
            }
            IpInclude::V6Cidr {
                network,
                broadcast,
                exclude_endpoints,
            } => {
                if let IpAddr::V6(ip) = address {
                    let value = ipv6_to_u128(ip);
                    cidr_contains(*network, *broadcast, *exclude_endpoints, value)
                } else {
                    false
                }
            }
        })
    }

    /// 迭代有限展开（CIDR / 范围）；`all` 不枚举全空间。
    #[must_use]
    pub fn iter(&self) -> IpAddressSetIter<'_> {
        IpAddressSetIter::new(self)
    }

    /// 加入；重复忽略（spec `ip_add_duplicate_ok`）。
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
            self.excludes.insert(parse_ip_address(excluded.trim())?);
            return Ok(());
        }
        if token.eq_ignore_ascii_case("all") {
            if !self
                .includes
                .iter()
                .any(|item| matches!(item, IpInclude::All))
            {
                self.includes.push(IpInclude::All);
            }
            return Ok(());
        }
        self.push_include_token(token)
    }

    /// 删除；不存在忽略。
    ///
    /// # Errors
    ///
    /// 解析失败时返回 [`Error::InvalidParameter`]。
    pub fn remove(&mut self, token: &str) -> Result<()> {
        let token = token.trim();
        if let Some(excluded) = token.strip_prefix('!') {
            self.excludes.remove(&parse_ip_address(excluded.trim())?);
            return Ok(());
        }
        if token.eq_ignore_ascii_case("all") {
            self.includes.retain(|item| !matches!(item, IpInclude::All));
            return Ok(());
        }
        let parsed = parse_ip_include(token)?;
        self.includes.retain(|item| item != &parsed);
        Ok(())
    }

    fn push_include_token(&mut self, token: &str) -> Result<()> {
        let parsed = parse_ip_include(token)?;
        if self.includes.contains(&parsed) {
            return Ok(());
        }
        self.includes.push(parsed);
        Ok(())
    }
}

/// IP 集合迭代器（仅展开有限 include 项）。
pub struct IpAddressSetIter<'set> {
    set: &'set IpAddressSet,
    include_index: usize,
    current_v4: Option<u32>,
    end_v4: Option<u32>,
}

impl<'set> IpAddressSetIter<'set> {
    fn new(set: &'set IpAddressSet) -> Self {
        Self {
            set,
            include_index: 0,
            current_v4: None,
            end_v4: None,
        }
    }
}

impl Iterator for IpAddressSetIter<'_> {
    type Item = IpAddr;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let (Some(current), Some(end)) = (self.current_v4, self.end_v4) {
                if current <= end {
                    let addr = IpAddr::V4(u32_to_ipv4(current));
                    self.current_v4 = Some(current.saturating_add(1));
                    if self.set.contains(addr) {
                        return Some(addr);
                    }
                    continue;
                }
                self.current_v4 = None;
                self.end_v4 = None;
            }

            if self.include_index >= self.set.includes.len() {
                return None;
            }

            match &self.set.includes[self.include_index] {
                IpInclude::Single(value) => {
                    self.include_index += 1;
                    if self.set.contains(*value) {
                        return Some(*value);
                    }
                }
                IpInclude::Range { start, end } => {
                    self.include_index += 1;
                    if let (IpRangeKey::V4(start), IpRangeKey::V4(end)) = (start, end) {
                        self.current_v4 = Some(*start);
                        self.end_v4 = Some(*end);
                    }
                }
                IpInclude::V4Cidr {
                    network,
                    broadcast,
                    exclude_endpoints,
                } => {
                    self.include_index += 1;
                    let (start, end) = if *exclude_endpoints && broadcast > network {
                        (network.saturating_add(1), broadcast.saturating_sub(1))
                    } else {
                        (*network, *broadcast)
                    };
                    self.current_v4 = Some(start);
                    self.end_v4 = Some(end);
                }
                IpInclude::All | IpInclude::V6Cidr { .. } => {
                    self.include_index += 1;
                }
            }
        }
    }
}

fn parse_ip_address(text: &str) -> Result<IpAddr> {
    text.parse::<IpAddr>()
        .map_err(|_| Error::invalid_parameter("invalid ip address"))
}

fn parse_ip_include(token: &str) -> Result<IpInclude> {
    if let Some((left, right)) = token.split_once('%').or_else(|| token.split_once('/')) {
        let exclude_endpoints = token.contains('%');
        return parse_cidr(left.trim(), right.trim(), exclude_endpoints);
    }
    if let Some((left, right)) = token.split_once('-') {
        let start = address_to_key(parse_ip_address(left.trim())?);
        let end = address_to_key(parse_ip_address(right.trim())?);
        if start > end {
            return Err(Error::invalid_parameter("ip range start after end"));
        }
        return Ok(IpInclude::Range { start, end });
    }
    Ok(IpInclude::Single(parse_ip_address(token)?))
}

fn parse_cidr(address_text: &str, prefix_text: &str, exclude_endpoints: bool) -> Result<IpInclude> {
    let prefix: u8 = prefix_text
        .parse()
        .map_err(|_| Error::invalid_parameter("invalid ip prefix"))?;
    let address = parse_ip_with_short_form(address_text)?;
    match address {
        IpAddr::V4(ip) => {
            if prefix > 32 {
                return Err(Error::invalid_parameter("ipv4 prefix out of range"));
            }
            let base = ipv4_to_u32(normalize_ipv4_network(ip, prefix));
            let (network, broadcast) = ipv4_prefix_bounds(base, prefix);
            Ok(IpInclude::V4Cidr {
                network,
                broadcast,
                exclude_endpoints,
            })
        }
        IpAddr::V6(ip) => {
            if prefix > 128 {
                return Err(Error::invalid_parameter("ipv6 prefix out of range"));
            }
            let base = ipv6_to_u128(normalize_ipv6_network(ip, prefix));
            let (network, broadcast) = ipv6_prefix_bounds(base, prefix);
            Ok(IpInclude::V6Cidr {
                network,
                broadcast,
                exclude_endpoints,
            })
        }
    }
}

/// 解析可缺省末段的 IPv4 文本（如 `1.2.3/24` → `1.2.3.0/24`）。
fn parse_ip_with_short_form(text: &str) -> Result<IpAddr> {
    if text.contains(':') {
        return parse_ip_address(text);
    }
    let parts: Vec<&str> = text.split('.').collect();
    if parts.len() == 3 {
        let padded = format!("{text}.0");
        return parse_ip_address(&padded);
    }
    parse_ip_address(text)
}

fn normalize_ipv4_network(ip: Ipv4Addr, prefix: u8) -> Ipv4Addr {
    let value = ipv4_to_u32(ip);
    u32_to_ipv4(value & ipv4_mask(prefix))
}

fn normalize_ipv6_network(ip: Ipv6Addr, prefix: u8) -> Ipv6Addr {
    let value = ipv6_to_u128(ip);
    u128_to_ipv6(value & ipv6_mask(prefix))
}

fn ipv4_prefix_bounds(base: u32, prefix: u8) -> (u32, u32) {
    let mask = ipv4_mask(prefix);
    let network = base & mask;
    let broadcast = network | !mask;
    (network, broadcast)
}

fn ipv6_prefix_bounds(base: u128, prefix: u8) -> (u128, u128) {
    let mask = ipv6_mask(prefix);
    let network = base & mask;
    let broadcast = network | !mask;
    (network, broadcast)
}

fn ipv4_mask(prefix: u8) -> u32 {
    if prefix == 0 {
        0
    } else {
        (!0u32) << (32 - prefix)
    }
}

fn ipv6_mask(prefix: u8) -> u128 {
    if prefix == 0 {
        0
    } else {
        (!0u128) << (128 - prefix)
    }
}

fn cidr_contains<T: Ord + Copy>(
    network: T,
    broadcast: T,
    exclude_endpoints: bool,
    value: T,
) -> bool {
    if value < network || value > broadcast {
        return false;
    }
    if exclude_endpoints && broadcast > network && (value == network || value == broadcast) {
        return false;
    }
    true
}

fn range_contains(start: IpRangeKey, end: IpRangeKey, value: IpRangeKey) -> bool {
    start <= value && value <= end
}

fn address_to_key(address: IpAddr) -> IpRangeKey {
    match address {
        IpAddr::V4(ip) => IpRangeKey::V4(ipv4_to_u32(ip)),
        IpAddr::V6(ip) => IpRangeKey::V6(ipv6_to_u128(ip)),
    }
}

fn ipv4_to_u32(ip: Ipv4Addr) -> u32 {
    u32::from_be_bytes(ip.octets())
}

fn u32_to_ipv4(value: u32) -> Ipv4Addr {
    Ipv4Addr::from(value.to_be_bytes())
}

fn ipv6_to_u128(ip: Ipv6Addr) -> u128 {
    u128::from_be_bytes(ip.octets())
}

fn u128_to_ipv6(value: u128) -> Ipv6Addr {
    Ipv6Addr::from(value.to_be_bytes())
}

impl<'set> IntoIterator for &'set IpAddressSet {
    type Item = IpAddr;
    type IntoIter = IpAddressSetIter<'set>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use super::IpAddressSet;

    /// spec `ip_parse_single_and_cidr`
    #[test]
    fn ip_parse_single_and_cidr() {
        let single = IpAddressSet::parse("1.2.3.4").expect("single");
        assert!(single.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))));

        let cidr = IpAddressSet::parse("1.2.3.0/24").expect("cidr");
        assert!(cidr.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 0))));
        assert!(cidr.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 255))));
        assert!(!cidr.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 4, 0))));
        let expanded: Vec<_> = cidr.iter().collect();
        assert_eq!(expanded.len(), 256);
    }

    /// spec `ip_percent_excludes_ends`
    #[test]
    fn ip_percent_excludes_ends() {
        let set = IpAddressSet::parse("1.2.3.0%24").expect("percent");
        assert!(!set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 0))));
        assert!(!set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 255))));
        assert!(set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 1))));
        assert!(set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 254))));
    }

    /// spec `ip_list_all_not_and_comma`
    #[test]
    fn ip_list_all_not_and_comma() {
        let set = IpAddressSet::parse("all,!1.2.3.4").expect("all not");
        assert!(!set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))));
        assert!(set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 5))));
    }

    /// spec `ip_add_duplicate_ok`
    #[test]
    fn ip_add_duplicate_ok() {
        let mut set = IpAddressSet::parse("1.2.3.4").expect("parse");
        set.add("1.2.3.4").expect("duplicate");
        assert_eq!(
            set.iter().collect::<Vec<_>>(),
            vec![IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))]
        );
    }

    #[test]
    fn ip_short_form_three_octets() {
        let set = IpAddressSet::parse("1.2.3/24").expect("short");
        assert!(set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 10))));
    }

    #[test]
    fn ip_range_and_remove() {
        let mut set = IpAddressSet::parse("1.2.3.4-1.2.3.6").expect("range");
        assert!(set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 5))));
        set.remove("1.2.3.4-1.2.3.6").expect("remove");
        assert!(!set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 5))));
    }

    #[test]
    fn ip_slash_vs_percent_on_network() {
        let slash = IpAddressSet::parse("10.0.0.0/24").expect("slash");
        assert!(slash.contains(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0))));
        let percent = IpAddressSet::parse("10.0.0.0%24").expect("percent");
        assert!(!percent.contains(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0))));
    }

    #[test]
    fn ip_add_exclude_and_empty_token() {
        let mut set = IpAddressSet::default();
        set.add("").expect("empty");
        set.add("!1.2.3.4").expect("exclude");
        set.add("1.2.3.5").expect("include");
        assert!(!set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))));
        assert!(set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 5))));
    }

    #[test]
    fn ip_ipv6_cidr_and_errors() {
        use std::net::Ipv6Addr;

        let v6 = IpAddressSet::parse("2001:db8::/32").expect("v6");
        assert!(v6.contains(IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1))));
        assert!(IpAddressSet::parse("1.2.3.4/33").is_err());
        assert!(IpAddressSet::parse("3.4.5.6-1.2.3.4").is_err());
        let mut set = IpAddressSet::default();
        set.add("all").expect("all");
        set.remove("all").expect("remove all");
    }

    #[test]
    fn ip_set_into_iterator() {
        let set = IpAddressSet::parse("1.2.3.4").expect("single");
        assert_eq!(
            (&set).into_iter().collect::<Vec<_>>(),
            vec![IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))]
        );
    }

    #[test]
    fn ip_v6_percent_excludes_endpoints() {
        use std::net::Ipv6Addr;

        let set = IpAddressSet::parse("2001:db8::%126").expect("v6 percent");
        assert!(!set.contains(IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 0))));
        assert!(set.contains(IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1))));
    }

    #[test]
    fn ip_remove_exclude_and_invalid_parse() {
        let mut set = IpAddressSet::parse("all,!1.2.3.4").expect("parse");
        set.remove("!1.2.3.4").expect("unexclude");
        assert!(set.contains(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))));
        assert!(IpAddressSet::parse("not-an-ip").is_err());
    }
}
