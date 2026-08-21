//! 工具 0（GUI 契约）：切片 1 + 切片 2（`--toolhelp` / `--formupdate`）。

mod command_file;
mod error_text;
mod surface;

use crate::registry::{
    ToolEntry, backspace_tool_ids, lookup_by_id, stdin_tool_ids, tools_for_search,
};
use crate::tool_schemas::{schema_for_tool, text_meta_for_tool, tool0_schema};
use nz_arg::{ParseMode, ParseOutcome, ParsedArgs, parse};
use std::path::Path;
use surface::{
    ErrorSurface, FormUpdateSurface, ToolHelpSurface, ToolsSurface, VersionSurface,
    form_fields_from_schema, form_update_items, format_error, format_formupdate, format_toolhelp,
    format_tools, format_version,
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
    /// 未知工具号（toolhelp / formupdate）。
    UnknownTool(u32),
    /// 命令文件问题。
    CommandFile(String),
    /// 目标工具尚无 ArgSchema（formupdate 需要）。
    MissingSchema(u32),
}

impl std::fmt::Display for Tool0Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(message) | Self::CommandFile(message) => write!(formatter, "{message}"),
            Self::Help { .. } => write!(formatter, "help requested"),
            Self::NotImplementedYet(name) => {
                write!(formatter, "tool 0 switch '{name}' is not implemented yet")
            }
            Self::UnknownTool(id) => write!(formatter, "unknown tool id {id}"),
            Self::MissingSchema(id) => {
                write!(formatter, "tool {id} has no argument schema for formupdate")
            }
        }
    }
}

impl std::error::Error for Tool0Error {}

/// 工具 0 信息面集合。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Tool0Output {
    /// `--tools`。
    pub tools: Option<ToolsSurface>,
    /// `--toolhelp`。
    pub toolhelp: Option<ToolHelpSurface>,
    /// `--formupdate`。
    pub formupdate: Option<FormUpdateSurface>,
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
        if let Some(toolhelp) = &self.toolhelp {
            chunks.push(format_toolhelp(toolhelp));
        }
        if let Some(formupdate) = &self.formupdate {
            chunks.push(format_formupdate(formupdate));
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

/// 解析并执行工具 0。
///
/// # Errors
///
/// 解析失败、请求 help、未知工具、命令文件失败、或打开未实现开关。
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
    // 顺序：tools → toolhelp → formupdate → … → error → … → version
    if values.get_bool('t') == Some(true) {
        output.tools = Some(collect_tools_surface());
    }
    if values.get_bool('h') == Some(true) {
        let tool_id = values.get_u32('u').unwrap_or(0);
        output.toolhelp = Some(collect_toolhelp(tool_id)?);
    }
    if values.get_bool('f') == Some(true) {
        let tool_id = values.get_u32('u').unwrap_or(0);
        let path = values.get_string('b').unwrap_or("");
        if path.is_empty() {
            return Err(Tool0Error::CommandFile(String::from(
                "formupdate requires --buf path",
            )));
        }
        output.formupdate = Some(collect_formupdate(tool_id, Path::new(path))?);
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
    const UNIMPLEMENTED: &[(char, &str)] =
        &[('r', "run"), ('R', "run-key"), ('k', "kill"), ('c', "conf")];
    for &(key, name) in UNIMPLEMENTED {
        if values.isset(key) && values.get_bool(key) == Some(true) {
            return Err(Tool0Error::NotImplementedYet(name));
        }
    }
    Ok(())
}

fn collect_toolhelp(tool_id: u32) -> Result<ToolHelpSurface, Tool0Error> {
    let entry = lookup_by_id(tool_id).ok_or(Tool0Error::UnknownTool(tool_id))?;
    let meta = text_meta_for_tool(tool_id);
    let (has_schema, form, form_advanced) = match schema_for_tool(tool_id) {
        Some(schema) => {
            let (normal, advanced) = form_fields_from_schema(&schema);
            (true, normal, advanced)
        }
        None => (false, Vec::new(), Vec::new()),
    };
    Ok(ToolHelpSurface {
        tool_id,
        title: entry.title.to_owned(),
        help: meta.map_or_else(|| entry.title.to_owned(), |m| m.help.to_owned()),
        example: meta.map(|m| m.example.to_owned()).unwrap_or_default(),
        usage: meta.map_or_else(
            || format!("nz {tool_id}|{} [options]", entry.suggested_name),
            |m| m.usage.to_owned(),
        ),
        has_schema,
        form,
        form_advanced,
    })
}

fn collect_formupdate(tool_id: u32, path: &Path) -> Result<FormUpdateSurface, Tool0Error> {
    let _entry = lookup_by_id(tool_id).ok_or(Tool0Error::UnknownTool(tool_id))?;
    let schema = schema_for_tool(tool_id).ok_or(Tool0Error::MissingSchema(tool_id))?;

    let tokens = command_file::read_command_file_then_delete(path)
        .map_err(|error| Tool0Error::CommandFile(error.to_string()))?;

    // 跳过首 token 工具号（与 argv[0] 一样）
    let rest = if tokens
        .first()
        .is_some_and(|token| token.parse::<u32>().is_ok())
    {
        &tokens[1..]
    } else {
        tokens.as_slice()
    };

    let parsed = match parse(&schema, rest, ParseMode::FormUpdate) {
        Ok(ParseOutcome::Parsed(values)) => values,
        Ok(ParseOutcome::Help { .. }) => {
            return Err(Tool0Error::CommandFile(String::from(
                "formupdate must not request help",
            )));
        }
        Err(error) => {
            return Err(Tool0Error::CommandFile(format!(
                "formupdate parse failed: {error}"
            )));
        }
    };

    Ok(FormUpdateSurface {
        tool_id,
        items: form_update_items(&schema, &parsed),
    })
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
    use std::io::Write;

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|part| (*part).to_owned()).collect()
    }

    fn temp_cmd_file(contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "nz-tool0-{}-{}.cmd",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        let mut file = std::fs::File::create(&path).expect("create");
        write!(file, "{contents}").expect("write");
        path
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
        let error = run_tool0(&args(&["-r", "-b", "x"])).expect_err("run");
        assert!(matches!(error, Tool0Error::NotImplementedYet("run")));
    }

    /// spec `tool0_toolhelp_form_has_advanced_split`
    #[test]
    fn tool0_toolhelp_form_has_advanced_split() {
        let output = run_tool0(&args(&["-h", "-u", "1"])).expect("help");
        let help = output.toolhelp.as_ref().expect("surface");
        assert!(help.has_schema);
        assert!(
            help.form
                .iter()
                .any(|field| field.key == 'd' && !field.advanced)
        );
        assert!(
            help.form_advanced
                .iter()
                .any(|field| field.key == 'a' && field.advanced)
        );
        assert!(!output.looks_like_tcl());
    }

    /// spec `tool0_formupdate_deletes_file` + `tool0_formupdate_skips_toolnum_token`
    #[test]
    fn tool0_formupdate_deletes_file_and_skips_toolnum() {
        let path = temp_cmd_file("1 -d eth0\n");
        let path_str = path.to_str().expect("utf8").to_owned();
        let output = run_tool0(&args(&["-f", "-u", "1", "-b", &path_str])).expect("form");
        assert!(!path.exists(), "command file must be deleted");
        let update = output.formupdate.expect("surface");
        assert_eq!(update.tool_id, 1);
        assert_eq!(update.items.len(), 1);
        assert_eq!(update.items[0].key, 'd');
        assert_eq!(update.items[0].value, "eth0");
    }
}
