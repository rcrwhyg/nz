//! 工具级参数 schema。

use nz_arg::{ArgSchema, ArgSpec};

/// 工具 0 完整开关表（对照 `doc/netwox/tool0.md`）。
///
/// 切片 1 只实现 `--tools` / `--version` / `--error`；其余开关可解析但执行时报未实现。
///
/// # Panics
///
/// 仅当静态描述表非法时 panic（正常构建不会发生）。
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
