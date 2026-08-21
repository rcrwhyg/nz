//! 工具 4：由 Ethernet 地址反查 IP / 主机名。
//!
//! 对照 `spec/netwox/info/004.md` 与 `tools/000004.c`。假 ARP 表，不对公网。

use std::net::IpAddr;
use std::str::FromStr;

use crate::tool_schemas::{text_meta_for_tool, tool4_schema};
use crate::tools::host_info::{FakeHostResolver, HostResolver};
use nz_arg::{ParseMode, ParseOutcome, ParsedArgs, parse};
use nz_net::{EthernetAddress, FakeLocalConfiguration, LocalConfiguration};

/// 工具 4 失败。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EthInfoError {
    /// 参数解析失败。
    Parse(String),
}

impl std::fmt::Display for EthInfoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(message) => write!(formatter, "{message}"),
        }
    }
}

impl std::error::Error for EthInfoError {}

/// Ethernet → IP 反查后端。
pub trait EthResolver {
    /// 由 MAC 查 IP。
    fn resolve_ip(&self, ethernet: EthernetAddress) -> Option<IpAddr>;
}

impl EthResolver for FakeLocalConfiguration {
    fn resolve_ip(&self, ethernet: EthernetAddress) -> Option<IpAddr> {
        self.list_arp_entries()
            .ok()?
            .into_iter()
            .find(|entry| entry.ethernet == ethernet)
            .map(|entry| entry.ip)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)] // 对齐原版四个显示开关
struct DisplayFlags {
    title: bool,
    ip: bool,
    host: bool,
    hosts: bool,
}

impl DisplayFlags {
    fn from_parsed(values: &ParsedArgs) -> Self {
        Self {
            title: values.get_bool('t').unwrap_or(false),
            ip: values.get_bool('i').unwrap_or(false),
            host: values.get_bool('h').unwrap_or(false),
            hosts: values.get_bool('H').unwrap_or(false),
        }
    }

    fn effective(self) -> Self {
        if !self.title && !self.ip && !self.host && !self.hosts {
            Self {
                title: true,
                ip: true,
                host: true,
                hosts: true,
            }
        } else {
            self
        }
    }
}

/// 默认假表入口。
///
/// # Errors
///
/// 解析失败或 eth 非法。
pub fn run_eth_info(tool_arguments: &[String]) -> Result<String, EthInfoError> {
    run_eth_info_with(
        tool_arguments,
        &FakeLocalConfiguration::sample(),
        &FakeHostResolver::sample(),
    )
}

/// 注入 ARP / 主机名后端。
///
/// # Errors
///
/// 解析失败或 eth 非法。
pub fn run_eth_info_with(
    tool_arguments: &[String],
    eth_resolver: &impl EthResolver,
    host_resolver: &impl HostResolver,
) -> Result<String, EthInfoError> {
    let schema = tool4_schema();
    match parse(&schema, tool_arguments, ParseMode::Cli) {
        Ok(ParseOutcome::Help { include_advanced }) => Ok(format_help(include_advanced)),
        Ok(ParseOutcome::Parsed(values)) => {
            let eth_text = values
                .get_string('e')
                .ok_or_else(|| EthInfoError::Parse(String::from("missing --eth")))?;
            let ethernet = EthernetAddress::from_str(eth_text).map_err(|error| {
                EthInfoError::Parse(format!("invalid ethernet address: {error}"))
            })?;
            let flags = DisplayFlags::from_parsed(&values).effective();
            Ok(render_report(ethernet, flags, eth_resolver, host_resolver))
        }
        Err(error) => Err(EthInfoError::Parse(error.to_string())),
    }
}

fn format_help(include_advanced: bool) -> String {
    let meta = text_meta_for_tool(4).expect("tool 4 meta");
    let mut lines = vec![
        meta.usage.to_owned(),
        meta.help.to_owned(),
        String::from("Options:"),
        String::from("  -e|--eth     Ethernet address (required)"),
        String::from("  -t|--title   display titles (advanced)"),
        String::from("  -i|--ip      obtain IP address (advanced)"),
        String::from("  -h|--host    obtain hostname (advanced)"),
        String::from("  -H|--hosts   obtain hostnames (advanced)"),
    ];
    if include_advanced {
        lines.push(String::from("(advanced options listed above)"));
    }
    lines.push(format!("Example: {}", meta.example));
    lines.join("\n")
}

fn render_report(
    ethernet: EthernetAddress,
    flags: DisplayFlags,
    eth_resolver: &impl EthResolver,
    host_resolver: &impl HostResolver,
) -> String {
    let address = eth_resolver.resolve_ip(ethernet);
    let mut lines = Vec::new();
    if flags.ip {
        lines.push(format_field(
            flags.title,
            "IP address:  ",
            address.map(|ip| ip.to_string()),
        ));
    }
    if flags.host {
        let hostname = address.and_then(|ip| host_resolver.resolve_hostname(ip));
        lines.push(format_field(flags.title, "Hostname:    ", hostname));
    }
    if flags.hosts {
        let hostnames = address
            .map(|ip| host_resolver.resolve_hostnames(ip))
            .unwrap_or_default();
        let text = if hostnames.is_empty() {
            None
        } else {
            Some(hostnames.join(" "))
        };
        lines.push(format_field(flags.title, "Hostnames:   ", text));
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
    use super::run_eth_info_with;
    use crate::tools::host_info::FakeHostResolver;
    use nz_net::FakeLocalConfiguration;

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|part| (*part).to_owned()).collect()
    }

    fn backends() -> (FakeLocalConfiguration, FakeHostResolver) {
        (FakeLocalConfiguration::sample(), FakeHostResolver::sample())
    }

    /// spec `t004_help_parses`
    #[test]
    fn t004_help_parses() {
        let (eth, host) = backends();
        let output = run_eth_info_with(&args(&["--help"]), &eth, &host).expect("help");
        assert!(output.contains("eth"));
    }

    /// spec `t004_missing_eth_fails`
    #[test]
    fn t004_missing_eth_fails() {
        let (eth, host) = backends();
        let error = run_eth_info_with(&args(&[]), &eth, &host).expect_err("need eth");
        assert!(
            error.to_string().contains("eth")
                || error.to_string().contains("required")
                || error.to_string().contains("missing")
        );
    }

    /// spec `t004_all_off_prints_all`
    #[test]
    fn t004_all_off_prints_all() {
        let (eth, host) = backends();
        let output =
            run_eth_info_with(&args(&["-e", "aa:bb:cc:dd:ee:01"]), &eth, &host).expect("run");
        assert!(output.contains("IP address:"));
        assert!(output.contains("192.168.1.10"));
    }

    /// spec `t004_unresolved_literal`
    #[test]
    fn t004_unresolved_literal() {
        let (eth, host) = backends();
        let output =
            run_eth_info_with(&args(&["-e", "00:00:00:00:00:00"]), &eth, &host).expect("run");
        assert!(output.contains("unresolved"));
    }
}
