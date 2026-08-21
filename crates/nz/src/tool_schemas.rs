//! 工具参数 schema 与 help 元数据（按工具号按需挂载）。

use nz_arg::{ArgSchema, ArgSpec};

/// 某工具的静态说明（toolhelp 用）。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolTextMeta {
    /// 帮助正文。
    pub help: &'static str,
    /// 示例命令行。
    pub example: &'static str,
    /// Usage 行。
    pub usage: &'static str,
}

/// 工具 0 完整开关表（对照 `doc/netwox/tool0.md`）。
///
/// # Panics
///
/// 仅当静态描述表非法时 panic。
#[must_use]
pub fn tool0_schema() -> ArgSchema {
    ArgSchema::try_from_specs(vec![
        ArgSpec::optional_bool('t', "tools", "list tools for GUI"),
        ArgSpec::optional_bool('h', "toolhelp", "tool help for GUI"),
        ArgSpec::optional_bool('f', "formupdate", "form update"),
        ArgSpec::optional_bool('r', "run", "run from file"),
        ArgSpec::optional_bool('R', "run-key", "run with key handling"),
        ArgSpec::optional_bool('k', "kill", "kill tool"),
        ArgSpec::optional_bool('e', "error", "format error"),
        ArgSpec::optional_bool('c', "conf", "export conf"),
        ArgSpec::optional_bool('v', "version", "version"),
        ArgSpec::optional_string('b', "buf", "buffer or path", None::<String>),
        ArgSpec::optional_u32('u', "uint", "tool id or error code", None),
    ])
    .expect("tool0 schema is static and valid")
}

/// 工具 1 参数表（对照 `000001.c` / `spec/netwox/info/001.md`）。
///
/// # Panics
///
/// 仅当静态描述表非法时 panic。
#[must_use]
pub fn tool1_schema() -> ArgSchema {
    ArgSchema::try_from_specs(vec![
        ArgSpec::optional_bool('d', "devices", "display devices"),
        ArgSpec::optional_bool('i', "ip", "display ip addresses"),
        ArgSpec::optional_bool('a', "arpcache", "display arp cache and neighbors"),
        ArgSpec::optional_bool('r', "routes", "display routes"),
    ])
    .expect("tool1 schema is static and valid")
}

/// 工具 2 参数表（无参数；对照 `000002.c`）。
///
/// # Panics
///
/// 仅当静态描述表非法时 panic。
#[must_use]
pub fn tool2_schema() -> ArgSchema {
    ArgSchema::try_from_specs(Vec::new()).expect("tool2 schema is static and valid")
}

/// 工具 3 参数表（对照 `000003.c` / `spec/netwox/info/003.md`）。
///
/// # Panics
///
/// 仅当静态描述表非法时 panic。
#[must_use]
pub fn tool3_schema() -> ArgSchema {
    ArgSchema::try_from_specs(vec![
        ArgSpec::optional_bool('t', "title", "display titles").advanced(),
        ArgSpec::optional_bool('i', "ip", "obtain IP address").advanced(),
        ArgSpec::optional_bool('h', "host", "obtain hostname").advanced(),
        ArgSpec::optional_bool('H', "hosts", "obtain hostnames").advanced(),
        ArgSpec::optional_bool('e', "eth", "obtain Ethernet address").advanced(),
        ArgSpec::optional_bool('a', "all", "display all IP addresses").advanced(),
        ArgSpec::required_string('q', "query", "IP address or hostname"),
    ])
    .expect("tool3 schema is static and valid")
}

/// 按工具号取参数表；未挂载则 `None`。
#[must_use]
pub fn schema_for_tool(tool_id: u32) -> Option<ArgSchema> {
    match tool_id {
        0 => Some(tool0_schema()),
        1 => Some(tool1_schema()),
        2 => Some(tool2_schema()),
        3 => Some(tool3_schema()),
        _ => None,
    }
}

/// 按工具号取文案；无专用文案时由调用方用登记标题兜底。
#[must_use]
pub fn text_meta_for_tool(tool_id: u32) -> Option<ToolTextMeta> {
    match tool_id {
        0 => Some(ToolTextMeta {
            help: "Obtain information needed by the GUI (Search/Form/Run).",
            example: "nz 0 --tools",
            usage: "nz 0|-|gui-info [options]",
        }),
        1 => Some(ToolTextMeta {
            help: "Display network configuration (devices, ip, arpcache, routes).",
            example: "nz 1 -d",
            usage: "nz 1|-|net-conf [-d|+d] [-i|+i] [-a|+a] [-r|+r]",
        }),
        2 => Some(ToolTextMeta {
            help: "Display debugging information (versions, platform, how conf is retrieved). No parameters.",
            example: "nz 2",
            usage: "nz 2|-|debug-info",
        }),
        3 => Some(ToolTextMeta {
            help: "Display information about an IP address or a hostname (ip/host/hosts/eth).",
            example: "nz 3 -q 127.0.0.1",
            usage: "nz 3|-|host-info -q hostname [advanced options]",
        }),
        _ => None,
    }
}
