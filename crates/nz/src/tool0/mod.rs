//! 工具 0（GUI 契约）切片 1：`--tools` / `--version` / `--error`。

mod error_text;
mod surface;

use crate::registry::{ToolEntry, backspace_tool_ids, stdin_tool_ids, tools_for_search};
use crate::tool_schemas::tool0_schema;
use nz_arg::{ParseMode, ParseOutcome, ParsedArgs, parse};
use surface::{
    ErrorSurface, ToolsSurface, VersionSurface, format_error, format_tools, format_version,
};

pub use surface::{ToolListEntry, ToolsSurface as Tool0ToolsSurface};

/// 工具 0 运行错误。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tool0Error {
    /// 参数解析失败。
    Parse(String),
    /// 用户要了帮助（调用方打印后退出 0）。
    Help {
        /// 是否含 Advanced。
        include_advanced: bool,
    },
    /// 切片尚未实现的开关。
    NotImplementedYet(&'static str),
}

impl std::fmt::Display for Tool0Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(message) => write!(formatter, "{message}"),
            Self::Help { .. } => write!(formatter, "help requested"),
            Self::NotImplementedYet(name) => {
                write!(formatter, "tool 0 switch '{name}' is not implemented yet")
            }
        }
    }
}

impl std::error::Error for Tool0Error {}

/// 切片 1 可产生的信息面集合。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Tool0Output {
    /// `--tools`。
    pub tools: Option<ToolsSurface>,
    /// `--error`。
    pub error: Option<ErrorSurface>,
    /// `--version`。
    pub version: Option<VersionSurface>,
}

impl Tool0Output {
    /// 渲染为稳定行协议（可空）。
    #[must_use]
    pub fn render(&self) -> String {
        let mut chunks = Vec::new();
        if let Some(tools) = &self.tools {
            chunks.push(format_tools(tools));
        }
        if let Some(error) = &self.error {
            chunks.push(format_error(error));
        }
        if let Some(version) = &self.version {
            chunks.push(format_version(version));
        }
        chunks.join("\n")
    }

    /// 是否含 Tcl 风格痕迹（验收用）。
    #[must_use]
    pub fn looks_like_tcl(&self) -> bool {
        let text = self.render();
        text.contains("lappend ") || text.contains("set ") || text.contains("Tcl")
    }
}

/// 解析并执行工具 0（切片 1）。
///
/// # Errors
///
/// 解析失败、请求 help、或打开未实现开关。
pub fn run_tool0(tool_arguments: &[String]) -> Result<Tool0Output, Tool0Error> {
    match parse(&tool0_schema(), tool_arguments, ParseMode::Cli) {
        Ok(ParseOutcome::Help { include_advanced }) => Err(Tool0Error::Help { include_advanced }),
        Ok(ParseOutcome::Parsed(values)) => build_output(&values),
        Err(error) => Err(Tool0Error::Parse(error.to_string())),
    }
}

fn build_output(values: &ParsedArgs) -> Result<Tool0Output, Tool0Error> {
    reject_unimplemented(values)?;

    let mut output = Tool0Output::default();
    // 顺序：tools → … → error → … → version
    if values.get_bool('t') == Some(true) {
        output.tools = Some(collect_tools_surface());
    }
    if values.get_bool('e') == Some(true) {
        let code = values.get_u32('u').unwrap_or(0);
        output.error = Some(ErrorSurface {
            code,
            text: error_text::describe(code),
        });
    }
    if values.get_bool('v') == Some(true) {
        output.version = Some(package_version());
    }
    Ok(output)
}

fn reject_unimplemented(values: &ParsedArgs) -> Result<(), Tool0Error> {
    const UNIMPLEMENTED: &[(char, &str)] = &[
        ('h', "toolhelp"),
        ('f', "formupdate"),
        ('r', "run"),
        ('R', "run-key"),
        ('k', "kill"),
        ('c', "conf"),
    ];
    for &(key, name) in UNIMPLEMENTED {
        if values.isset(key) && values.get_bool(key) == Some(true) {
            return Err(Tool0Error::NotImplementedYet(name));
        }
    }
    Ok(())
}

fn collect_tools_surface() -> ToolsSurface {
    let release: Vec<ToolEntry> = tools_for_search().collect();
    let max_id = release.iter().map(|entry| entry.id.0).max().unwrap_or(0);
    let tools: Vec<ToolListEntry> = release
        .iter()
        .map(|entry| ToolListEntry {
            id: entry.id.0,
            title: entry.title.to_owned(),
            suggested_name: entry.suggested_name.to_owned(),
        })
        .collect();

    let mut synonyms = Vec::new();
    for entry in &release {
        for token in synonym_tokens(entry.title) {
            synonyms.push((entry.id.0, token));
        }
    }

    let tree_children: Vec<u32> = release.iter().map(|entry| entry.id.0).collect();

    ToolsSurface {
        max_id,
        count: u32::try_from(tools.len()).unwrap_or(u32::MAX),
        tools,
        synonyms,
        stdin: stdin_tool_ids(),
        backspace: backspace_tool_ids(),
        tree_root: String::from("main"),
        tree_children,
    }
}

fn synonym_tokens(title: &str) -> Vec<String> {
    title
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(str::to_ascii_lowercase)
        .collect()
}

fn package_version() -> VersionSurface {
    let raw = env!("CARGO_PKG_VERSION");
    let mut parts = raw.split('.');
    let major = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let micro = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    VersionSurface {
        major,
        minor,
        micro,
    }
}

/// CLI 入口：打印信息面或帮助；返回进程码。
#[must_use]
pub fn invoke_tool0(tool_arguments: &[String]) -> i32 {
    match run_tool0(tool_arguments) {
        Ok(output) => {
            let text = output.render();
            if !text.is_empty() {
                println!("{text}");
            }
            0
        }
        Err(Tool0Error::Help { include_advanced }) => {
            if include_advanced {
                println!("nz tool 0 help (advanced)");
            } else {
                println!("nz tool 0 help");
            }
            0
        }
        Err(error) => {
            eprintln!("nz: {error}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Tool0Error, run_tool0};
    use crate::registry::{DEFERRED_TOOL_IDS, backspace_tool_ids, stdin_tool_ids};

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|part| (*part).to_owned()).collect()
    }

    /// spec `tool0_tools_lists_and_marks`
    #[test]
    fn tool0_tools_lists_and_marks() {
        let output = run_tool0(&args(&["--tools"])).expect("tools");
        let tools = output.tools.expect("surface");
        assert!(tools.tools.iter().any(|tool| tool.id == 1));
        assert!(
            !tools
                .tools
                .iter()
                .any(|tool| tool.id == 0 || tool.id == 218)
        );
        for id in DEFERRED_TOOL_IDS {
            assert!(
                !tools.tools.iter().any(|tool| tool.id == *id),
                "deferred {id}"
            );
        }
        assert_eq!(tools.stdin, stdin_tool_ids());
        assert_eq!(tools.backspace, backspace_tool_ids());
        assert_eq!(tools.stdin, vec![7, 14, 87, 88, 89, 90, 99, 152, 171]);
        assert_eq!(tools.backspace, vec![138, 139, 210]);
        assert_eq!(tools.tree_root, "main");
        assert!(tools.tree_children.contains(&1));
    }

    /// spec `tool0_version_triple`
    #[test]
    fn tool0_version_triple() {
        let output = run_tool0(&args(&["-v"])).expect("version");
        let version = output.version.expect("surface");
        assert_eq!(
            (version.major, version.minor, version.micro),
            (0, 1, 0),
            "workspace package version"
        );
    }

    /// spec `tool0_error_text_nonempty`
    #[test]
    fn tool0_error_text_nonempty() {
        let ok = run_tool0(&args(&["-e", "-u", "0"])).expect("e0");
        assert!(!ok.error.expect("s").text.is_empty());
        let other = run_tool0(&args(&["--error", "--uint", "2001"])).expect("e2001");
        assert!(!other.error.expect("s").text.is_empty());
    }

    /// spec `tool0_no_tcl_required`
    #[test]
    fn tool0_no_tcl_required() {
        let output = run_tool0(&args(&["-t", "-v", "-e", "-u", "0"])).expect("combo");
        assert!(!output.looks_like_tcl());
        let rendered = output.render();
        assert!(rendered.contains("section:tools"));
        assert!(rendered.contains("section:version"));
        assert!(rendered.contains("section:error"));
        assert!(!rendered.contains("lappend "));
    }

    /// spec `tool0_unimplemented_switch_errors`
    #[test]
    fn tool0_unimplemented_switch_errors() {
        let error = run_tool0(&args(&["-h", "-u", "1"])).expect_err("toolhelp");
        assert!(matches!(error, Tool0Error::NotImplementedYet("toolhelp")));
    }
}
