//! 参数描述表。

use crate::ParseError;

/// 参数类：可选、必选、或 MORE 尾部开关。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArgClass {
    /// 可选。
    Optional,
    /// 必选（Cli 模式结束时必须有用户值或默认）。
    Required,
    /// 打开 MORE 收集；本身不占短键表。
    More,
}

/// MVP 支持的值类型。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueKind {
    /// 布尔三态语法。
    Bool,
    /// 任意字符串。
    String,
    /// 无符号十进制 `u32`。
    U32,
}

/// 单条参数描述（对齐 `netwox_toolarg` 字段语义）。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArgSpec {
    /// 短选项字符；[`ArgClass::More`] 时忽略。
    pub key: char,
    /// 长名（不含 `--`）。
    pub long_name: String,
    /// 可选 / 必选 / MORE。
    pub class: ArgClass,
    /// 值类型。
    pub value_kind: ValueKind,
    /// 是否 Advanced（仅影响帮助分组元数据）。
    pub advanced: bool,
    /// 默认值字符串；布尔常用 `"0"`/`"1"`。
    pub default: Option<String>,
    /// 说明文字。
    pub help: String,
}

impl ArgSpec {
    /// 可选布尔。
    #[must_use]
    pub fn optional_bool(key: char, long_name: impl Into<String>, help: impl Into<String>) -> Self {
        Self {
            key,
            long_name: long_name.into(),
            class: ArgClass::Optional,
            value_kind: ValueKind::Bool,
            advanced: false,
            default: Some(String::from("0")),
            help: help.into(),
        }
    }

    /// 可选字符串。
    #[must_use]
    pub fn optional_string(
        key: char,
        long_name: impl Into<String>,
        help: impl Into<String>,
        default: Option<impl Into<String>>,
    ) -> Self {
        Self {
            key,
            long_name: long_name.into(),
            class: ArgClass::Optional,
            value_kind: ValueKind::String,
            advanced: false,
            default: default.map(Into::into),
            help: help.into(),
        }
    }

    /// 必选字符串。
    #[must_use]
    pub fn required_string(
        key: char,
        long_name: impl Into<String>,
        help: impl Into<String>,
    ) -> Self {
        Self {
            key,
            long_name: long_name.into(),
            class: ArgClass::Required,
            value_kind: ValueKind::String,
            advanced: false,
            default: None,
            help: help.into(),
        }
    }

    /// 可选 `u32`。
    #[must_use]
    pub fn optional_u32(
        key: char,
        long_name: impl Into<String>,
        help: impl Into<String>,
        default: Option<u32>,
    ) -> Self {
        Self {
            key,
            long_name: long_name.into(),
            class: ArgClass::Optional,
            value_kind: ValueKind::U32,
            advanced: false,
            default: default.map(|value| value.to_string()),
            help: help.into(),
        }
    }

    /// 必选 `u32`。
    #[must_use]
    pub fn required_u32(key: char, long_name: impl Into<String>, help: impl Into<String>) -> Self {
        Self {
            key,
            long_name: long_name.into(),
            class: ArgClass::Required,
            value_kind: ValueKind::U32,
            advanced: false,
            default: None,
            help: help.into(),
        }
    }

    /// MORE 描述项（打开尾部收集）。
    #[must_use]
    pub fn more(name: impl Into<String>, help: impl Into<String>) -> Self {
        Self {
            key: '_',
            long_name: name.into(),
            class: ArgClass::More,
            value_kind: ValueKind::String,
            advanced: false,
            default: None,
            help: help.into(),
        }
    }

    /// 标记为 Advanced。
    #[must_use]
    pub fn advanced(mut self) -> Self {
        self.advanced = true;
        self
    }
}

/// 校验后的参数表。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArgSchema {
    specs: Vec<ArgSpec>,
    allow_more: bool,
    more_help: Option<String>,
}

impl ArgSchema {
    /// 从描述列表构建；失败时返回 [`ParseError::InvalidSchema`]。
    ///
    /// # Errors
    ///
    /// 重复短键/长名、保留长名、非法短键 `-` 时失败。
    pub fn try_from_specs(specs: Vec<ArgSpec>) -> Result<Self, ParseError> {
        let mut allow_more = false;
        let mut more_help = None;
        let mut keys = std::collections::HashSet::new();
        let mut names = std::collections::HashSet::new();
        let mut stored = Vec::new();

        for spec in specs {
            if spec.class == ArgClass::More {
                allow_more = true;
                more_help = Some(spec.help.clone());
                continue;
            }
            if spec.key == '-' {
                return Err(ParseError::InvalidSchema(String::from(
                    "option key '-' is invalid",
                )));
            }
            if !keys.insert(spec.key) {
                return Err(ParseError::InvalidSchema(format!(
                    "duplicate short key '-{}'",
                    spec.key
                )));
            }
            let long_name = spec.long_name.clone();
            if long_name.is_empty() {
                return Err(ParseError::InvalidSchema(format!(
                    "option '-{}' has empty long name",
                    spec.key
                )));
            }
            if matches!(long_name.as_str(), "help" | "kbd" | "argfile") {
                return Err(ParseError::InvalidSchema(format!(
                    "option '-{}' has reserved long name '--{long_name}'",
                    spec.key
                )));
            }
            if !names.insert(long_name.clone()) {
                return Err(ParseError::InvalidSchema(format!(
                    "duplicate long name '--{long_name}'"
                )));
            }
            stored.push(spec);
        }

        Ok(Self {
            specs: stored,
            allow_more,
            more_help,
        })
    }

    /// 是否允许 MORE。
    #[must_use]
    pub fn allow_more(&self) -> bool {
        self.allow_more
    }

    /// MORE 帮助文案（若有）。
    #[must_use]
    pub fn more_help(&self) -> Option<&str> {
        self.more_help.as_deref()
    }

    /// 全部非 MORE 描述。
    #[must_use]
    pub fn specs(&self) -> &[ArgSpec] {
        &self.specs
    }

    /// 按短键查找。
    #[must_use]
    pub fn find_by_key(&self, key: char) -> Option<&ArgSpec> {
        self.specs.iter().find(|spec| spec.key == key)
    }

    /// 按长名精确或无冲突前缀查找。
    ///
    /// # Errors
    ///
    /// 未知或歧义前缀。
    pub fn resolve_long_name(&self, name: &str) -> Result<&ArgSpec, ParseError> {
        let mut exact: Option<&ArgSpec> = None;
        let mut partial: Option<&ArgSpec> = None;
        for spec in &self.specs {
            if spec.long_name == name {
                exact = Some(spec);
                break;
            }
            if spec.long_name.starts_with(name) {
                if let Some(previous) = partial {
                    return Err(ParseError::AmbiguousPrefix {
                        prefix: name.to_owned(),
                        first: previous.long_name.clone(),
                        second: spec.long_name.clone(),
                    });
                }
                partial = Some(spec);
            }
        }
        if let Some(spec) = exact.or(partial) {
            return Ok(spec);
        }
        Err(ParseError::UnknownOption(format!("--{name}")))
    }
}
