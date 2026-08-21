//! 工具级参数 schema（起步：工具 0 子集，供注册表验收与后续工具 0）。

use nz_arg::{ArgSchema, ArgSpec};

/// 工具 0 参数表（对照 `doc/netwox/tool0.md` 的布尔/字符串子集；完整契约另闸）。
///
/// 当前含 `-t/--tools` 等，足够 `bool_triple` / `help2` 验收；未列全的开关后续补齐。
///
/// # Panics
///
/// 仅当静态描述表非法时 panic（编译期常量错误，正常构建不会发生）。
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
