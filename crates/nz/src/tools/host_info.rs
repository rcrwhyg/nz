//! 工具 3：查询 IP / 主机名信息。
//!
//! 对照 `spec/netwox/info/003.md` 与 `tools/000003.c`。
//! CI 使用可注入的 [`HostResolver`]，不对公网。

use std::net::IpAddr;

use crate::tool_schemas::{text_meta_for_tool, tool3_schema};
use nz_arg::{ParseMode, ParseOutcome, ParsedArgs, parse};
use nz_net::{EthernetAddress, FakeLocalConfiguration, IpAddressSet, LocalConfiguration};

/// 工具 3 失败。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HostInfoError {
    /// 参数解析失败。
    Parse(String),
}

impl std::fmt::Display for HostInfoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(message) => write!(formatter, "{message}"),
        }
    }
}

impl std::error::Error for HostInfoError {}

/// 主机/以太网解析后端（测试注入假表）。
pub trait HostResolver {
    /// 将查询解析为第一个 IP；失败返回 `None`。
    fn resolve_first_ip(&self, query: &str) -> Option<IpAddr>;

    /// 主主机名。
    fn resolve_hostname(&self, address: IpAddr) -> Option<String>;

    /// 全部主机名。
    fn resolve_hostnames(&self, address: IpAddr) -> Vec<String>;

    /// Ethernet；失败返回 `None`。
    fn resolve_ethernet(&self, address: IpAddr) -> Option<EthernetAddress>;
}

/// CI / 本地假解析：回环、样例 ARP 邻居、显式失败名。
#[derive(Clone, Debug)]
pub struct FakeHostResolver {
    arp_table: FakeLocalConfiguration,
}

impl FakeHostResolver {
    /// 使用 [`FakeLocalConfiguration::sample`] 的 ARP 表。
    #[must_use]
    pub fn sample() -> Self {
        Self {
            arp_table: FakeLocalConfiguration::sample(),
        }
    }
}

impl HostResolver for FakeHostResolver {
    fn resolve_first_ip(&self, query: &str) -> Option<IpAddr> {
        if let Ok(address) = query.parse::<IpAddr>() {
            return Some(address);
        }
        match query {
            "localhost" | "localhost." => Some(IpAddr::from([127, 0, 0, 1])),
            _ => None,
        }
    }

    fn resolve_hostname(&self, address: IpAddr) -> Option<String> {
        if address == IpAddr::from([127, 0, 0, 1]) {
            Some(String::from("localhost"))
        } else {
            None
        }
    }

    fn resolve_hostnames(&self, address: IpAddr) -> Vec<String> {
        self.resolve_hostname(address).into_iter().collect()
    }

    fn resolve_ethernet(&self, address: IpAddr) -> Option<EthernetAddress> {
        self.arp_table
            .list_arp_entries()
            .ok()?
            .into_iter()
            .find(|entry| entry.ip == address)
            .map(|entry| entry.ethernet)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)] // 对齐原版五个显示开关 + all
struct DisplayFlags {
    title: bool,
    ip: bool,
    host: bool,
    hosts: bool,
    eth: bool,
    expand_all: bool,
}

impl DisplayFlags {
    fn from_parsed(values: &ParsedArgs) -> Self {
        Self {
            title: values.get_bool('t').unwrap_or(false),
            ip: values.get_bool('i').unwrap_or(false),
            host: values.get_bool('h').unwrap_or(false),
            hosts: values.get_bool('H').unwrap_or(false),
            eth: values.get_bool('e').unwrap_or(false),
            expand_all: values.get_bool('a').unwrap_or(false),
        }
    }

    fn effective_display(self) -> Self {
        if !self.title && !self.ip && !self.host && !self.hosts && !self.eth {
            Self {
                title: true,
                ip: true,
                host: true,
                hosts: true,
                eth: true,
                expand_all: self.expand_all,
            }
        } else {
            self
        }
    }
}

/// 使用 [`FakeHostResolver::sample`] 运行（默认入口）。
///
/// # Errors
///
/// 解析失败或查询非法。
pub fn run_host_info(tool_arguments: &[String]) -> Result<String, HostInfoError> {
    run_host_info_with(tool_arguments, &FakeHostResolver::sample())
}

/// 注入解析后端运行工具 3。
///
/// # Errors
///
/// 解析失败或查询非法。
pub fn run_host_info_with(
    tool_arguments: &[String],
    resolver: &impl HostResolver,
) -> Result<String, HostInfoError> {
    let schema = tool3_schema();
    match parse(&schema, tool_arguments, ParseMode::Cli) {
        Ok(ParseOutcome::Help { include_advanced }) => Ok(format_help(include_advanced)),
        Ok(ParseOutcome::Parsed(values)) => {
            let query = values
                .get_string('q')
                .ok_or_else(|| HostInfoError::Parse(String::from("missing --query")))?
                .to_owned();
            let flags = DisplayFlags::from_parsed(&values).effective_display();
            Ok(render_report(&query, flags, resolver))
        }
        Err(error) => Err(HostInfoError::Parse(error.to_string())),
    }
}

fn format_help(include_advanced: bool) -> String {
    let meta = text_meta_for_tool(3).expect("tool 3 meta");
    let mut lines = vec![
        meta.usage.to_owned(),
        meta.help.to_owned(),
        String::from("Options:"),
        String::from("  -q|--query   IP address or hostname (required)"),
        String::from("  -t|--title   display titles (advanced)"),
        String::from("  -i|--ip      obtain IP address (advanced)"),
        String::from("  -h|--host    obtain hostname (advanced)"),
        String::from("  -H|--hosts   obtain hostnames (advanced)"),
        String::from("  -e|--eth     obtain Ethernet address (advanced)"),
        String::from("  -a|--all     expand IP list/CIDR (advanced)"),
    ];
    if include_advanced {
        lines.push(String::from("(advanced options listed above)"));
    }
    lines.push(format!("Example: {}", meta.example));
    lines.join("\n")
}

fn render_report(query: &str, flags: DisplayFlags, resolver: &impl HostResolver) -> String {
    let addresses = if flags.expand_all {
        match IpAddressSet::parse(query) {
            Ok(set) => {
                let list: Vec<IpAddr> = set.iter().collect();
                if list.is_empty() {
                    vec![None]
                } else {
                    list.into_iter().map(Some).collect()
                }
            }
            Err(_) => vec![resolver.resolve_first_ip(query)],
        }
    } else {
        vec![resolver.resolve_first_ip(query)]
    };

    let mut blocks = Vec::new();
    for address in addresses {
        blocks.push(format_one(address, flags, resolver));
    }
    blocks.join("\n")
}

fn format_one(
    address: Option<IpAddr>,
    flags: DisplayFlags,
    resolver: &impl HostResolver,
) -> String {
    let mut lines = Vec::new();
    if flags.ip {
        lines.push(format_field(
            flags.title,
            "IP address:  ",
            address.map(|ip| ip.to_string()),
        ));
    }
    if flags.host {
        let hostname = address.and_then(|ip| resolver.resolve_hostname(ip));
        lines.push(format_field(flags.title, "Hostname:    ", hostname));
    }
    if flags.hosts {
        let hostnames = address
            .map(|ip| resolver.resolve_hostnames(ip))
            .unwrap_or_default();
        let text = if hostnames.is_empty() {
            None
        } else {
            Some(hostnames.join(" "))
        };
        lines.push(format_field(flags.title, "Hostnames:   ", text));
    }
    if flags.eth {
        let ethernet = address.and_then(|ip| resolver.resolve_ethernet(ip));
        lines.push(format_field(
            flags.title,
            "Eth address: ",
            ethernet.map(|mac| mac.to_string()),
        ));
    }
    lines.join("\n")
}

fn format_field(with_title: bool, title: &str, value: Option<String>) -> String {
    let body = value.unwrap_or_else(|| String::from("unresolved"));
    if with_title {
        format!("{title}{body}")
    } else {
        body
    }
}

#[cfg(test)]
mod tests {
    use super::{FakeHostResolver, run_host_info_with};

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|part| (*part).to_owned()).collect()
    }

    /// spec `t003_help_parses`
    #[test]
    fn t003_help_parses() {
        let output =
            run_host_info_with(&args(&["--help"]), &FakeHostResolver::sample()).expect("help");
        assert!(output.contains("query"));
        assert!(output.contains("title") || output.contains("--title"));
    }

    /// spec `t003_missing_query_fails`
    #[test]
    fn t003_missing_query_fails() {
        let error =
            run_host_info_with(&args(&[]), &FakeHostResolver::sample()).expect_err("need q");
        assert!(
            error.to_string().contains("query")
                || error.to_string().contains("required")
                || error.to_string().contains("missing")
        );
    }

    /// spec `t003_all_flags_off_prints_all`
    #[test]
    fn t003_all_flags_off_prints_all() {
        let output = run_host_info_with(&args(&["-q", "127.0.0.1"]), &FakeHostResolver::sample())
            .expect("run");
        assert!(output.contains("IP address:"));
        assert!(output.contains("127.0.0.1"));
        assert!(output.contains("Hostname:"));
    }

    /// spec `t003_ip_only`
    #[test]
    fn t003_ip_only() {
        let output = run_host_info_with(
            &args(&["-q", "127.0.0.1", "--ip", "--no-title"]),
            &FakeHostResolver::sample(),
        )
        .expect("run");
        assert!(output.contains("127.0.0.1"));
        assert!(!output.contains("Hostname:"));
        assert!(!output.contains("IP address:"));
    }

    /// spec `t003_unresolved_literal`
    #[test]
    fn t003_unresolved_literal() {
        let output = run_host_info_with(
            &args(&["-q", "unresolved.host"]),
            &FakeHostResolver::sample(),
        )
        .expect("run");
        assert!(output.contains("unresolved"));
    }

    /// spec `t003_all_expands_cidr`
    #[test]
    fn t003_all_expands_cidr() {
        let output = run_host_info_with(
            &args(&["-a", "-q", "10.0.0.0/30", "--ip", "--no-title"]),
            &FakeHostResolver::sample(),
        )
        .expect("run");
        assert!(output.contains("10.0.0.0"));
        assert!(output.contains("10.0.0.1"));
        assert!(output.contains("10.0.0.2"));
        assert!(output.contains("10.0.0.3"));
        assert!(output.matches("10.0.0.").count() >= 4);
    }
}
