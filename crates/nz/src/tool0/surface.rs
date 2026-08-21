//! 工具 0 信息面结构与行协议渲染。

/// `--tools` 一条工具。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolListEntry {
    /// 工具号。
    pub id: u32,
    /// 标题。
    pub title: String,
    /// 建议名。
    pub suggested_name: String,
}

/// `--tools` 信息面。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolsSurface {
    /// 最大已发布工具号。
    pub max_id: u32,
    /// 已发布条数。
    pub count: u32,
    /// 排序表。
    pub tools: Vec<ToolListEntry>,
    /// 同义词：`(工具号, token)`。
    pub synonyms: Vec<(u32, String)>,
    /// 需要 stdin 的号。
    pub stdin: Vec<u32>,
    /// 需要 backspace 的号。
    pub backspace: Vec<u32>,
    /// 树根名（切片 1：`main`）。
    pub tree_root: String,
    /// 挂在树根下的工具号。
    pub tree_children: Vec<u32>,
}

/// `--version` 信息面。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VersionSurface {
    /// 主版本。
    pub major: u32,
    /// 次版本。
    pub minor: u32,
    /// 补丁版本。
    pub micro: u32,
}

/// `--error` 信息面。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorSurface {
    /// 输入码。
    pub code: u32,
    /// 可读文本。
    pub text: String,
}

/// 渲染 `section:tools`。
#[must_use]
pub fn format_tools(surface: &ToolsSurface) -> String {
    let mut lines = vec![
        String::from("section:tools"),
        format!("max_id:{}", surface.max_id),
        format!("count:{}", surface.count),
    ];
    for tool in &surface.tools {
        lines.push(format!(
            "tool:{}:{}:{}",
            tool.id, tool.title, tool.suggested_name
        ));
    }
    for (id, token) in &surface.synonyms {
        lines.push(format!("synonym:{id}:{token}"));
    }
    for id in &surface.stdin {
        lines.push(format!("stdin:{id}"));
    }
    for id in &surface.backspace {
        lines.push(format!("backspace:{id}"));
    }
    lines.push(format!("tree_node:{}", surface.tree_root));
    for id in &surface.tree_children {
        lines.push(format!("tree_child_tool:{}:{id}", surface.tree_root));
    }
    lines.join("\n")
}

/// 渲染 `section:version`。
#[must_use]
pub fn format_version(surface: &VersionSurface) -> String {
    [
        String::from("section:version"),
        format!("major:{}", surface.major),
        format!("minor:{}", surface.minor),
        format!("micro:{}", surface.micro),
    ]
    .join("\n")
}

/// 渲染 `section:error`。
#[must_use]
pub fn format_error(surface: &ErrorSurface) -> String {
    [
        String::from("section:error"),
        format!("code:{}", surface.code),
        format!("text:{}", surface.text),
    ]
    .join("\n")
}
