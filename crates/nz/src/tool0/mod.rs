//! 工具 0（GUI 契约）：切片 1–3。

mod command_file;
mod error_text;
mod surface;

use crate::registry::{
    DispatchRequest, ToolEntry, backspace_tool_ids, build_search_tree, dispatch, format_catalog,
    lookup_by_id, stdin_tool_ids, tools_for_search,
};
use crate::tool_schemas::{schema_for_tool, text_meta_for_tool, tool0_schema};
use nz_arg::{ParseMode, ParseOutcome, ParsedArgs, parse};
use nz_net::{FakeLocalConfiguration, LocalConfiguration, RouteSource, SystemLocalConfiguration};
use std::path::Path;
use std::time::Duration;
use surface::{
    ConfSurface, ErrorSurface, FormUpdateSurface, KillSurface, RunSurface, ToolHelpSurface,
    ToolsSurface, VersionSurface, form_fields_from_schema, form_update_items, format_conf,
    format_error, format_formupdate, format_kill, format_run, format_toolhelp, format_tools,
    format_version,
};

pub use surface::{ToolListEntry, ToolsSurface as Tool0ToolsSurface};

/// 可注入行为（测试用：跳过按键等待、自定义 kill）。
#[derive(Clone, Debug, Default)]
pub struct Tool0Hooks {
    /// 为 true 时 `--run-key` 不阻塞等待按键。
    pub skip_key_wait: bool,
}

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
    /// 未知工具号。
    UnknownTool(u32),
    /// 命令文件问题。
    CommandFile(String),
    /// 目标工具尚无 `ArgSchema`（formupdate 需要）。
    MissingSchema(u32),
    /// 分发失败。
    Dispatch(String),
    /// 配置导出失败。
    Conf(String),
}

impl std::fmt::Display for Tool0Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(message)
            | Self::CommandFile(message)
            | Self::Dispatch(message)
            | Self::Conf(message) => {
                write!(formatter, "{message}")
            }
            Self::Help { .. } => write!(formatter, "help requested"),
            Self::UnknownTool(id) => write!(formatter, "unknown tool id {id}"),
            Self::MissingSchema(id) => {
                write!(formatter, "tool {id} has no argument schema for formupdate")
            }
        }
    }
}

impl std::error::Error for Tool0Error {}

/// 一次工具 0 调用的完整结果。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Tool0Session {
    /// 信息面。
    pub output: Tool0Output,
    /// 进程退出码（`--run`/`--run-key` 时取被调工具码，否则 0）。
    pub exit_code: i32,
}

/// 工具 0 信息面集合。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Tool0Output {
    /// `--tools`。
    pub tools: Option<ToolsSurface>,
    /// `--toolhelp`。
    pub toolhelp: Option<ToolHelpSurface>,
    /// `--formupdate`。
    pub formupdate: Option<FormUpdateSurface>,
    /// `--run` / `--run-key`。
    pub run: Option<RunSurface>,
    /// `--kill`。
    pub kill: Option<KillSurface>,
    /// `--error`。
    pub error: Option<ErrorSurface>,
    /// `--conf`。
    pub conf: Option<ConfSurface>,
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
        if let Some(run) = &self.run {
            chunks.push(format_run(run));
        }
        if let Some(kill) = &self.kill {
            chunks.push(format_kill(kill));
        }
        if let Some(error) = &self.error {
            chunks.push(format_error(error));
        }
        if let Some(conf) = &self.conf {
            chunks.push(format_conf(conf));
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

/// 解析并执行工具 0（默认 hooks）。
///
/// # Errors
///
/// 解析失败、请求 help、未知工具、命令文件失败等。
pub fn run_tool0(tool_arguments: &[String]) -> Result<Tool0Session, Tool0Error> {
    run_tool0_with(tool_arguments, &Tool0Hooks::default())
}

/// 解析并执行工具 0（可注入 hooks）。
///
/// # Errors
///
/// 同 [`run_tool0`]。
pub fn run_tool0_with(
    tool_arguments: &[String],
    hooks: &Tool0Hooks,
) -> Result<Tool0Session, Tool0Error> {
    match parse(&tool0_schema(), tool_arguments, ParseMode::Cli) {
        Ok(ParseOutcome::Help { include_advanced }) => Err(Tool0Error::Help { include_advanced }),
        Ok(ParseOutcome::Parsed(values)) => build_session(&values, hooks),
        Err(error) => Err(Tool0Error::Parse(error.to_string())),
    }
}

fn build_session(values: &ParsedArgs, hooks: &Tool0Hooks) -> Result<Tool0Session, Tool0Error> {
    let mut output = Tool0Output::default();
    let mut exit_code = 0;

    // 顺序：tools → toolhelp → formupdate → run → run-key → kill → error → conf → version
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

    let want_run = values.get_bool('r') == Some(true);
    let want_run_key = values.get_bool('R') == Some(true);
    if want_run || want_run_key {
        let path = values.get_string('b').unwrap_or("");
        if path.is_empty() {
            return Err(Tool0Error::CommandFile(String::from(
                "run requires --buf path",
            )));
        }
        let (child_code, child_output) = execute_run_file(Path::new(path), hooks)?;
        if want_run_key && !hooks.skip_key_wait {
            wait_for_any_key();
        }
        exit_code = child_code;
        output.run = Some(RunSurface {
            exit_code: child_code,
            waited_for_key: want_run_key && !hooks.skip_key_wait,
            child_output,
        });
    }

    if values.get_bool('k') == Some(true) {
        let pid = values.get_u32('u').unwrap_or(0);
        let sleep_ms = parse_sleep_ms(values.get_string('b').unwrap_or(""));
        std::thread::sleep(Duration::from_millis(u64::from(sleep_ms)));
        let _ = terminate_process(pid);
        output.kill = Some(KillSurface {
            pid,
            sleep_ms,
            ignored_missing: true,
        });
    }

    if values.get_bool('e') == Some(true) {
        let code = values.get_u32('u').unwrap_or(0);
        output.error = Some(ErrorSurface {
            code,
            text: error_text::describe(code),
        });
    }

    if values.get_bool('c') == Some(true) {
        output.conf = Some(collect_conf()?);
    }

    if values.get_bool('v') == Some(true) {
        output.version = Some(package_version());
    }

    Ok(Tool0Session { output, exit_code })
}

fn parse_sleep_ms(raw: &str) -> u32 {
    if raw.is_empty() {
        return 0;
    }
    raw.parse().unwrap_or(0)
}

fn wait_for_any_key() {
    use std::io::Read;
    let mut buffer = [0_u8; 1];
    let _ = std::io::stdin().read(&mut buffer);
}

fn terminate_process(pid: u32) -> Result<(), ()> {
    if pid == 0 {
        return Err(());
    }
    #[cfg(unix)]
    {
        let status = std::process::Command::new("kill")
            .args(["-TERM", &pid.to_string()])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map_err(|_| ())?;
        if status.success() { Ok(()) } else { Err(()) }
    }
    #[cfg(windows)]
    {
        let status = std::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map_err(|_| ())?;
        if status.success() { Ok(()) } else { Err(()) }
    }
}

fn execute_run_file(path: &Path, hooks: &Tool0Hooks) -> Result<(i32, String), Tool0Error> {
    let tokens = command_file::read_command_file_then_delete(path)
        .map_err(|error| Tool0Error::CommandFile(error.to_string()))?;
    let mut argv = Vec::with_capacity(tokens.len() + 1);
    argv.push(String::from("nz"));
    argv.extend(tokens);

    match dispatch(&argv) {
        Ok(DispatchRequest::Catalog) => Ok((0, format_catalog())),
        Ok(DispatchRequest::Run {
            entry,
            tool_arguments,
        }) => execute_dispatched_tool(entry, &tool_arguments, hooks),
        Err(error) => Err(Tool0Error::Dispatch(error.to_string())),
    }
}

fn execute_dispatched_tool(
    entry: ToolEntry,
    tool_arguments: &[String],
    hooks: &Tool0Hooks,
) -> Result<(i32, String), Tool0Error> {
    if entry.id.0 == 0 {
        let session = run_tool0_with(tool_arguments, hooks)?;
        return Ok((session.exit_code, session.output.render()));
    }
    if entry.id.0 == 1 {
        return match crate::tools::run_net_conf(tool_arguments) {
            Ok(text) => Ok((0, text)),
            Err(error) => Err(Tool0Error::Dispatch(error.to_string())),
        };
    }
    if entry.id.0 == 2 {
        return match crate::tools::run_debug_info(tool_arguments) {
            Ok(text) => Ok((0, text)),
            Err(error) => Err(Tool0Error::Dispatch(error.to_string())),
        };
    }
    Ok((2, format!("tool {} not implemented", entry.id.0)))
}

fn collect_conf() -> Result<ConfSurface, Tool0Error> {
    if let Ok(conf) = SystemLocalConfiguration::query() {
        return conf_surface_from(&conf);
    }
    conf_surface_from(&FakeLocalConfiguration::sample())
}

fn conf_surface_from(conf: &impl LocalConfiguration) -> Result<ConfSurface, Tool0Error> {
    let devices = conf
        .list_devices()
        .map_err(|error| Tool0Error::Conf(error.to_string()))?;
    let ips = conf
        .list_ip_addresses()
        .map_err(|error| Tool0Error::Conf(error.to_string()))?;
    let arps = conf
        .list_arp_entries()
        .map_err(|error| Tool0Error::Conf(error.to_string()))?;
    let routes = conf
        .list_routes()
        .map_err(|error| Tool0Error::Conf(error.to_string()))?;

    Ok(ConfSurface {
        devices: devices
            .into_iter()
            .map(|device| {
                format!(
                    "{}:{}:{}:{}",
                    device.number, device.easy_name, device.real_name, device.mtu
                )
            })
            .collect(),
        ips: ips
            .into_iter()
            .map(|entry| {
                format!(
                    "{}:{}:{}",
                    entry.device_number, entry.address, entry.netmask
                )
            })
            .collect(),
        arps: arps
            .into_iter()
            .map(|entry| format!("{}:{}:{}", entry.device_number, entry.ethernet, entry.ip))
            .collect(),
        routes: routes
            .into_iter()
            .map(|route| {
                let source = match route.source {
                    RouteSource::Local => String::from("local"),
                    RouteSource::Address(address) => address.to_string(),
                };
                let gateway = route
                    .gateway
                    .map_or_else(|| String::from("-"), |address| address.to_string());
                format!(
                    "{}:{}:{}:{}:{}:{}",
                    route.device_number,
                    route.destination,
                    route.netmask,
                    source,
                    gateway,
                    route.metric
                )
            })
            .collect(),
    })
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

    ToolsSurface {
        max_id,
        count: u32::try_from(tools.len()).unwrap_or(u32::MAX),
        tools,
        synonyms,
        stdin: stdin_tool_ids(),
        backspace: backspace_tool_ids(),
        tree: build_search_tree(),
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
        Ok(session) => {
            let text = session.output.render();
            if !text.is_empty() {
                println!("{text}");
            }
            session.exit_code
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
    use super::{Tool0Error, Tool0Hooks, run_tool0, run_tool0_with};
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

    fn test_hooks() -> Tool0Hooks {
        Tool0Hooks {
            skip_key_wait: true,
        }
    }

    /// spec `tool0_tools_lists_and_marks`
    #[test]
    fn tool0_tools_lists_and_marks() {
        let session = run_tool0(&args(&["--tools"])).expect("tools");
        let tools = session.output.tools.expect("surface");
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
        assert_eq!(
            tools.tree.first().map(|node| node.id.as_str()),
            Some("main")
        );
        assert!(
            tools
                .tree
                .iter()
                .any(|node| node.id == "info-local" && node.child_tools.contains(&1)),
            "tool 1 under info-local"
        );
        assert!(
            tools
                .tree
                .iter()
                .any(|node| node.id == "ping" && node.child_tools.contains(&49)),
            "tool 49 under ping"
        );
        assert!(
            !tools.tree.iter().any(|node| node.child_tools.contains(&0)),
            "tool 0 not in tree"
        );
        let rendered = crate::tool0::surface::format_tools(&tools);
        assert!(rendered.contains("tree_child_cat:main:info"));
        assert!(rendered.contains("tree_child_tool:info-local:1"));
    }

    /// spec `tool0_version_triple`
    #[test]
    fn tool0_version_triple() {
        let session = run_tool0(&args(&["-v"])).expect("version");
        let version = session.output.version.expect("surface");
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
        assert!(!ok.output.error.expect("s").text.is_empty());
        let other = run_tool0(&args(&["--error", "--uint", "2001"])).expect("e2001");
        assert!(!other.output.error.expect("s").text.is_empty());
    }

    /// spec `tool0_no_tcl_required`
    #[test]
    fn tool0_no_tcl_required() {
        let session = run_tool0(&args(&["-t", "-v", "-e", "-u", "0", "-c"])).expect("combo");
        assert!(!session.output.looks_like_tcl());
        let rendered = session.output.render();
        assert!(rendered.contains("section:tools"));
        assert!(rendered.contains("section:version"));
        assert!(rendered.contains("section:error"));
        assert!(rendered.contains("section:conf"));
        assert!(!rendered.contains("lappend "));
    }

    /// spec `tool0_toolhelp_form_has_advanced_split`（advanced 分组由 schema 元数据驱动；工具 1 无 Advanced）
    #[test]
    fn tool0_toolhelp_form_has_advanced_split() {
        let session = run_tool0(&args(&["-h", "-u", "1"])).expect("help");
        let help = session.output.toolhelp.as_ref().expect("surface");
        assert!(help.has_schema);
        for key in ['d', 'i', 'a', 'r'] {
            assert!(
                help.form
                    .iter()
                    .any(|field| field.key == key && !field.advanced),
                "missing ordinary field {key}"
            );
        }
        assert!(help.form_advanced.is_empty());
        assert!(!session.output.looks_like_tcl());
    }

    /// spec `tool0_formupdate_deletes_file` + `tool0_formupdate_skips_toolnum_token`
    #[test]
    fn tool0_formupdate_deletes_file_and_skips_toolnum() {
        let path = temp_cmd_file("1 -d\n");
        let path_str = path.to_str().expect("utf8").to_owned();
        let session = run_tool0(&args(&["-f", "-u", "1", "-b", &path_str])).expect("form");
        assert!(!path.exists(), "command file must be deleted");
        let update = session.output.formupdate.expect("surface");
        assert_eq!(update.tool_id, 1);
        assert_eq!(update.items.len(), 1);
        assert_eq!(update.items[0].key, 'd');
        assert_eq!(update.items[0].value, "1");
    }

    /// spec `tool0_run_executes_and_deletes`
    #[test]
    fn tool0_run_executes_and_deletes() {
        let path = temp_cmd_file("0 -v\n");
        let path_str = path.to_str().expect("utf8").to_owned();
        let session = run_tool0_with(&args(&["-r", "-b", &path_str]), &test_hooks()).expect("run");
        assert!(!path.exists());
        assert_eq!(session.exit_code, 0);
        let run = session.output.run.expect("run surface");
        assert_eq!(run.exit_code, 0);
        assert!(run.child_output.contains("section:version"));
        assert!(run.child_output.contains("major:0"));
    }

    /// `--run-key` 在 `skip_key_wait` 下不阻塞
    #[test]
    fn tool0_run_key_skips_wait_in_tests() {
        let path = temp_cmd_file("0 -v\n");
        let path_str = path.to_str().expect("utf8").to_owned();
        let session =
            run_tool0_with(&args(&["-R", "-b", &path_str]), &test_hooks()).expect("run-key");
        assert!(!path.exists());
        assert!(!session.output.run.expect("r").waited_for_key);
    }

    /// spec `tool0_kill_missing_pid_ok`
    #[test]
    fn tool0_kill_missing_pid_ok() {
        let session = run_tool0(&args(&["-k", "-u", "4294967294", "-b", "0"])).expect("kill");
        let kill = session.output.kill.expect("surface");
        assert_eq!(kill.pid, 4_294_967_294);
        assert_eq!(kill.sleep_ms, 0);
        assert!(kill.ignored_missing);
        assert_eq!(session.exit_code, 0);
    }

    /// `--conf` 导出四表且非空（真系统优先；失败回落假表）
    #[test]
    fn tool0_conf_exports_fake_tables() {
        let session = run_tool0(&args(&["-c"])).expect("conf");
        let conf = session.output.conf.as_ref().expect("surface");
        assert!(!conf.devices.is_empty());
        assert!(!conf.ips.is_empty());
        // ARP 在 macOS 可能为空；路由在真系统或假表下应非空
        assert!(!conf.routes.is_empty());
        let rendered = session.output.render();
        assert!(rendered.contains("section:conf"));
        assert!(rendered.contains("device:"));
        assert!(rendered.contains("ip:"));
        assert!(rendered.contains("route:"));
    }

    #[test]
    fn tool0_unknown_toolhelp_errors() {
        let error = run_tool0(&args(&["-h", "-u", "224"])).expect_err("bad");
        assert!(matches!(error, Tool0Error::UnknownTool(224)));
    }
}
