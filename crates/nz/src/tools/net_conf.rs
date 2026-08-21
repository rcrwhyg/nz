//! 工具 1：显示本机网络配置。
//!
//! 对照 `spec/netwox/info/001.md` 与 `tools/000001.c`：
//! 显示类布尔全关（或未开）则四块都打；任一开启则只打开启块。

use crate::tool_schemas::{text_meta_for_tool, tool1_schema};
use nz_arg::{ParseMode, ParseOutcome, ParsedArgs, parse};
use nz_net::{
    ArpEntry, Device, FakeLocalConfiguration, IpOnDevice, LocalConfiguration, Route, RouteSource,
    SystemLocalConfiguration,
};

/// 工具 1 失败。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NetConfError {
    /// 参数解析失败。
    Parse(String),
    /// 读配置失败。
    Conf(String),
}

impl std::fmt::Display for NetConfError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(message) | Self::Conf(message) => write!(formatter, "{message}"),
        }
    }
}

impl std::error::Error for NetConfError {}

/// 四块显示开关（解析后；「全关」表示打全部）。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_excessive_bools)] // 对齐原版四个独立显示开关
struct DisplayFlags {
    devices: bool,
    ip: bool,
    arpcache: bool,
    routes: bool,
}

impl DisplayFlags {
    fn from_parsed(values: &ParsedArgs) -> Self {
        Self {
            devices: values.get_bool('d').unwrap_or(false),
            ip: values.get_bool('i').unwrap_or(false),
            arpcache: values.get_bool('a').unwrap_or(false),
            routes: values.get_bool('r').unwrap_or(false),
        }
    }

    /// 任一开启则只打开启块；否则四块全打（对齐原版 `!disp`）。
    fn effective(self) -> Self {
        if self.devices || self.ip || self.arpcache || self.routes {
            self
        } else {
            Self {
                devices: true,
                ip: true,
                arpcache: true,
                routes: true,
            }
        }
    }
}

/// 使用自动后端（真系统优先，失败回落假表）运行工具 1。
///
/// `tool_arguments` 为跳过程序名与工具选择后的参数。帮助文本也作为 `Ok` 返回。
///
/// # Errors
///
/// 解析失败或读配置失败。
pub fn run_net_conf(tool_arguments: &[String]) -> Result<String, NetConfError> {
    match SystemLocalConfiguration::query() {
        Ok(configuration) => run_net_conf_with(tool_arguments, &configuration),
        Err(_) => run_net_conf_with(tool_arguments, &FakeLocalConfiguration::sample()),
    }
}

/// 使用给定配置后端运行工具 1（测试注入假表）。
///
/// # Errors
///
/// 解析失败或列举配置失败。
pub fn run_net_conf_with(
    tool_arguments: &[String],
    configuration: &impl LocalConfiguration,
) -> Result<String, NetConfError> {
    let schema = tool1_schema();
    match parse(&schema, tool_arguments, ParseMode::Cli) {
        Ok(ParseOutcome::Help { include_advanced }) => Ok(format_help(include_advanced)),
        Ok(ParseOutcome::Parsed(values)) => {
            let flags = DisplayFlags::from_parsed(&values).effective();
            render_sections(configuration, flags)
        }
        Err(error) => Err(NetConfError::Parse(error.to_string())),
    }
}

fn format_help(include_advanced: bool) -> String {
    let meta = text_meta_for_tool(1).expect("tool 1 meta");
    let mut lines = vec![
        meta.usage.to_owned(),
        meta.help.to_owned(),
        String::from("Options:"),
        String::from("  -d|--devices     display devices"),
        String::from("  -i|--ip          display ip addresses"),
        String::from("  -a|--arpcache    display arp cache and neighbors"),
        String::from("  -r|--routes      display routes"),
    ];
    if include_advanced {
        lines.push(String::from("(no advanced options)"));
    }
    lines.push(format!("Example: {}", meta.example));
    lines.join("\n")
}

fn render_sections(
    configuration: &impl LocalConfiguration,
    flags: DisplayFlags,
) -> Result<String, NetConfError> {
    let mut chunks = Vec::new();
    if flags.devices {
        let devices = configuration
            .list_devices()
            .map_err(|error| NetConfError::Conf(error.to_string()))?;
        chunks.push(format_devices(&devices));
    }
    if flags.ip {
        let ips = configuration
            .list_ip_addresses()
            .map_err(|error| NetConfError::Conf(error.to_string()))?;
        chunks.push(format_ips(&ips));
    }
    if flags.arpcache {
        let arps = configuration
            .list_arp_entries()
            .map_err(|error| NetConfError::Conf(error.to_string()))?;
        chunks.push(format_arp(&arps));
    }
    if flags.routes {
        let routes = configuration
            .list_routes()
            .map_err(|error| NetConfError::Conf(error.to_string()))?;
        chunks.push(format_routes(&routes));
    }
    Ok(chunks.join("\n"))
}

fn format_devices(devices: &[Device]) -> String {
    let mut lines = vec![String::from("## devices")];
    for device in devices {
        let eth_or_hw = match device.ethernet_address {
            Some(address) => address.to_string(),
            None => device.hardware_type.to_string(),
        };
        lines.push(format!(
            "nu={} easy={} eth_hw={} mtu={} real_dev={}",
            device.number, device.easy_name, eth_or_hw, device.mtu, device.real_name
        ));
    }
    lines.join("\n")
}

fn format_ips(ips: &[IpOnDevice]) -> String {
    let mut lines = vec![String::from("## ip")];
    for entry in ips {
        let peer = entry
            .ppp_peer
            .map_or_else(|| String::from("-"), |address| address.to_string());
        lines.push(format!(
            "nu={} ip={} netmask={} ppp={} ppp_with={}",
            entry.device_number,
            entry.address,
            entry.netmask,
            u8::from(entry.is_ppp),
            peer
        ));
    }
    lines.join("\n")
}

fn format_arp(entries: &[ArpEntry]) -> String {
    let mut lines = vec![String::from("## arpcache")];
    for entry in entries {
        lines.push(format!(
            "nu={} eth={} ip={}",
            entry.device_number, entry.ethernet, entry.ip
        ));
    }
    lines.join("\n")
}

fn format_routes(routes: &[Route]) -> String {
    let mut lines = vec![String::from("## routes")];
    for route in routes {
        let source = match route.source {
            RouteSource::Local => String::from("local"),
            RouteSource::Address(address) => address.to_string(),
        };
        let gateway = route
            .gateway
            .map_or_else(|| String::from("-"), |address| address.to_string());
        lines.push(format!(
            "nu={} destination={} netmask={} source={} gateway={} metric={}",
            route.device_number, route.destination, route.netmask, source, gateway, route.metric
        ));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{DisplayFlags, run_net_conf_with};
    use nz_net::FakeLocalConfiguration;

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|part| (*part).to_owned()).collect()
    }

    /// spec `t001_help_parses`
    #[test]
    fn t001_help_parses() {
        let output =
            run_net_conf_with(&args(&["--help"]), &FakeLocalConfiguration::sample()).expect("help");
        assert!(output.contains("devices"));
        assert!(output.contains("ip"));
        assert!(output.contains("arpcache"));
        assert!(output.contains("routes"));
    }

    /// spec `t001_no_flags_prints_all`
    #[test]
    fn t001_no_flags_prints_all() {
        let output = run_net_conf_with(&args(&[]), &FakeLocalConfiguration::sample()).expect("run");
        assert!(output.contains("## devices"));
        assert!(output.contains("## ip"));
        assert!(output.contains("## arpcache"));
        assert!(output.contains("## routes"));
        assert!(output.contains("easy=Eth0"));
        assert!(output.contains("netmask="));
        assert!(output.contains("gateway="));
    }

    /// spec `t001_devices_only`
    #[test]
    fn t001_devices_only() {
        let output =
            run_net_conf_with(&args(&["-d"]), &FakeLocalConfiguration::sample()).expect("run");
        assert!(output.contains("## devices"));
        assert!(!output.contains("## routes"));
        assert!(!output.contains("## ip"));
        assert!(!output.contains("## arpcache"));
    }

    /// spec `t001_all_closed_prints_all`
    #[test]
    fn t001_all_closed_prints_all() {
        let output = run_net_conf_with(
            &args(&["+d", "+i", "+a", "+r"]),
            &FakeLocalConfiguration::sample(),
        )
        .expect("run");
        assert!(output.contains("## devices"));
        assert!(output.contains("## ip"));
        assert!(output.contains("## arpcache"));
        assert!(output.contains("## routes"));
    }

    #[test]
    fn effective_flags_any_open_keeps_subset() {
        let flags = DisplayFlags {
            devices: true,
            ip: false,
            arpcache: false,
            routes: false,
        }
        .effective();
        assert!(flags.devices);
        assert!(!flags.ip);
    }
}
