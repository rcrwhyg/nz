//! TCP/UDP 端口集合。
//!
//! 支持单值、`a-b` 闭区间、逗号、`all` 与 `!` 排除。

use std::collections::HashSet;

use crate::error::{Error, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
enum PortInclude {
    All,
    Single(u16),
    Range { start: u16, end: u16 },
}

/// 端口集合（`u16`）。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PortSet {
    includes: Vec<PortInclude>,
    excludes: HashSet<u16>,
}

impl PortSet {
    /// 解析集合文本。
    ///
    /// # Errors
    ///
    /// 语法非法或范围颠倒时返回 [`Error::InvalidParameter`]。
    pub fn parse(text: &str) -> Result<Self> {
        let mut set = Self::default();
        for token in text.split(',') {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }
            if let Some(excluded) = token.strip_prefix('!') {
                set.excludes.insert(parse_port(excluded.trim())?);
            } else if token.eq_ignore_ascii_case("all") {
                set.includes.push(PortInclude::All);
            } else {
                set.push_include_token(token)?;
            }
        }
        Ok(set)
    }

    /// 是否包含端口。
    #[must_use]
    pub fn contains(&self, port: u16) -> bool {
        if self.excludes.contains(&port) {
            return false;
        }
        self.includes.iter().any(|include| match include {
            PortInclude::All => true,
            PortInclude::Single(value) => *value == port,
            PortInclude::Range { start, end } => *start <= port && port <= *end,
        })
    }

    /// 迭代包含的端口（有限集合；`all` 不含排除时遍历 1..=65535）。
    #[must_use]
    pub fn iter(&self) -> PortSetIter<'_> {
        PortSetIter::new(self)
    }

    /// 加入；重复忽略。
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
            self.excludes.insert(parse_port(excluded.trim())?);
            return Ok(());
        }
        if token.eq_ignore_ascii_case("all") {
            if !self
                .includes
                .iter()
                .any(|item| matches!(item, PortInclude::All))
            {
                self.includes.push(PortInclude::All);
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
            self.excludes.remove(&parse_port(excluded.trim())?);
            return Ok(());
        }
        if token.eq_ignore_ascii_case("all") {
            self.includes
                .retain(|item| !matches!(item, PortInclude::All));
            return Ok(());
        }
        let parsed = parse_port_include(token)?;
        self.includes.retain(|item| item != &parsed);
        Ok(())
    }

    fn push_include_token(&mut self, token: &str) -> Result<()> {
        let parsed = parse_port_include(token)?;
        if self.includes.contains(&parsed) {
            return Ok(());
        }
        self.includes.push(parsed);
        Ok(())
    }
}

/// 端口集合迭代器。
pub struct PortSetIter<'set> {
    set: &'set PortSet,
    include_index: usize,
    current: u16,
    range_end: Option<u16>,
    all_mode: bool,
}

impl<'set> PortSetIter<'set> {
    fn new(set: &'set PortSet) -> Self {
        Self {
            set,
            include_index: 0,
            current: 0,
            range_end: None,
            all_mode: false,
        }
    }
}

impl Iterator for PortSetIter<'_> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.all_mode {
                while self.current != 0 {
                    let port = self.current;
                    self.current = if port == u16::MAX { 0 } else { port + 1 };
                    if self.set.contains(port) {
                        return Some(port);
                    }
                }
                return None;
            }

            if let Some(end) = self.range_end {
                if self.current <= end {
                    let port = self.current;
                    self.current += 1;
                    if self.set.contains(port) {
                        return Some(port);
                    }
                    continue;
                }
                self.range_end = None;
            }

            if self.include_index >= self.set.includes.len() {
                return None;
            }

            match &self.set.includes[self.include_index] {
                PortInclude::All => {
                    self.include_index += 1;
                    self.all_mode = true;
                    self.current = 1;
                }
                PortInclude::Single(value) => {
                    self.include_index += 1;
                    if self.set.contains(*value) {
                        return Some(*value);
                    }
                }
                PortInclude::Range { start, end } => {
                    self.include_index += 1;
                    self.current = *start;
                    self.range_end = Some(*end);
                }
            }
        }
    }
}

fn parse_port(text: &str) -> Result<u16> {
    text.parse::<u16>()
        .map_err(|_| Error::invalid_parameter("invalid port number"))
}

fn parse_port_include(token: &str) -> Result<PortInclude> {
    if let Some((left, right)) = token.split_once('-') {
        let start = parse_port(left.trim())?;
        let end = parse_port(right.trim())?;
        if start > end {
            return Err(Error::invalid_parameter("port range start after end"));
        }
        return Ok(PortInclude::Range { start, end });
    }
    Ok(PortInclude::Single(parse_port(token)?))
}

impl<'set> IntoIterator for &'set PortSet {
    type Item = u16;
    type IntoIter = PortSetIter<'set>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::PortSet;

    /// spec `port_range_iter`
    #[test]
    fn port_range_iter() {
        let set = PortSet::parse("80-82").expect("parse");
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![80, 81, 82]);
    }

    #[test]
    fn port_all_with_exclude() {
        let set = PortSet::parse("all,!80").expect("parse");
        assert!(!set.contains(80));
        assert!(set.contains(81));
    }

    #[test]
    fn port_add_duplicate_ok() {
        let mut set = PortSet::parse("80").expect("parse");
        set.add("80").expect("duplicate add");
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![80]);
    }

    #[test]
    fn port_remove_and_all_mode_iter() {
        let mut set = PortSet::parse("80,81").expect("singles");
        set.remove("80").expect("remove single");
        assert_eq!(set.iter().collect::<Vec<_>>(), vec![81]);
        let all = PortSet::parse("all,!2").expect("all");
        let first: Vec<_> = all.iter().take(3).collect();
        assert_eq!(first, vec![1, 3, 4]);
    }

    #[test]
    fn port_invalid_range_errors() {
        assert!(PortSet::parse("90-80").is_err());
    }

    #[test]
    fn port_add_all_remove_and_into_iter() {
        let mut set = PortSet::default();
        set.add("all").expect("all");
        set.add("all").expect("dup");
        set.remove("all").expect("remove");
        set.add("80-82").expect("range");
        assert_eq!(set.into_iter().collect::<Vec<_>>(), vec![80, 81, 82]);
    }

    #[test]
    fn port_remove_exclude() {
        let mut set = PortSet::parse("all,!80").expect("parse");
        set.remove("!80").expect("remove exclude");
        assert!(set.contains(80));
    }
}
