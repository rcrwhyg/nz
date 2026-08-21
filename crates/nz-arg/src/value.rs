//! 解析结果存储。

use crate::ParseError;
use crate::schema::{ArgSchema, ValueKind};
use std::collections::HashMap;

/// 已解析的参数值。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArgValue {
    /// 布尔。
    Bool(bool),
    /// 字符串。
    String(String),
    /// `u32`。
    U32(u32),
}

/// 单槽状态。
#[derive(Clone, Debug, Eq, PartialEq)]
struct Slot {
    set_by_user: bool,
    value: ArgValue,
}

/// 一次成功解析的结果。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedArgs {
    slots: HashMap<char, Slot>,
    more: Vec<String>,
}

impl ParsedArgs {
    pub(crate) fn new_from_schema(schema: &ArgSchema) -> Result<Self, ParseError> {
        let mut slots = HashMap::new();
        for spec in schema.specs() {
            let value = match (&spec.default, spec.value_kind) {
                (Some(raw), _) => parse_raw(
                    spec.value_kind,
                    raw,
                    &display_option(spec.key, &spec.long_name),
                )?,
                (None, ValueKind::Bool) => ArgValue::Bool(false),
                (None, ValueKind::String) => ArgValue::String(String::new()),
                (None, ValueKind::U32) => ArgValue::U32(0),
            };
            slots.insert(
                spec.key,
                Slot {
                    set_by_user: false,
                    value,
                },
            );
        }
        Ok(Self {
            slots,
            more: Vec::new(),
        })
    }

    pub(crate) fn set_raw(
        &mut self,
        schema: &ArgSchema,
        key: char,
        raw: &str,
    ) -> Result<(), ParseError> {
        let spec = schema
            .find_by_key(key)
            .ok_or_else(|| ParseError::UnknownOption(format!("-{key}")))?;
        let value = parse_raw(
            spec.value_kind,
            raw,
            &display_option(spec.key, &spec.long_name),
        )?;
        self.slots.insert(
            key,
            Slot {
                set_by_user: true,
                value,
            },
        );
        Ok(())
    }

    pub(crate) fn set_bool(&mut self, key: char, on: bool) -> Result<(), ParseError> {
        if !self.slots.contains_key(&key) {
            return Err(ParseError::UnknownOption(format!("-{key}")));
        }
        self.slots.insert(
            key,
            Slot {
                set_by_user: true,
                value: ArgValue::Bool(on),
            },
        );
        Ok(())
    }

    pub(crate) fn push_more(&mut self, token: String) {
        self.more.push(token);
    }

    pub(crate) fn first_unset_required_key(&self, schema: &ArgSchema) -> Option<char> {
        for spec in schema.specs() {
            if spec.class == crate::schema::ArgClass::Required
                && let Some(slot) = self.slots.get(&spec.key)
                && !slot.set_by_user
            {
                return Some(spec.key);
            }
        }
        None
    }

    pub(crate) fn check_required(&self, schema: &ArgSchema) -> Result<(), ParseError> {
        for spec in schema.specs() {
            if spec.class != crate::schema::ArgClass::Required {
                continue;
            }
            let slot = self
                .slots
                .get(&spec.key)
                .expect("required key always present");
            if !slot.set_by_user && spec.default.is_none() {
                return Err(ParseError::MissingRequired {
                    key: spec.key,
                    long_name: spec.long_name.clone(),
                });
            }
        }
        Ok(())
    }

    /// 用户是否显式设置过该短键。
    #[must_use]
    pub fn isset(&self, key: char) -> bool {
        self.slots.get(&key).is_some_and(|slot| slot.set_by_user)
    }

    /// 读取布尔；键不存在或类型不符时返回 `None`。
    #[must_use]
    pub fn get_bool(&self, key: char) -> Option<bool> {
        match self.slots.get(&key).map(|slot| &slot.value) {
            Some(ArgValue::Bool(value)) => Some(*value),
            _ => None,
        }
    }

    /// 读取字符串。
    #[must_use]
    pub fn get_string(&self, key: char) -> Option<&str> {
        match self.slots.get(&key).map(|slot| &slot.value) {
            Some(ArgValue::String(value)) => Some(value.as_str()),
            _ => None,
        }
    }

    /// 读取 `u32`。
    #[must_use]
    pub fn get_u32(&self, key: char) -> Option<u32> {
        match self.slots.get(&key).map(|slot| &slot.value) {
            Some(ArgValue::U32(value)) => Some(*value),
            _ => None,
        }
    }

    /// MORE 尾部参数。
    #[must_use]
    pub fn more(&self) -> &[String] {
        &self.more
    }
}

fn display_option(key: char, long_name: &str) -> String {
    format!("-{key}|--{long_name}")
}

fn parse_raw(kind: ValueKind, raw: &str, option: &str) -> Result<ArgValue, ParseError> {
    match kind {
        ValueKind::Bool => {
            let value = match raw {
                "1" | "true" | "yes" | "on" => true,
                "0" | "false" | "no" | "off" => false,
                _ => {
                    return Err(ParseError::InvalidValue {
                        option: option.to_owned(),
                        value: raw.to_owned(),
                    });
                }
            };
            Ok(ArgValue::Bool(value))
        }
        ValueKind::String => Ok(ArgValue::String(raw.to_owned())),
        ValueKind::U32 => {
            let value = raw.parse::<u32>().map_err(|_| ParseError::InvalidValue {
                option: option.to_owned(),
                value: raw.to_owned(),
            })?;
            Ok(ArgValue::U32(value))
        }
    }
}
