//! CLI 工具登记与双模式分发。
//!
//! 对照 `spec/netwox/registry.md`：静态表 + 数字/建议名等价；不含工具体实现。

mod entries;
mod tooltree_data;

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

pub use entries::{DEFERRED_TOOL_IDS, TOOL_ENTRIES};
pub use tooltree_data::{TOOL_TREE_PLACEMENTS, TREE_CATEGORIES, TreeCategory};

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

/// Search 树中的一个分类节点（已按发布工具剪枝）。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchTreeNode {
    /// 稳定 id（如 `info-local`）。
    pub id: String,
    /// 显示标签。
    pub label: String,
    /// 子分类 id。
    pub child_categories: Vec<String>,
    /// 直接挂接的已发布工具号（排序）。
    pub child_tools: Vec<u32>,
}

/// 判断分类子树是否含已发布工具（直接或间接）。
fn subtree_relevant(
    category_id: &str,
    categories: &HashMap<&str, &TreeCategory>,
    tools_by_category: &HashMap<&str, Vec<u32>>,
    memo: &mut HashMap<String, bool>,
) -> bool {
    if let Some(cached) = memo.get(category_id) {
        return *cached;
    }
    let has_direct = tools_by_category
        .get(category_id)
        .is_some_and(|tools| !tools.is_empty());
    let Some(category) = categories.get(category_id) else {
        memo.insert(category_id.to_owned(), false);
        return false;
    };
    let has_child = category
        .child_categories
        .iter()
        .any(|child| subtree_relevant(child, categories, tools_by_category, memo));
    let relevant = has_direct || has_child;
    memo.insert(category_id.to_owned(), relevant);
    relevant
}

/// 构建供 `--tools` 使用的分类树（根为 `main`；后置/未登记工具不入树）。
#[must_use]
pub fn build_search_tree() -> Vec<SearchTreeNode> {
    let release_ids: HashSet<u32> = tools_for_search().map(|entry| entry.id.0).collect();
    let categories: HashMap<&str, &TreeCategory> = TREE_CATEGORIES
        .iter()
        .map(|category| (category.id, category))
        .collect();

    let mut tools_by_category: HashMap<&str, Vec<u32>> = HashMap::new();
    for &(tool_id, category_ids) in TOOL_TREE_PLACEMENTS {
        if !release_ids.contains(&tool_id) {
            continue;
        }
        for &category_id in category_ids {
            tools_by_category
                .entry(category_id)
                .or_default()
                .push(tool_id);
        }
    }
    for tool_ids in tools_by_category.values_mut() {
        tool_ids.sort_unstable();
        tool_ids.dedup();
    }

    let mut relevant_memo = HashMap::new();
    let mut kept: Vec<SearchTreeNode> = Vec::new();
    let mut stack = vec![String::from("main")];
    let mut seen = HashSet::new();
    while let Some(category_id) = stack.pop() {
        if !seen.insert(category_id.clone()) {
            continue;
        }
        if !subtree_relevant(
            category_id.as_str(),
            &categories,
            &tools_by_category,
            &mut relevant_memo,
        ) {
            continue;
        }
        let Some(category) = categories.get(category_id.as_str()) else {
            continue;
        };
        let child_categories: Vec<String> = category
            .child_categories
            .iter()
            .copied()
            .filter(|child| {
                subtree_relevant(child, &categories, &tools_by_category, &mut relevant_memo)
            })
            .map(str::to_owned)
            .collect();
        stack.extend(child_categories.iter().cloned());
        let child_tools = tools_by_category
            .get(category_id.as_str())
            .cloned()
            .unwrap_or_default();
        kept.push(SearchTreeNode {
            id: category_id,
            label: category.label.to_owned(),
            child_categories,
            child_tools,
        });
    }
    kept.sort_by(|left, right| left.id.cmp(&right.id));
    if let Some(index) = kept.iter().position(|node| node.id == "main") {
        kept.swap(0, index);
    }
    kept
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

/// 工具入口：已实现工具走业务；其余打印未实现。
#[must_use]
pub fn invoke_tool(entry: ToolEntry, tool_arguments: &[String]) -> i32 {
    match entry.id.0 {
        0 => crate::tool0::invoke_tool0(tool_arguments),
        1 => match crate::tools::run_net_conf(tool_arguments) {
            Ok(text) => {
                if !text.is_empty() {
                    println!("{text}");
                }
                0
            }
            Err(error) => {
                eprintln!("nz: {error}");
                1
            }
        },
        2 => match crate::tools::run_debug_info(tool_arguments) {
            Ok(text) => {
                if !text.is_empty() {
                    println!("{text}");
                }
                0
            }
            Err(error) => {
                eprintln!("nz: {error}");
                1
            }
        },
        3 => match crate::tools::run_host_info(tool_arguments) {
            Ok(text) => {
                if !text.is_empty() {
                    println!("{text}");
                }
                0
            }
            Err(error) => {
                eprintln!("nz: {error}");
                1
            }
        },
        _ => {
            eprintln!(
                "nz: tool {} ({}) is registered but not implemented yet",
                entry.id.0, entry.suggested_name
            );
            2
        }
    }
}

/// 兼容旧名：同 [`invoke_tool`]。
#[must_use]
pub fn invoke_stub(entry: ToolEntry, tool_arguments: &[String]) -> i32 {
    invoke_tool(entry, tool_arguments)
}

#[cfg(test)]
mod tests {
    use super::{
        DEFERRED_TOOL_IDS, DispatchRequest, PublishKind, TOOL_ENTRIES, backspace_tool_ids,
        build_search_tree, dispatch, format_catalog, lookup_by_id, lookup_by_name, stdin_tool_ids,
        tools_for_search,
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

    /// Search 树含分类与交叉挂载（非 main 平铺）
    #[test]
    fn registry_search_tree_has_categories() {
        let tree = build_search_tree();
        assert_eq!(tree.first().map(|node| node.id.as_str()), Some("main"));
        assert!(
            tree.iter()
                .any(|node| node.id == "info-local" && node.child_tools.contains(&1))
        );
        assert!(
            tree.iter()
                .any(|node| node.id == "ping" && node.child_tools.contains(&49))
        );
        assert!(
            tree.iter()
                .any(|node| node.id == "main" && node.child_categories.iter().any(|c| c == "info"))
        );
        for id in DEFERRED_TOOL_IDS {
            assert!(
                !tree.iter().any(|node| node.child_tools.contains(id)),
                "deferred {id} must not appear in search tree"
            );
        }
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

    /// spec `bool_triple_parses`
    #[test]
    fn bool_triple_parses() {
        use crate::tool_schemas::tool0_schema;
        use nz_arg::{ParseMode, ParseOutcome, parse};

        let schema = tool0_schema();
        let on = match parse(&schema, &["-t"], ParseMode::Cli).expect("on") {
            ParseOutcome::Parsed(values) => values,
            ParseOutcome::Help { .. } => panic!("help"),
        };
        assert_eq!(on.get_bool('t'), Some(true));
        assert!(on.isset('t'));

        let off = match parse(&schema, &["+t"], ParseMode::Cli).expect("off") {
            ParseOutcome::Parsed(values) => values,
            ParseOutcome::Help { .. } => panic!("help"),
        };
        assert_eq!(off.get_bool('t'), Some(false));
        assert!(off.isset('t'));

        let no = match parse(&schema, &["--no-tools"], ParseMode::Cli).expect("no") {
            ParseOutcome::Parsed(values) => values,
            ParseOutcome::Help { .. } => panic!("help"),
        };
        assert_eq!(no.get_bool('t'), Some(false));
        assert!(no.isset('t'));
    }

    /// spec `help_and_help2_flags_exist`
    #[test]
    fn help_and_help2_flags_exist() {
        use crate::tool_schemas::tool0_schema;
        use nz_arg::{ParseMode, ParseOutcome, parse};

        let schema = tool0_schema();
        assert_eq!(
            parse(&schema, &["--help"], ParseMode::Cli).expect("help"),
            ParseOutcome::Help {
                include_advanced: false
            }
        );
        assert_eq!(
            parse(&schema, &["--help2"], ParseMode::Cli).expect("help2"),
            ParseOutcome::Help {
                include_advanced: true
            }
        );
    }
}
