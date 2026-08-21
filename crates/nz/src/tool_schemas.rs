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

/// 工具 1 参数桩（业务未实现；供 formupdate / toolhelp 验收）。
///
/// 含普通 `-d/--device` 与 Advanced `-a/--advnote`。
///
/// # Panics
///
/// 仅当静态描述表非法时 panic。
#[must_use]
pub fn tool1_stub_schema() -> ArgSchema {
    ArgSchema::try_from_specs(vec![
        ArgSpec::optional_string('d', "device", "device to use", None::<String>),
        ArgSpec::optional_string('a', "advnote", "advanced note", None::<String>).advanced(),
    ])
    .expect("tool1 stub schema is static and valid")
}

/// 按工具号取参数表；未挂载则 `None`。
#[must_use]
pub fn schema_for_tool(tool_id: u32) -> Option<ArgSchema> {
    match tool_id {
        0 => Some(tool0_schema()),
        1 => Some(tool1_stub_schema()),
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
            help: "Display network configuration (stub schema for GUI form tests).",
            example: "nz 1 -d eth0",
            usage: "nz 1|-|net-conf [options]",
        }),
        _ => None,
    }
}
