//! 工具 2：打印排障用调试信息。
//!
//! 对照 `spec/netwox/info/002.md` 与 `tools/000002.c`：
//! 版本三元组、平台相关定义、conf 如何取得，并以 `END` 收尾。

use crate::tool_schemas::{text_meta_for_tool, tool2_schema};
use crate::tools::net_conf::run_net_conf_with;
use nz_arg::{ParseMode, ParseOutcome, parse};
use nz_net::{FakeLocalConfiguration, SystemLocalConfiguration};

/// 工具 2 失败。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DebugInfoError {
    /// 参数解析失败。
    Parse(String),
}

impl std::fmt::Display for DebugInfoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(message) => write!(formatter, "{message}"),
        }
    }
}

impl std::error::Error for DebugInfoError {}

/// 运行工具 2（无业务参数；帮助文本也作为 `Ok` 返回）。
///
/// # Errors
///
/// 解析失败（例如多余位置参数）。
pub fn run_debug_info(tool_arguments: &[String]) -> Result<String, DebugInfoError> {
    let schema = tool2_schema();
    match parse(&schema, tool_arguments, ParseMode::Cli) {
        Ok(ParseOutcome::Help { include_advanced }) => Ok(format_help(include_advanced)),
        Ok(ParseOutcome::Parsed(_)) => Ok(render_debug_report()),
        Err(error) => Err(DebugInfoError::Parse(error.to_string())),
    }
}

fn format_help(include_advanced: bool) -> String {
    let meta = text_meta_for_tool(2).expect("tool 2 meta");
    let mut lines = vec![
        meta.usage.to_owned(),
        meta.help.to_owned(),
        String::from("Options: (none)"),
    ];
    if include_advanced {
        lines.push(String::from("(no advanced options)"));
    }
    lines.push(format!("Example: {}", meta.example));
    lines.join("\n")
}

fn render_debug_report() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let mut lines = vec![
        format!("nz toolbox version {version}."),
        format!("nz-net library version {version}."),
        String::new(),
        String::from("####****####****####****####****####"),
        String::from("## platform"),
        format!("os={}", std::env::consts::OS),
        format!("arch={}", std::env::consts::ARCH),
        format!("family={}", std::env::consts::FAMILY),
        format!("pointer_width={}", std::mem::size_of::<usize>() * 8),
        String::new(),
        String::from("####****####****####****####****####"),
        String::from("## conf"),
    ];

    let conf_dump = match SystemLocalConfiguration::query() {
        Ok(configuration) => {
            lines.push(String::from("conf_source=system-inventory"));
            lines.push(String::from("####****####****####****####****####"));
            lines.push(String::from("## conf_debug"));
            lines.push(String::from(
                "Network configuration is read from the live host via if-addrs.",
            ));
            lines.push(String::from(
                "On Linux, ARP and routes also come from /proc/net/arp and /proc/net/route.",
            ));
            run_net_conf_with(&[], &configuration).unwrap_or_default()
        }
        Err(error) => {
            lines.push(String::from("conf_source=fake-sample"));
            lines.push(format!("system_inventory_error={error}"));
            lines.push(String::from("####****####****####****####****####"));
            lines.push(String::from("## conf_debug"));
            lines.push(String::from(
                "Live inventory failed; using FakeLocalConfiguration::sample().",
            ));
            run_net_conf_with(&[], &FakeLocalConfiguration::sample()).unwrap_or_default()
        }
    };
    if !conf_dump.is_empty() {
        lines.push(conf_dump);
    }

    lines.push(String::new());
    lines.push(String::from("####****####****####****####****####"));
    lines.push(String::from("END"));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::run_debug_info;

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|part| (*part).to_owned()).collect()
    }

    /// spec `t002_help_parses`
    #[test]
    fn t002_help_parses() {
        let output = run_debug_info(&args(&["--help"])).expect("help");
        assert!(output.contains("(none)"));
        assert!(
            output.contains("debugging") || output.contains("Debug") || output.contains("debug")
        );
    }

    /// spec `t002_prints_version_and_end`
    #[test]
    fn t002_prints_version_and_end() {
        let output = run_debug_info(&args(&[])).expect("run");
        assert!(output.contains(env!("CARGO_PKG_VERSION")));
        assert!(output.trim_end().ends_with("END"));
    }

    /// spec `t002_prints_conf_debug_section`
    #[test]
    fn t002_prints_conf_debug_section() {
        let output = run_debug_info(&args(&[])).expect("run");
        assert!(output.contains("## conf_debug"));
        assert!(
            output.contains("conf_source=system-inventory")
                || output.contains("conf_source=fake-sample")
        );
    }
}
