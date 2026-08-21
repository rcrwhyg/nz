//! CLI 工具登记与双模式分发。
//!
//! 对照 `spec/netwox/registry.md`：静态表 + 数字/建议名等价；不含工具体实现。

mod entries;

use std::collections::HashMap;
use std::sync::OnceLock;

pub use entries::{DEFERRED_TOOL_IDS, TOOL_ENTRIES};

/// 工具号（0–223）。
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ToolId(pub u32);

/// 发布分类。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublishKind {
    /// 默认编进二进制，出现在 `nz 0 --tools`。
    Release,
    /// 后置审计；默认不编进表。
    Deferred,
    /// 可调用但不进 Search / `--tools`（工具 0）；或根本不提供 CLI（218）。
    Hidden,
}

/// 一条工具登记。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolEntry {
    /// 工具号。
    pub id: ToolId,
    /// 原 toollist 标题。
    pub title: &'static str,
    /// 具名子命令。
    pub suggested_name: &'static str,
    /// 发布规则。
    pub publish: PublishKind,
    /// 是否需要 stdin（工具 0 `--tools` 标记）。
    pub needs_stdin: bool,
    /// 是否按退格擦除输出。
    pub needs_backspace: bool,
}

/// 分发解析结果。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DispatchRequest {
    /// 打印总目录。
    Catalog,
    /// 调用已登记工具。
    Run {
        /// 工具登记。
        entry: ToolEntry,
        /// 跳过程序名与工具选择后的参数。
        tool_arguments: Vec<String>,
    },
}

/// 分发错误。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DispatchError {
    /// 未登记的号或名。
    UnknownTool(String),
}

impl std::fmt::Display for DispatchError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownTool(token) => write!(formatter, "unknown tool: {token}"),
        }
    }
}

impl std::error::Error for DispatchError {}

fn by_id() -> &'static HashMap<u32, ToolEntry> {
    static MAP: OnceLock<HashMap<u32, ToolEntry>> = OnceLock::new();
    MAP.get_or_init(|| {
        TOOL_ENTRIES
            .iter()
            .map(|entry| (entry.id.0, *entry))
            .collect()
    })
}

fn by_name() -> &'static HashMap<&'static str, ToolEntry> {
    static MAP: OnceLock<HashMap<&'static str, ToolEntry>> = OnceLock::new();
    MAP.get_or_init(|| {
        TOOL_ENTRIES
            .iter()
            .map(|entry| (entry.suggested_name, *entry))
            .collect()
    })
}

/// 按工具号查找（含 hidden；不含 deferred）。
#[must_use]
pub fn lookup_by_id(id: u32) -> Option<ToolEntry> {
    by_id().get(&id).copied()
}

/// 按建议名查找（大小写敏感，与表一致）。
#[must_use]
pub fn lookup_by_name(name: &str) -> Option<ToolEntry> {
    by_name().get(name).copied()
}

/// 出现在 `nz 0 --tools` 的条目：仅 `Release`。
pub fn tools_for_search() -> impl Iterator<Item = ToolEntry> {
    TOOL_ENTRIES
        .iter()
        .copied()
        .filter(|entry| entry.publish == PublishKind::Release)
}

/// 需要 stdin 的工具号（排序）。
#[must_use]
pub fn stdin_tool_ids() -> Vec<u32> {
    let mut ids: Vec<u32> = TOOL_ENTRIES
        .iter()
        .filter(|entry| entry.needs_stdin)
        .map(|entry| entry.id.0)
        .collect();
    ids.sort_unstable();
    ids
}

/// 需要 backspace 的工具号（排序）。
#[must_use]
pub fn backspace_tool_ids() -> Vec<u32> {
    let mut ids: Vec<u32> = TOOL_ENTRIES
        .iter()
        .filter(|entry| entry.needs_backspace)
        .map(|entry| entry.id.0)
        .collect();
    ids.sort_unstable();
    ids
}

/// 解析 `argv`（含程序名）为分发请求。
///
/// 规则：无参数或首个业务参数为 `--help` → 目录；否则数字优先，否则建议名。
///
/// # Errors
///
/// 未知工具号/名时返回 [`DispatchError::UnknownTool`]。
pub fn dispatch(argv: &[String]) -> Result<DispatchRequest, DispatchError> {
    let rest = argv.get(1..).unwrap_or(&[]);
    if rest.is_empty() || rest.first().is_some_and(|arg| arg == "--help") {
        return Ok(DispatchRequest::Catalog);
    }
    let selector = &rest[0];
    let entry = if let Ok(id) = selector.parse::<u32>() {
        lookup_by_id(id).ok_or_else(|| DispatchError::UnknownTool(selector.clone()))?
    } else {
        lookup_by_name(selector).ok_or_else(|| DispatchError::UnknownTool(selector.clone()))?
    };
    // 工具 218：hidden 且不提供 CLI 入口
    if entry.id.0 == 218 {
        return Err(DispatchError::UnknownTool(selector.clone()));
    }
    Ok(DispatchRequest::Run {
        entry,
        tool_arguments: rest[1..].to_vec(),
    })
}

/// 格式化总目录文本（号 + 标题 + 建议名）。
#[must_use]
pub fn format_catalog() -> String {
    let mut lines = Vec::new();
    lines.push(String::from("nz tools:"));
    for entry in tools_for_search() {
        lines.push(format!(
            "  {:>3}  {}  ({})",
            entry.id.0, entry.title, entry.suggested_name
        ));
    }
    lines.join("\n")
}

/// 工具入口桩：本闸不实现业务，仅供分发测试。
#[must_use]
pub fn invoke_stub(entry: ToolEntry, _tool_arguments: &[String]) -> i32 {
    if entry.id.0 == 0 {
        // 工具 0 本体下一任务实现；此处返回成功以便分发联通。
        return 0;
    }
    eprintln!(
        "nz: tool {} ({}) is registered but not implemented yet",
        entry.id.0, entry.suggested_name
    );
    2
}

#[cfg(test)]
mod tests {
    use super::{
        DEFERRED_TOOL_IDS, DispatchRequest, PublishKind, TOOL_ENTRIES, backspace_tool_ids,
        dispatch, format_catalog, lookup_by_id, lookup_by_name, stdin_tool_ids, tools_for_search,
    };

    fn argv(args: &[&str]) -> Vec<String> {
        std::iter::once("nz")
            .chain(args.iter().copied())
            .map(str::to_owned)
            .collect()
    }

    /// spec `dispatch_numeric_eq_named`
    #[test]
    fn dispatch_numeric_eq_named() {
        let by_id = dispatch(&argv(&["0"])).expect("id");
        let by_name = dispatch(&argv(&["gui-info"])).expect("name");
        match (by_id, by_name) {
            (
                DispatchRequest::Run {
                    entry: left,
                    tool_arguments: left_args,
                },
                DispatchRequest::Run {
                    entry: right,
                    tool_arguments: right_args,
                },
            ) => {
                assert_eq!(left.id, right.id);
                assert_eq!(left.suggested_name, "gui-info");
                assert_eq!(left_args, right_args);
            }
            _ => panic!("expected run"),
        }
    }

    /// spec `dispatch_unknown_id_fails`
    #[test]
    fn dispatch_unknown_id_fails() {
        assert!(dispatch(&argv(&["224"])).is_err());
        assert!(dispatch(&argv(&["no-such-tool"])).is_err());
        assert!(dispatch(&argv(&["218"])).is_err());
    }

    /// spec `registry_stdin_backspace_lists`
    #[test]
    fn registry_stdin_backspace_lists() {
        assert_eq!(stdin_tool_ids(), vec![7, 14, 87, 88, 89, 90, 99, 152, 171]);
        assert_eq!(backspace_tool_ids(), vec![138, 139, 210]);
    }

    /// spec `registry_hides_zero_and_218`
    #[test]
    fn registry_hides_zero_and_218() {
        let catalog = format_catalog();
        assert!(!catalog.contains("gui-info"));
        assert!(!tools_for_search().any(|entry| entry.id.0 == 0 || entry.id.0 == 218));
        assert_eq!(
            lookup_by_id(0).map(|entry| entry.publish),
            Some(PublishKind::Hidden)
        );
        assert_eq!(
            lookup_by_id(218).map(|entry| entry.publish),
            Some(PublishKind::Hidden)
        );
    }

    /// spec `registry_omits_deferred_by_default`
    #[test]
    fn registry_omits_deferred_by_default() {
        for id in DEFERRED_TOOL_IDS {
            assert!(lookup_by_id(*id).is_none(), "deferred {id} must be absent");
        }
        assert!(
            !TOOL_ENTRIES
                .iter()
                .any(|entry| { matches!(entry.publish, PublishKind::Deferred) })
        );
        assert!(lookup_by_name("gui-info").is_some());
    }

    #[test]
    fn catalog_request_on_empty_or_help() {
        assert_eq!(
            dispatch(&argv(&[])).expect("empty"),
            DispatchRequest::Catalog
        );
        assert_eq!(
            dispatch(&argv(&["--help"])).expect("help"),
            DispatchRequest::Catalog
        );
    }
}
