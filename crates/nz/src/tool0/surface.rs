//! 工具 0 信息面结构与行协议渲染。

use nz_arg::{ArgClass, ArgSchema, ArgSpec, ParsedArgs, ValueKind};

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
    /// 分类树（根为 `main`；已剪去无已发布工具的空枝）。
    pub tree: Vec<crate::registry::SearchTreeNode>,
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

/// form 里一条参数描述。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormField {
    /// 短键。
    pub key: char,
    /// 长名。
    pub long_name: String,
    /// 类型标签。
    pub value_kind: String,
    /// 默认（若有）。
    pub default: Option<String>,
    /// 是否 Advanced。
    pub advanced: bool,
    /// 是否必填。
    pub required: bool,
    /// 说明。
    pub help: String,
}

/// `--toolhelp` 信息面。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolHelpSurface {
    /// 工具号。
    pub tool_id: u32,
    /// 标题。
    pub title: String,
    /// 帮助正文。
    pub help: String,
    /// 示例。
    pub example: String,
    /// Usage。
    pub usage: String,
    /// 是否挂了 `ArgSchema`。
    pub has_schema: bool,
    /// 普通参数。
    pub form: Vec<FormField>,
    /// Advanced 参数。
    pub form_advanced: Vec<FormField>,
}

/// `--formupdate` 回填一项。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormUpdateItem {
    /// 短键；多余位置参数用 `-`。
    pub key: char,
    /// 字符串化的值。
    pub value: String,
}

/// `--formupdate` 信息面。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormUpdateSurface {
    /// 目标工具号。
    pub tool_id: u32,
    /// 用户显式项。
    pub items: Vec<FormUpdateItem>,
}

/// 从 schema 拆分 form 字段。
#[must_use]
pub fn form_fields_from_schema(schema: &ArgSchema) -> (Vec<FormField>, Vec<FormField>) {
    let mut normal = Vec::new();
    let mut advanced = Vec::new();
    for spec in schema.specs() {
        let field = form_field_from_spec(spec);
        if field.advanced {
            advanced.push(field);
        } else {
            normal.push(field);
        }
    }
    (normal, advanced)
}

fn form_field_from_spec(spec: &ArgSpec) -> FormField {
    FormField {
        key: spec.key,
        long_name: spec.long_name.clone(),
        value_kind: match spec.value_kind {
            ValueKind::Bool => String::from("bool"),
            ValueKind::String => String::from("string"),
            ValueKind::U32 => String::from("u32"),
        },
        default: spec.default.clone(),
        advanced: spec.advanced,
        required: spec.class == ArgClass::Required,
        help: spec.help.clone(),
    }
}

/// 从解析结果收集显式设置项（含 MORE → 键 `-`）。
#[must_use]
pub fn form_update_items(schema: &ArgSchema, parsed: &ParsedArgs) -> Vec<FormUpdateItem> {
    let mut items = Vec::new();
    for spec in schema.specs() {
        if !parsed.isset(spec.key) {
            continue;
        }
        let value = match spec.value_kind {
            ValueKind::Bool => {
                if parsed.get_bool(spec.key) == Some(true) {
                    String::from("1")
                } else {
                    String::from("0")
                }
            }
            ValueKind::String => parsed.get_string(spec.key).unwrap_or("").to_owned(),
            ValueKind::U32 => parsed
                .get_u32(spec.key)
                .map_or_else(String::new, |n| n.to_string()),
        };
        items.push(FormUpdateItem {
            key: spec.key,
            value,
        });
    }
    if !parsed.more().is_empty() {
        items.push(FormUpdateItem {
            key: '-',
            value: parsed.more().join(" "),
        });
    }
    items
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
    lines.push(format!(
        "tree_root:{}",
        surface.tree.first().map_or("main", |node| node.id.as_str())
    ));
    for node in &surface.tree {
        lines.push(format!("tree_node:{}:{}", node.id, node.label));
        for child in &node.child_categories {
            lines.push(format!("tree_child_cat:{}:{child}", node.id));
        }
        for tool_id in &node.child_tools {
            lines.push(format!("tree_child_tool:{}:{tool_id}", node.id));
        }
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

/// 渲染 `section:toolhelp`。
#[must_use]
pub fn format_toolhelp(surface: &ToolHelpSurface) -> String {
    let mut lines = vec![
        String::from("section:toolhelp"),
        format!("tool_id:{}", surface.tool_id),
        format!("title:{}", surface.title),
        format!("help:{}", surface.help),
        format!("example:{}", surface.example),
        format!("usage:{}", surface.usage),
        format!("has_schema:{}", u8::from(surface.has_schema)),
    ];
    for field in &surface.form {
        lines.push(format_form_field("form", field));
    }
    for field in &surface.form_advanced {
        lines.push(format_form_field("form_advanced", field));
    }
    lines.join("\n")
}

fn format_form_field(prefix: &str, field: &FormField) -> String {
    format!(
        "{prefix}:{}:{}:{}:{}:{}:{}:{}",
        field.key,
        field.long_name,
        field.value_kind,
        field.default.as_deref().unwrap_or(""),
        u8::from(field.advanced),
        u8::from(field.required),
        field.help
    )
}

/// 渲染 `section:formupdate`。
#[must_use]
pub fn format_formupdate(surface: &FormUpdateSurface) -> String {
    let mut lines = vec![
        String::from("section:formupdate"),
        format!("tool_id:{}", surface.tool_id),
    ];
    for item in &surface.items {
        lines.push(format!("set:{}:{}", item.key, item.value));
    }
    lines.join("\n")
}

/// `--run` / `--run-key` 结果。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunSurface {
    /// 被调工具退出码。
    pub exit_code: i32,
    /// 是否为 run-key。
    pub waited_for_key: bool,
    /// 若子工具为工具 0，为其渲染文本；否则可空。
    pub child_output: String,
}

/// `--kill` 结果。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KillSurface {
    /// 目标 PID。
    pub pid: u32,
    /// 睡眠毫秒。
    pub sleep_ms: u32,
    /// 始终视为成功（目标已死也忽略）。
    pub ignored_missing: bool,
}

/// `--conf` 信息面（假或真配置的四表摘要）。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfSurface {
    /// `device:<num>:<easy>:<real>:<mtu>`
    pub devices: Vec<String>,
    /// `ip:<dev>:<addr>:<mask>`
    pub ips: Vec<String>,
    /// `arp:<dev>:<eth>:<ip>`
    pub arps: Vec<String>,
    /// `route:<dev>:<dst>:<mask>:<src>:<gw>:<metric>`
    pub routes: Vec<String>,
}

/// 渲染 `section:run`。
#[must_use]
pub fn format_run(surface: &RunSurface) -> String {
    let mut text = format!(
        "section:run\nexit_code:{}\nwaited_for_key:{}",
        surface.exit_code,
        u8::from(surface.waited_for_key)
    );
    if !surface.child_output.is_empty() {
        text.push('\n');
        text.push_str(&surface.child_output);
    }
    text
}

/// 渲染 `section:kill`。
#[must_use]
pub fn format_kill(surface: &KillSurface) -> String {
    [
        String::from("section:kill"),
        format!("pid:{}", surface.pid),
        format!("sleep_ms:{}", surface.sleep_ms),
        format!("ignored_missing:{}", u8::from(surface.ignored_missing)),
    ]
    .join("\n")
}

/// 渲染 `section:conf`。
#[must_use]
pub fn format_conf(surface: &ConfSurface) -> String {
    let mut lines = vec![String::from("section:conf")];
    for line in &surface.devices {
        lines.push(format!("device:{line}"));
    }
    for line in &surface.ips {
        lines.push(format!("ip:{line}"));
    }
    for line in &surface.arps {
        lines.push(format!("arp:{line}"));
    }
    for line in &surface.routes {
        lines.push(format!("route:{line}"));
    }
    lines.join("\n")
}
